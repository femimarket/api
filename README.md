# Rust FFI & API Client

A cross-platform Rust library providing a unified FFI (Foreign Function Interface) for interacting with the Femi AI backend services. It exposes endpoints for image generation, video synthesis, speech-to-text, and LLM chat, as well as local file metadata management (XMP).

The project is designed to be consumed by **iOS/macOS** (via Swift Package Manager), **Android** (via JNI), and **Web** (via WebAssembly).

## Architecture

The codebase is split into two main layers:

1.  **`Rust/` (The Core)**: A Rust crate (`rust_ffi`) that contains the business logic. It communicates with the remote API (`https://femi.market/api`) and handles local file operations. It is compiled into platform-specific binaries:
    *   **Apple**: A static library linked into `RustFFI.xcframework`.
    *   **Android**: A shared library (`librust_ffi.so`) loaded via JNI.
    *   **Web**: A WebAssembly module (`pkg/rust_ffi.js` + embedded `.wasm`).

2.  **`Sources/Api/` (The Swift Wrapper)**: Swift extensions that wrap the Rust FFI calls, providing idiomatic async/await APIs, cancellation support, and data conversion.

### Key Directories

*   `Rust/src/api/`: Contains the implementation for each AI endpoint (e.g., `flux2_pro.rs`, `qwen3_asr_flash.rs`). Each file defines a `core_*` async function and platform-specific FFI bindings (`native` for Apple/Android, `wasm` for Web).
*   `Rust/src/project_service/`: Handles XMP metadata embedding and reading. Uses `xmp_toolkit` on Apple and a pure-Rust `xmpkit` crate on Android/Web.
*   `Rust/src/id3/`: Handles ID3 SYLT (synchronized lyrics) extraction from MP3 files.
*   `Sources/Api/`: Swift files that expose the `Api` enum and `ProjectService` singleton to Swift consumers.
*   `Kmp/api/src/`: Kotlin Multiplatform bindings for Android and Web.

## Features

### AI Endpoints
All endpoints accept `user` and `password` for authentication. They return binary data (images/videos) or strings (text/lyrics).

*   **Image Generation**:
    *   `z_image_turbo`: Standard image generation.
    *   `flux2_pro`: Flux 2 Pro image generation.
    *   `nano_banana2`: Nano Banana 2 image generation.
    *   `flux2_dev_i2i`: Flux 2 Dev Image-to-Image.
    *   `flux2_klein_i2i`: Flux 2 Klein Image-to-Image (supports two input images).
*   **Video Generation**:
    *   `ltx2_3a2v`: LTX-2 Audio-to-Video (generates video from image + audio + prompt).
*   **Audio/Text**:
    *   `qwen3_asr_flash`: Speech-to-text (lyrics extraction) from audio base64.
    *   `qwen3_6_35b_a3b`: LLM Chat (Qwen 3 6.35B A3B). Accepts a JSON array of messages and returns an updated array with the assistant's reply.

### Local File Services (`ProjectService`)
Manages XMP metadata in local files (PNG, JPEG, MP4, MOV, etc.) using the Adobe XMP Toolkit (Apple) or `xmpkit` (Android/Web).

*   **Embed**: Writes `prompt`, `model`, and `subject` keywords into file metadata.
*   **Read**: Retrieves prompt, model, subjects, and rating from file metadata.
*   **Like**: Sets a rating (5 for liked, 0 for not) in the file.
*   **Storage**: Saves files to the app's Documents directory.

### ID3 Lyrics
*   `extract_sylt`: Extracts synchronized lyrics (SYLT frame) from MP3 bytes and returns them as a JSON array.

## Installation & Build

### Prerequisites

*   **Rust**: `rustup` and `cargo`.
*   **Android NDK**: Required for Android targets.
*   **Xcode**: Required for Apple targets (`xcodebuild`).
*   **Node.js**: Required for WebAssembly generation.
*   **wasm-bindgen-cli**: Installed automatically by the build script if missing.

### Building All Platforms

Run the build script from the repository root:

```bash
./build-rust.sh
```

This script performs the following:
1.  Installs necessary Rust targets (`aarch64-apple-ios`, `x86_64-apple-darwin`, `arm64-v8a`, etc.).
2.  Builds the Rust crate for all platforms.
3.  Creates `RustFFI.xcframework` for iOS/macOS.
4.  Copies `.so` files to `Kmp/api/src/androidMain/jniLibs/` for Android.
5.  Generates the WebAssembly module, inlines the `.wasm` into the JS glue code, and embeds the JS as a base64 string in `Kmp/api/src/webMain/kotlin/market/femi/api/RustFfiBundle.kt`.

### Manual Build Steps

If you need to rebuild only specific parts:

**Apple (iOS/macOS)**
```bash
cd Rust
cargo build --release --target aarch64-apple-ios
cargo build --release --target aarch64-apple-ios-sim
cargo build --release --target x86_64-apple-ios
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-apple-darwin

# Create xcframework
xcodebuild -create-xcframework \
  -library target/aarch64-apple-ios/release/librust_ffi.a \
  -library target/ios-sim-universal/release/librust_ffi.a \
  -library target/macos-universal/release/librust_ffi.a \
  -headers include \
  -output ../RustFFI.xcframework
```

**Android**
```bash
cd Rust
# Ensure ANDROID_NDK_HOME is set or configured in .cargo/config.toml
cargo build --release --target aarch64-linux-android
cargo build --release --target armv7-linux-androideabi
cargo build --release --target x86_64-linux-android
cargo build --release --target i686-linux-android

# Copy to Android project structure
mkdir -p Kmp/api/src/androidMain/jniLibs/arm64-v8a
cp target/aarch64-linux-android/release/librust_ffi.so Kmp/api/src/androidMain/jniLibs/arm64-v8a/
# ... repeat for other ABIs
```

**WebAssembly**
```bash
cd Rust
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/rust_ffi.wasm --out-dir pkg --target web

# Inline WASM into JS (see build-rust.sh for the Node.js script)
```

## Usage

### Swift (iOS/macOS)

Import the `Api` package in your `Package.swift`:

```swift
.package(path: "../"), // or remote URL
```

Use the `Api` enum for AI endpoints:

```swift
import Api

let image = await Api.flux2Pro(
    user: "myuser",
    password: "mypass",
    prompt: "a cat in space"
)
```

Use `ProjectService` for local file management:

```swift
import Api

// Save a file with metadata
ProjectService.saveFile(
    imageData,
    named: "generated.png",
    prompt: "a cat in space",
    model: "flux2-pro"
)

// Read metadata
let prompt = ProjectService.getPrompt("generated.png")
let isLiked = ProjectService.getLike("generated.png")
```

### Kotlin Multiplatform (Android/Web)

The Kotlin bindings are generated in `Kmp/api/src/`.

**Android**:
Load the library via `System.loadLibrary("rust_ffi")` in your JVM entry point. The FFI functions are exposed as `external fun` in Kotlin classes like `FemiApiJvm`.

**Web**:
The WebAssembly module is embedded in `RustFfiBundle.kt` as a base64 string. The Kotlin code decodes this at runtime and initializes the WASM module. No external `.wasm` file is required.

```kotlin
// Example usage in Kotlin
val result = FemiApiJvm.rustFfiFlux2Pro(user, password, prompt, cancelFlag)
```

## API Reference

### Rust FFI Functions (C ABI)

Defined in `Rust/include/RustFFI/RustFFI.h`.

#### AI Endpoints
All AI functions return `uint8_t*` (heap-allocated bytes) and set `size_t* out_len`. The caller is responsible for freeing the memory using `free()`.

*   `rust_ffi_z_image_turbo(user, password, prompt, cancel_flag, out_len)`
*   `rust_ffi_flux2_pro(user, password, prompt, cancel_flag, out_len)`
*   `rust_ffi_nano_banana2(user, password, prompt, cancel_flag, out_len)`
*   `rust_ffi_flux2_dev_i2i(user, password, image_b64, prompt, cancel_flag, out_len)`
*   `rust_ffi_flux2_klein_i2i(user, password, image_b64, image2_b64, prompt, cancel_flag, out_len)`
*   `rust_ffi_ltx2_3a2v(user, password, image_b64, audio_b64, prompt, cancel_flag, out_len)`
*   `rust_ffi_qwen3_asr_flash(user, password, audio_b64, cancel_flag, out_len)`
*   `rust_ffi_qwen3_6_35b_a3b(user, password, messages_json, cancel_flag, out_len)`

**Cancellation**: Pass a pointer to a `uint8_t` flag. If the flag is non-zero, the operation will stop and return a fallback response (fallback image/video or "Could not respond").

#### ProjectService XMP FFI
*   `psxmp_embed(path, prompt, model, subject, subject_count)`: Embeds metadata. Returns 0 on success, -1 on failure.
*   `psxmp_read_prompt(path, buf, buf_len)`: Reads prompt into buffer. Returns length written.
*   `psxmp_read_model(path, buf, buf_len)`: Reads model into buffer.
*   `psxmp_read_subject_count(path)`: Returns number of subjects.
*   `psxmp_read_subject_at(path, index, buf, buf_len)`: Reads subject at index.
*   `psxmp_set_rating(path, rating)`: Sets rating (0-5). Returns 0 on success.
*   `psxmp_read_rating(path)`: Returns rating (-100 if absent).
*   `psxmp_read_property(path, namespace_uri, property_name, buf, buf_len)`: Reads arbitrary XMP property.

#### ID3 FFI
*   `id3_ffi_extract_sylt(bytes, bytes_len, out_len)`: Extracts SYLT lyrics. Returns JSON string bytes.

## Testing

### Rust Tests
Run tests for native targets (Apple/Android):
```bash
cd Rust
cargo test
```

### Swift Tests
Run Swift tests via Xcode or `swift test`:
```bash
swift test
```

## Non-Obvious Conventions

1.  **Fallback Handling**: If an API call fails or is cancelled, the Rust FFI returns embedded fallback assets (images/videos) or specific error strings. This ensures the UI always has content to display.
2.  **Memory Management**:
    *   **Rust -> C**: Rust allocates memory with `Box::into_raw`. The caller **must** call `free()` on the returned pointer.
    *   **Swift**: The Swift wrappers use `Data(bytesNoCopy:count:deallocator:.free)` to automatically free the memory when the `Data` object is deallocated.
    *   **Kotlin/Android**: The JNI macros handle allocation and conversion to `jbyteArray`, which is managed by the JVM.
3.  **Cancellation**: The `cancel_flag` is a shared memory location. The Rust code polls this flag every 10ms. Setting it to non-zero triggers a graceful shutdown of the async task.
4.  **WebAssembly Inlining**: The `build-rust.sh` script inlines the `.wasm` binary into the JavaScript glue code as base64. This eliminates the need for a separate HTTP request to fetch the `.wasm` file, simplifying deployment.
5.  **XMP Namespaces**: The project uses specific XMP namespaces:
    *   `http://purl.org/dc/elements/1.1/` (DC)
    *   `http://ns.adobe.com/xap/1.0/` (XMP)
    *   `http://iptc.org/std/Iptc4xmpExt/2008-02-29/` (IPTC Ext)