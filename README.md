# RustFFI

A cross-platform Rust FFI crate that provides a unified interface for AI media generation (image, video, audio transcription, LLM chat) and local project metadata management (XMP). It targets **Apple (iOS/macOS)**, **Android**, and **WebAssembly (Kotlin Multiplatform)**.

## Overview

This project bridges a remote AI API (`https://femi.market/api`) and local file storage to native mobile and web applications. It handles:

1.  **AI Endpoints**: Image generation, image-to-image, video generation, audio transcription, and LLM chat.
2.  **Project Service**: Local storage of generated assets with embedded XMP metadata (prompts, models, subjects, ratings) and in-memory state for character casting/image editing.
3.  **ID3 Processing**: Extraction of synchronized lyrics (SYLT) from MP3 files.

The crate is compiled into platform-specific binaries:
*   **Apple**: `RustFFI.xcframework` (static library wrapped in an XCFramework).
*   **Android**: `librust_ffi.so` (JNI library).
*   **Web**: `pkg/rust_ffi.js` + embedded `.wasm` (self-contained ES module).

## Architecture

The codebase is split into three main layers:

### 1. Rust Core (`Rust/`)
The source of truth. Uses conditional compilation (`#[cfg]`) to expose different interfaces per platform.

*   **`src/api/`**: Implements the AI endpoints. Each endpoint (e.g., `flux2_pro.rs`) contains:
    *   `core_*`: Pure async Rust logic (HTTP POST, JSON parsing, fallback resolution).
    *   `native`: C-ABI wrappers for Apple/Android. Handles string marshaling, cancellation flags, and memory ownership.
    *   `wasm`: `wasm_bindgen` wrappers for Web.
*   **`src/project_service/`**: Manages local files.
    *   `shared/xmpkit_body.rs`: Pure Rust logic for embedding/reading XMP metadata into bytes. Used by Android and WASM.
    *   `apple.rs`: Uses `xmp_toolkit` for smart XMP handling on Apple.
    *   `android.rs`: JNI wrappers that read/write files directly using `xmpkit_body`.
    *   `wasm.rs`: Uses the Web File System Access API (OPFS) for storage.
*   **`src/id3/`**: MP3 SYLT extraction.
*   **`src/lib.rs`**: Entry point, global Tokio runtime (native), and shared constants.

### 2. Platform Bindings
*   **Swift (`Sources/Api/`)**: Thin wrappers around the C-ABI. Uses `withTaskCancellationHandler` to support async cancellation.
*   **Kotlin/Android (`Kmp/api/src/androidMain/`)**: Uses JNI to call `librust_ffi.so`.
*   **Kotlin/Web (`Kmp/api/src/webMain/`)**: Loads the embedded WASM module from a base64 constant (`RustFfiBundle.kt`).

### 3. Build System
*   **`build-rust.sh`**: The single entry point for building. It cross-compiles for all targets, generates the XCFramework, copies `.so` files for Android, and inlines the WASM into the JS glue code and Kotlin bundle.

## Key Files

| Path | Description |
| :--- | :--- |
| `build-rust.sh` | Master build script. Run this to generate all artifacts. |
| `Rust/src/lib.rs` | Rust library root. Defines global clients and fallback assets. |
| `Rust/include/RustFFI/RustFFI.h` | The C header defining the public FFI API. |
| `RustFFI.xcframework/` | Generated Apple binary. |
| `Kmp/api/src/androidMain/jniLibs/` | Generated Android `.so` libraries. |
| `Rust/pkg/` | Generated Web artifacts (JS + embedded WASM). |
| `Kmp/api/src/webMain/kotlin/market/femi/api/RustFfiBundle.kt` | Generated Kotlin constant containing the base64-encoded WASM module. |
| `Sources/Api/*.swift` | Swift extensions for `Api` enum and `ProjectService`. |

## Installation & Build

### Prerequisites
*   **Rust**: `rustup` installed.
*   **Android NDK**: Installed and configured in `Rust/.cargo/config.toml`.
*   **Apple SDKs**: Xcode command line tools.
*   **Node.js**: Required for the WASM inlining step.
*   **wasm-bindgen-cli**: Installed automatically by the build script if missing.

### Building All Targets
From the repository root:

```bash
chmod +x build-rust.sh
./build-rust.sh
```

This script will:
1.  Add necessary Rust targets (`aarch64-apple-ios`, `aarch64-linux-android`, `wasm32-unknown-unknown`, etc.).
2.  Build the Apple static libraries and combine them into `RustFFI.xcframework`.
3.  Build Android `.so` files and place them in `Kmp/api/src/androidMain/jniLibs/`.
4.  Build the WASM module, inline it into the JS glue code, and generate the Kotlin bundle.

### Building Only Rust (for testing)
```bash
cd Rust
cargo build --release
```

## Usage

### Swift (iOS/macOS)
Import the `Api` module. The `Api` enum provides async methods for each AI endpoint.

```swift
import Api

// Image Generation
let image = await Api.flux2Pro(
    user: "my_user",
    password: "my_pass",
    prompt: "a cat in space"
)

// Project Service
ProjectService.saveFile(image, named: "cat.png", prompt: "a cat in space")
let prompt = ProjectService.getPrompt("cat.png")
```

### Kotlin Multiplatform (Android)
Call the JVM methods via the generated JNI bindings.

```kotlin
// Android
val image = FemiApiJvm.rustFfiFlux2Pro(user, password, prompt)
ProjectServiceJvm.psxmpSaveFile("cat.png", image, "a cat in space", null, null)
```

### Kotlin Multiplatform (Web)
The WASM module is embedded in the Kotlin library. Initialize it once.

```kotlin
// Web
import market.femi.api.RustFfiBundle

// Initialize the WASM module (decodes base64 and instantiates)
RustFfiBundle.init()

// Call API
val image = RustFfi.wasmFlux2Pro(user, password, prompt)
```

## API Reference

### AI Endpoints
All AI endpoints follow a similar signature:
*   **Inputs**: `user`, `password`, `prompt` (or image/audio bytes), `cancel_flag` (native only).
*   **Output**: `uint8_t*` (native) or `Uint8Array` (Wasm) containing the result bytes.
*   **Cancellation**: On native platforms, pass a pointer to a `uint8_t` flag. Setting it to `1` cancels the operation, returning a fallback image/video.

| Function | Description |
| :--- | :--- |
| `rust_ffi_z_image_turbo` | Text-to-Image |
| `rust_ffi_flux2_pro` | Text-to-Image (Flux Pro) |
| `rust_ffi_flux2_dev_i2i` | Image-to-Image (Flux Dev) |
| `rust_ffi_flux2_klein_i2i` | Image-to-Image (Flux Klein, 2 images) |
| `rust_ffi_nano_banana2` | Text-to-Image (Nano Banana) |
| `rust_ffi_ltx2_3a2v` | Image+Audio-to-Video |
| `rust_ffi_qwen3_asr_flash` | Audio-to-Text (Lyrics) |
| `rust_ffi_qwen3_6_35b_a3b` | Chat (LLM) |

### Project Service (XMP)
Rust owns the file system. All paths are relative to the app's Documents directory.

| Function | Description |
| :--- | :--- |
| `psxmp_save_file` | Save file with optional XMP metadata (prompt, model, subjects). |
| `psxmp_save_audio` | Save audio file (replaces any existing audio). |
| `psxmp_like` | Set rating (like/unlike) via XMP. |
| `psxmp_get_all_generations` | List all files in Documents. |
| `psxmp_get_audio` | Get the current audio file name. |
| `psxmp_get_prompt` | Read prompt from XMP. |
| `psxmp_get_model` | Read model name from XMP. |
| `psxmp_get_subject` | Read subjects from XMP. |
| `psxmp_get_like` | Check if file is liked. |
| `psxmp_get_url` | Get full path for a filename. |
| `psxmp_set_character_cast` | Set in-memory character cast (A -> B). |
| `psxmp_set_image_edit` | Set in-memory image edit target. |

### ID3
| Function | Description |
| :--- | :--- |
| `id3_ffi_extract_sylt` | Extract synchronized lyrics from MP3 bytes. Returns JSON array. |

## Testing

### Rust Tests
Run native integration tests against the live API:
```bash
cd Rust
cargo test
```
*Note: Tests create a temporary user account on the server. Ensure you have network access.*

### Swift Tests
Run via Xcode or `swift test`. Tests use `Bundle.module` resources for fallback assets.

### Android Tests
Run via Android Studio or Gradle. Tests use the JNI bindings.

## Non-Obvious Conventions

1.  **Memory Ownership**:
    *   **Native**: The Rust FFI functions return a `*mut u8` allocated with `Box::into_raw`. The caller **must** free this memory using `free()` (Swift) or `Box::from_raw` (Kotlin/JNI). The `out_len` parameter is filled with the byte count.
    *   **Wasm**: Returns `Uint8Array` or `String`. Memory is managed by the JS runtime.

2.  **Cancellation**:
    *   Native functions accept a `cancel_flag: *const u8`. This is a pointer to a shared memory location. The Rust code polls this flag every 10ms. If set to non-zero, it aborts the async task and returns a fallback asset.
    *   Swift/Kotlin wrappers use `withTaskCancellationHandler` to set this flag when the Swift `Task` is cancelled.

3.  **WASM Inlining**:
    *   The `build-rust.sh` script inlines the `.wasm` binary into the `rust_ffi.js` glue code as a base64 string. This eliminates the need to serve a separate `.wasm` file.
    *   The JS module is then base64-encoded again and embedded into `RustFfiBundle.kt` for Kotlin Multiplatform Web.

4.  **XMP Metadata**:
    *   **Apple**: Uses `xmp_toolkit`'s smart handler, which preserves existing XMP data and updates only specific fields.
    *   **Android/WASM**: Uses a pure Rust byte-manipulation approach (`xmpkit_body`) to embed XMP into the file bytes without external dependencies.

5.  **Fallback Assets**:
    *   If an API call fails (network error, 402 payment required, etc.), the Rust code returns embedded fallback assets (`fallback.png`, `topup.jpg`, `could-not-generate.mp4`) instead of empty data. This ensures the UI always has something to display.