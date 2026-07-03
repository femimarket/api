# RustFFI

A cross-platform Rust FFI crate providing a unified interface to the Femi AI backend (`https://femi.market/api`) and local project management. It compiles to native binaries for Apple platforms (iOS/macOS), Android, and WebAssembly, exposing C-compatible functions for image/video generation, transcription, chat, and XMP metadata management.

## Architecture

The project is split into two main layers:

1.  **Rust Core (`Rust/`)**: The source of truth. It contains the FFI bindings, API client logic, and platform-specific backends.
2.  **Swift Wrapper (`Sources/Api/`)**: A thin Swift layer (`Api` enum) that marshals arguments to the Rust FFI, handles cancellation flags, and returns `Data`.

### Platform Targets

*   **Apple (iOS/macOS)**:
    *   Built as static libraries (`librust_ffi.a`) for multiple architectures.
    *   Combined into `RustFFI.xcframework` via `build-rust.sh`.
    *   Consumed by SwiftPM via `Package.swift`.
    *   Uses `xmp_toolkit` for XMP metadata on Apple.
*   **Android**:
    *   Built as shared libraries (`librust_ffi.so`) for `arm64-v8a`, `armeabi-v7a`, `x86_64`, `x86`.
    *   Placed in `Kmp/api/src/androidMain/jniLibs/`.
    *   Exposed via JNI symbols (e.g., `Java_market_femi_api_FemiApiJvm_*`).
    *   Uses `xmpkit` (pure Rust) for XMP metadata on Android.
*   **WebAssembly (KMP)**:
    *   Built to `wasm32-unknown-unknown`.
    *   Processed by `wasm-bindgen` to generate JS glue.
    *   The `.wasm` is inlined into the JS glue as base64, and the JS is further embedded into a Kotlin constant (`RustFfiBundle.kt`) to ship inside the KMP klib without external assets.
    *   Uses OPFS (Origin Private File System) for file storage on Web.

## Key Files

*   `build-rust.sh`: The master build script. Runs `cargo build` for all targets, creates the `.xcframework`, copies `.so` files for Android, and inlines WASM assets.
*   `Rust/src/lib.rs`: Entry point. Defines the server URL, fallback assets, and shared client/runtime logic.
*   `Rust/src/api/`: Individual modules for each AI endpoint (e.g., `flux2_pro.rs`, `qwen3_asr_flash.rs`). Each contains:
    *   `core_*`: Async logic that POSTs to the server.
    *   `native` module: C-ABI wrapper for native platforms (handles cancellation flags).
    *   `wasm` module: `wasm_bindgen` wrapper for Web.
*   `Rust/src/project_service/`: Handles local file storage and XMP metadata.
    *   `apple.rs`: Uses `xmp_toolkit` and `dirs::document_dir()`.
    *   `android.rs`: Uses JNI and `xmpkit_body` (bytes-based XMP).
    *   `wasm.rs`: Uses OPFS and `xmpkit_body`.
    *   `share.rs`: Shared XMP namespace constants and `xmpkit_body` logic.
*   `Rust/include/RustFFI/RustFFI.h`: The public C header defining all FFI symbols.
*   `Sources/Api/`: Swift extensions for `Api` and `ProjectService` that call the Rust FFI.

## Installation & Build

### Prerequisites

*   **Rust**: `rustup` installed.
*   **Android NDK**: Required for Android targets. Ensure `Rust/.cargo/config.toml` points to the correct NDK path.
*   **Wasm-bindgen**: Installed automatically by the build script if missing.
*   **Apple Tools**: `xcodebuild` and `lipo` for creating the `.xcframework`.
*   **Node.js**: Required for the WASM inlining step in `build-rust.sh`.

### Building

Run the build script from the repository root:

```bash
./build-rust.sh
```

This script performs the following:
1.  Installs necessary Rust targets (`aarch64-apple-ios`, `x86_64-apple-darwin`, `wasm32-unknown-unknown`, etc.).
2.  Builds the Rust crate in `--release` mode for all targets.
3.  **Apple**: Creates `RustFFI.xcframework` containing universal iOS simulator, iOS device, and macOS binaries.
4.  **Android**: Copies `librust_ffi.so` to `Kmp/api/src/androidMain/jniLibs/<abi>/`.
5.  **WASM**: Runs `wasm-bindgen`, then inlines the `.wasm` binary into the JS glue code as base64. Finally, it embeds the JS into `Kmp/api/src/webMain/kotlin/market/femi/api/RustFfiBundle.kt`.

## Usage

### Swift (Apple)

Import the `Api` package via SwiftPM. The `Api` enum provides async methods for each endpoint.

```swift
import Api

// Image Generation
let image = await Api.flux2Pro(
    user: "username",
    password: "password",
    prompt: "a red apple"
)

// Video Generation
let video = await Api.ltx2_3a2v(
    user: "username",
    password: "password",
    image: imageData,
    audio: audioData,
    prompt: "the man walks"
)

// Project Service (Local Storage)
ProjectService.saveFile(image, named: "apple.png", prompt: "a red apple")
let prompt = ProjectService.getPrompt("apple.png")
```

### Kotlin (Android/JVM)

The Rust FFI is exposed via JNI. Use the `FemiApiJvm` class (generated/defined in the Kotlin side) to call functions like `rustFfiFlux2Pro`.

```kotlin
// Example JNI call structure (actual class names depend on Kotlin bindings)
val result = FemiApiJvm.rustFfiFlux2Pro(
    user = "username",
    password = "password",
    prompt = "a red apple",
    cancelFlag = 0L // 0 for no cancellation, or a pointer to an atomic flag
)
```

### Kotlin (Web/WASM)

For KMP Web, the Rust FFI is bundled inside the `RustFfiBundle.kt` constant. The Kotlin side decodes the base64 JS, creates a Blob URL, and imports it.

```kotlin
// The RustFfiBundle.kt file contains:
// internal val RUST_FFI_JS_B64: String = "..."

// Usage is handled by the KMP wrapper which initializes the WASM module
// and exposes async functions mirroring the native API.
```

## API Endpoints

The following endpoints are exposed via `rust_ffi_*` functions. All return `uint8_t*` (bytes) and `size_t` (length).

| Function | Description | Inputs |
| :--- | :--- | :--- |
| `rust_ffi_z_image_turbo` | Generate image | `user`, `password`, `prompt` |
| `rust_ffi_flux2_pro` | Generate image (Flux 2 Pro) | `user`, `password`, `prompt` |
| `rust_ffi_flux2_dev_i2i` | Image-to-Image (Flux 2 Dev) | `user`, `password`, `image_b64`, `prompt` |
| `rust_ffi_flux2_klein_i2i` | Image-to-Image (Flux 2 Klein) | `user`, `password`, `image_b64`, `image2_b64`, `prompt` |
| `rust_ffi_nano_banana2` | Generate image (Nano Banana) | `user`, `password`, `prompt` |
| `rust_ffi_ltx2_3a2v` | Video generation (LTX-2) | `user`, `password`, `image_b64`, `audio_b64`, `prompt` |
| `rust_ffi_qwen3_asr_flash` | Audio transcription (Lyrics) | `user`, `password`, `audio_b64` |
| `rust_ffi_qwen3_6_35b_a3b` | Chat completion (Qwen 3) | `user`, `password`, `messages_json` |

### Cancellation

Native functions accept a `cancel_flag: *const u8`. If non-null, the Rust side polls this memory address. If the value becomes non-zero, the operation is cancelled and returns a fallback asset (e.g., `FALLBACK_IMAGE` or `FALLBACK_VIDEO`).

### Fallbacks

If the server returns a 402 (payment required) or an error, the Rust FFI automatically returns embedded fallback assets:
*   `Assets/fallback.png` (Image errors)
*   `Assets/topup.jpg` (Payment required for images)
*   `Assets/could-not-generate.mp4` (Video errors)
*   `Assets/topup-video.mp4` (Payment required for video)

## Project Service (XMP)

The `psxmp_*` functions manage local files and XMP metadata.

*   **Storage**:
    *   **Apple**: `Documents/` directory.
    *   **Android**: `context.filesDir` (initialized via `psxmpInitDocuments`).
    *   **Web**: OPFS (Origin Private File System).
*   **Metadata**:
    *   Prompts are stored in `IPTC:Ext:AIPromptInformation` and `DC:description`.
    *   Models are stored in `IPTC:Ext:AISystemUsed` and `XMP:CreatorTool`.
    *   Subjects are stored in `DC:subject` array.
    *   Likes (ratings) are stored in `XMP:Rating`.

## Testing

### Rust Tests

Run native tests (Apple/Android host):

```bash
cd Rust
cargo test
```

Tests are located in `Rust/tests/`. They use a fresh test user account (`funded-test-<uuid>`) to ensure isolation.

### Swift Tests

Run Swift tests via Xcode or Swift Package Manager:

```bash
swift test
```

Tests are in `Tests/ApiTests/`. They mirror the Rust test logic, verifying the Swift wrapper's correctness and cancellation behavior.