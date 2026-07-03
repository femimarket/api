# Femi Rust FFI

A cross-platform Rust library providing FFI bindings for the Femi AI API and local Project Service (XMP metadata management). It targets **iOS/macOS** (via Swift Package Manager), **Android** (via JNI), and **Web** (via WebAssembly).

## Overview

This crate serves as the backend engine for the Femi application, handling two primary responsibilities:

1.  **AI API Client:** Connects to `https://femi.market/api` to generate images, videos, and transcribe audio using models like Flux2, ZImageTurbo, and Qwen3. It handles authentication, cancellation, and fallback assets (topup/fallback images/videos) internally.
2.  **Project Service (XMP):** Manages local file storage and metadata. It embeds XMP metadata (prompts, models, subjects, ratings) into image/video files using platform-specific libraries (`xmp_toolkit` on Apple, `xmpkit` on Android/Wasm).

## Architecture

The project is structured into three main layers:

*   **Rust Core (`Rust/`):** The source of truth. Contains the business logic, FFI definitions, and platform-specific implementations.
*   **Platform Bindings:**
    *   **Apple:** `Sources/Api/` (Swift) wraps the `RustFFI.xcframework`.
    *   **Android:** `Kmp/api/src/androidMain/` consumes the compiled `.so` libraries.
    *   **Web:** `Kmp/api/src/webMain/` consumes the inlined WebAssembly module.
*   **Build System:** `build-rust.sh` orchestrates cross-compilation for all targets.

### Key Directories

*   `Rust/src/api/`: Implementations for AI endpoints (e.g., `flux2_pro.rs`, `z_image_turbo.rs`).
*   `Rust/src/project_service/`: XMP metadata handling.
    *   `shared/`: Common XMP logic (`xmpkit_body`).
    *   `apple.rs`: Uses `xmp_toolkit` for macOS/iOS.
    *   `android.rs`: Uses JNI and `xmpkit` for Android.
    *   `wasm.rs`: Uses OPFS (Origin Private File System) for Web.
*   `Rust/src/id3/`: ID3 tag parsing (SYLT lyrics extraction).
*   `Sources/Api/`: Swift extensions exposing the Rust FFI to Swift code.

## Installation & Build

### Prerequisites

*   **Rust:** `rustup` installed.
*   **Android NDK:** Required for Android targets. Ensure `ANDROID_NDK_HOME` or equivalent is set.
*   **Xcode:** Required for Apple targets (for `xcodebuild`).
*   **Node.js:** Required for WebAssembly inlining step.
*   **wasm-bindgen:** Installed automatically by the build script if missing.

### Build Script

Run the main build script from the repository root:

```bash
./build-rust.sh
```

This script performs the following:

1.  **Installs Targets:** Adds `aarch64-apple-ios`, `aarch64-apple-darwin`, `wasm32-unknown-unknown`, and Android targets via `rustup`.
2.  **Apple Build:** Compiles static libraries for iOS (device/simulator) and macOS, then bundles them into `RustFFI.xcframework`.
3.  **Android Build:** Compiles `.so` libraries for `arm64-v8a`, `armeabi-v7a`, `x86_64`, and `x86`, placing them in `Kmp/api/src/androidMain/jniLibs/`.
4.  **WebAssembly Build:** Compiles to `.wasm`, runs `wasm-bindgen`, and **inlines the `.wasm` binary into the generated JavaScript glue code** as base64. This creates a self-contained `pkg/rust_ffi.js` that requires no external `.wasm` file.
5.  **Kotlin Bundle:** Generates `Kmp/api/src/webMain/kotlin/market/femi/api/RustFfiBundle.kt`, embedding the JS module as a base64 string for distribution via KMP.

### Manual Build Steps

If you need to rebuild only specific parts:

#### Apple
```bash
cd Rust
for target in aarch64-apple-ios aarch64-apple-ios-sim aarch64-apple-darwin; do
    cargo build --release --target $target
done
xcodebuild -create-xcframework \
  -library target/aarch64-apple-ios/release/librust_ffi.a -headers include \
  -library target/aarch64-apple-ios-sim/release/librust_ffi.a -headers include \
  -library target/aarch64-apple-darwin/release/librust_ffi.a -headers include \
  -output ../RustFFI.xcframework
```

#### Android
```bash
cd Rust
# Assuming NDK is configured in .cargo/config.toml
for triple in aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android; do
    cargo build --release --target $triple
    # Copy .so to appropriate jniLibs folder
done
```

#### WebAssembly
```bash
cd Rust
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/rust_ffi.wasm --out-dir pkg --target web
# Run the inline-wasm node script provided in build-rust.sh
```

## API Reference

### AI Endpoints

All AI endpoints follow a similar pattern: they accept credentials, input data (prompts, images, audio), and an optional cancellation flag. They return binary data (image/video bytes or JSON strings).

#### Common Parameters

*   `user` / `password`: Credentials for the Femi API.
*   `cancel_flag`: A pointer to a `u8`. If non-null and set to `1`, the operation cancels and returns a fallback asset.
*   Return Value: A pointer to `u8` and a length pointer. The caller is responsible for freeing the memory (via `free()` or equivalent).

#### Image Generation

*   **ZImageTurbo:** `rust_ffi_z_image_turbo(user, password, prompt, cancel_flag, out_len)`
*   **Flux2 Pro:** `rust_ffi_flux2_pro(user, password, prompt, cancel_flag, out_len)`
*   **Nano Banana2:** `rust_ffi_nano_banana2(user, password, prompt, cancel_flag, out_len)`
*   **Flux2 Dev I2I:** `rust_ffi_flux2_dev_i2i(user, password, image_b64, prompt, cancel_flag, out_len)`
*   **Flux2 Klein I2I:** `rust_ffi_flux2_klein_i2i(user, password, image_b64, image2_b64, prompt, cancel_flag, out_len)`

#### Video Generation

*   **LTX-2 3A2V:** `rust_ffi_ltx2_3a2v(user, password, image_b64, audio_b64, prompt, cancel_flag, out_len)`

#### Audio/Text

*   **Qwen3 ASR Flash:** `rust_ffi_qwen3_asr_flash(user, password, audio_b64, cancel_flag, out_len)`
    *   Returns lyrics text.
*   **Qwen3 6.35B A3B:** `rust_ffi_qwen3_6_35b_a3b(user, password, messages_json, cancel_flag, out_len)`
    *   `messages_json`: JSON array of chat turns.
    *   Returns updated JSON array with the assistant's reply appended.

### Project Service (XMP)

These functions manage local files and metadata. Rust owns the `Documents/` directory. All functions take a **filename** (not a full path).

*   **Initialization:**
    *   `psxmp_save_file(name, bytes, len, prompt, model, subject, subject_count)`: Saves a file with optional XMP metadata.
    *   `psxmp_save_audio(name, bytes, len)`: Saves an audio file, deleting any existing audio files.
*   **Reading Metadata:**
    *   `psxmp_get_prompt(file, buf, buf_len)`: Returns the prompt string.
    *   `psxmp_get_model(file, buf, buf_len)`: Returns the model name.
    *   `psxmp_get_subject(file, buf, buf_len)`: Returns a JSON array of subjects.
    *   `psxmp_get_like(file)`: Returns `1` if liked, `0` otherwise.
    *   `psxmp_get_url(file, buf, buf_len)`: Returns the full file path.
    *   `psxmp_get_all_generations(buf, buf_len)`: Returns a JSON array of all filenames.
    *   `psxmp_get_audio(buf, buf_len)`: Returns the name of the current audio file.
*   **Writing Metadata:**
    *   `psxmp_like(file, liked)`: Sets the like status (1 or 0).
    *   `psxmp_set_character_cast(a, b)`: Sets in-memory character cast pair.
    *   `psxmp_get_character_cast(buf, buf_len)`: Returns JSON array `[a, b]`.
    *   `psxmp_set_image_edit(file)`: Sets in-memory image edit reference.
    *   `psxmp_get_image_edit(buf, buf_len)`: Returns the filename.

### ID3 SYLT Extraction

*   `id3_ffi_extract_sylt(bytes, bytes_len, out_len)`: Extracts synchronized lyrics from MP3 bytes. Returns a JSON array of timed lines.

## Usage Examples

### Swift (Apple)

The Swift bindings are provided in `Sources/Api/`. Use the `Api` enum for AI tasks and `ProjectService` for local file management.

```swift
import Api

// Generate an image
let imageData = await Api.flux2Pro(
    user: "myuser",
    password: "mypass",
    prompt: "a cat in space"
)

// Save with metadata
ProjectService.saveFile(
    imageData,
    named: "cat.png",
    prompt: "a cat in space",
    model: "flux2-pro"
)

// Retrieve metadata
if let prompt = ProjectService.getPrompt("cat.png") {
    print("Prompt: \(prompt)")
}
```

### Kotlin (Android/Web)

The Kotlin bindings are generated via KMP. The Rust code is compiled to JNI for Android and inlined WASM for Web.

```kotlin
import market.femi.api.FemiApiJvm

// Android: Load library once
// System.loadLibrary("rust_ffi")

// Generate image
val bytes = FemiApiJvm.rustFfiFlux2Pro(
    user = "myuser",
    password = "mypass",
    prompt = "a cat in space",
    cancelFlag = 0L
)

// Project Service
ProjectServiceJvm.psxmpSaveFile(
    name = "cat.png",
    bytes = bytes,
    prompt = "a cat in space",
    model = "flux2-pro",
    subject = null
)
```

## Testing

### Rust Tests

Run Rust integration tests against the live API:

```bash
cd Rust
cargo test
```

Tests create a temporary user account (`funded-test-<uuid>`) with 50 credits.

### Swift Tests

Run Swift tests using Xcode or `swift test`. Tests use `Bundle.module` resources for fallback assets.

```bash
swift test
```

## Non-Obvious Conventions

1.  **Memory Management:** FFI functions returning `uint8_t*` allocate memory on the Rust heap. The caller **must** free this memory. In Swift, `Data(bytesNoCopy:count:deallocator:)` is used. In Kotlin/JNI, the `jni` crate handles conversion, but raw pointers must be managed carefully.
2.  **Cancellation:** The `cancel_flag` parameter is a pointer to a `u8`. If you pass a non-null pointer and set the value to `1`, the Rust async task will detect this and return a fallback asset (image/video/text) immediately. This is implemented using `tokio::select!` polling the flag every 10ms.
3.  **WebAssembly Inlining:** The Web build process inlines the `.wasm` binary into the JavaScript glue code as base64. This means the `pkg/rust_ffi.js` file is self-contained and does not require a separate `.wasm` file to be served. The Kotlin side decodes this base64 string at runtime to instantiate the WebAssembly module.
4.  **XMP Namespaces:** The project uses specific XMP namespaces for AI metadata:
    *   `http://iptc.org/std/Iptc4xmpExt/2008-02-29/`: `AIPromptInformation`, `AISystemUsed`
    *   `http://purl.org/dc/elements/1.1/`: `description`, `subject`
    *   `http://ns.adobe.com/xap/1.0/`: `CreatorTool`, `Rating`
5.  **Platform-Specific XMP Libraries:**
    *   **Apple:** Uses `xmp_toolkit` with smart handlers for robust metadata editing.
    *   **Android/Web:** Uses `xmpkit` (pure Rust) for bytes-in/bytes-out manipulation, avoiding native dependencies.