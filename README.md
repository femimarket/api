# RustFFI

A cross-platform Rust FFI crate providing a unified interface to AI generation APIs (image, video, audio transcription, LLM chat) and local file metadata management (XMP). It is designed to be consumed by Swift (iOS/macOS), Kotlin (Android), and WebAssembly (Web) clients.

## Overview

This project compiles a single Rust library into three distinct artifacts:
1.  **`RustFFI.xcframework`**: A universal XCFramework for Apple platforms (iOS, macOS, tvOS, watchOS).
2.  **`jniLibs`**: Native `.so` libraries for Android, organized by ABI.
3.  **`pkg/`**: WebAssembly bindings generated via `wasm-bindgen`.

The crate exposes two main functional areas:
*   **API Endpoints**: Async calls to remote AI services (Flux, ZImage, Qwen, etc.) with built-in cancellation support and fallback asset handling.
*   **ProjectService (XMP)**: Local metadata embedding and reading for images and videos using the Adobe XMP Toolkit (Apple) or a pure-Rust byte-based implementation (Android/WASM).

## Architecture

The codebase is split into `Rust/` (the FFI core) and `Sources/` (Swift wrappers).

### Rust Core (`Rust/`)

*   **`src/lib.rs`**: Entry point. Defines shared constants (fallback images/videos), the global `reqwest` client, and the Tokio runtime singleton for native platforms.
*   **`src/api/`**: Contains the implementation for each AI endpoint.
    *   Each endpoint (e.g., `flux2_pro.rs`) has a `core_*` async function that handles the HTTP request and response parsing.
    *   It exposes a `native` module (for iOS/Android) with `extern "C"` functions that manage memory allocation and cancellation flags.
    *   It exposes a `wasm` module with `#[wasm_bindgen]` functions for browser usage.
*   **`src/project_service/`**: Handles XMP metadata.
    *   `shared/xmpkit_body.rs`: Pure Rust logic for reading/writing XMP bytes. Used by Android and WASM.
    *   `apple.rs`: Uses the `xmp_toolkit` crate to interact with the OS-level smart handlers for various file formats.
    *   `android.rs` / `wasm.rs`: Platform-specific wrappers that bridge JNI/OPFS to the shared logic.
*   **`src/id3/`**: Extracts synchronized lyrics (SYLT) from MP3 files.

### Swift Wrappers (`Sources/Api/`)

Swift extensions on `Api` enum provide safe, async interfaces to the Rust FFI.
*   **Cancellation**: Uses `withTaskCancellationHandler` to set a flag in Rust, which triggers a `tokio::select!` fallback to return embedded placeholder assets (e.g., `fallback.png`) instead of waiting for the network call.
*   **Memory Management**: Uses `Data(bytesNoCopy:count:deallocator:.free)` to avoid copying the byte slices returned by Rust.

### Android Integration (`Kmp/`)

*   **`Kmp/api/src/androidMain/jniLibs/`**: Contains the compiled `.so` files.
*   **`Rust/src/api/jni_api.rs`**: Generates JNI symbols (e.g., `Java_market_femi_kotlinapi_FemiApiJvm_rustFfiFlux2Pro`) that map directly to Kotlin `external fun` declarations.

## Installation & Build

### Prerequisites

*   **Rust**: `rustup` installed.
*   **Android NDK**: Required for Android targets. Ensure `ANDROID_NDK_HOME` or `NDK_HOME` is set, or configured in `Rust/.cargo/config.toml`.
*   **Xcode**: Required for Apple targets and `xcodebuild`.
*   **wasm-bindgen-cli**: Installed automatically by the build script if missing.

### Build Script

Run the provided shell script from the repository root to build all targets:

```bash
./build-rust.sh
```

This script:
1.  Installs necessary Rust targets (`aarch64-apple-ios`, `armv7-linux-androideabi`, `wasm32-unknown-unknown`, etc.).
2.  Builds static libraries for Apple, Android, and WASM.
3.  Creates `RustFFI.xcframework` in the root directory.
4.  Copies Android `.so` files to `Kmp/api/src/androidMain/jniLibs/`.
5.  Generates WASM bindings in `Rust/pkg/`.

### Manual Build Steps

If you prefer to build manually:

1.  **Add Targets**:
    ```bash
    rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios aarch64-apple-darwin x86_64-apple-darwin wasm32-unknown-unknown
    rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android
    cargo install wasm-bindgen-cli
    ```

2.  **Build Apple**:
    ```bash
    cd Rust
    for t in aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios aarch64-apple-darwin x86_64-apple-darwin; do
      cargo build --release --target $t
    done
    # Create xcframework (requires xcodebuild)
    xcodebuild -create-xcframework \
      -library target/aarch64-apple-ios/release/librust_ffi.a -headers include \
      -library target/aarch64-apple-ios-sim/release/librust_ffi.a -headers include \
      -library target/aarch64-apple-darwin/release/librust_ffi.a -headers include \
      -output ../RustFFI.xcframework
    ```

3.  **Build Android**:
    ```bash
    for t in aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android; do
      cargo build --release --target $t
      mkdir -p ../Kmp/api/src/androidMain/jniLibs/${t#*-linux-android}
      cp target/$t/release/librust_ffi.so ../Kmp/api/src/androidMain/jniLibs/${t#*-linux-android}/librust_ffi.so
    done
    ```

4.  **Build WASM**:
    ```bash
    cargo build --release --target wasm32-unknown-unknown
    wasm-bindgen target/wasm32-unknown-unknown/release/rust_ffi.wasm --out-dir pkg --target web
    ```

## API Reference

### AI Endpoints

All API endpoints require `user` and `password` for basic auth against the server (`https://femi.market/api`).

| Endpoint | Rust FFI Function | Swift Method | Description |
| :--- | :--- | :--- | :--- |
| **ZImage Turbo** | `rust_ffi_z_image_turbo` | `Api.zImageTurbo` | Text-to-Image generation. |
| **Flux 2 Pro** | `rust_ffi_flux2_pro` | `Api.flux2Pro` | Text-to-Image generation (Flux Pro). |
| **Flux 2 Dev I2I** | `rust_ffi_flux2_dev_i2i` | `Api.flux2DevI2I` | Image-to-Image generation. |
| **Flux 2 Klein I2I** | `rust_ffi_flux2_klein_i2i` | `Api.flux2KleinI2I` | Image-to-Image with two input images. |
| **Nano Banana 2** | `rust_ffi_nano_banana2` | `Api.nanoBanana2` | Text-to-Image generation (Nano Banana). |
| **LTX 2 3A2V** | `rust_ffi_ltx2_3a2v` | `Api.ltx2_3a2v` | Image + Audio to Video generation. |
| **Qwen 3 ASR Flash** | `rust_ffi_qwen3_asr_flash` | `Api.qwen3AsrFlash` | Audio transcription to lyrics/text. |
| **Qwen 3 6.35B A3B** | `rust_ffi_qwen3_6_35b_a3b` | `Api.qwen3_6_35b_a3b` | LLM Chat completion. |

**Cancellation**:
All API functions accept a `cancel_flag` pointer (native) or use Swift's `Task` cancellation handler. If cancelled, the function returns immediately with embedded fallback assets (e.g., `fallback.png` or `could-not-generate.mp4`) instead of waiting for the network response.

### ProjectService (XMP)

Manages local file metadata.

| Operation | Rust FFI Function | Swift Method | Description |
| :--- | :--- | :--- | :--- |
| **Embed** | `psxmp_embed` | `ProjectService.saveFile` | Writes prompt, model, and subject metadata into a file. |
| **Read Prompt** | `psxmp_read_prompt` | `ProjectService.getPrompt` | Reads `dc:description` or `Iptc4xmpExt:AIPromptInformation`. |
| **Read Model** | `psxmp_read_model` | `ProjectService.getModel` | Reads `xmp:CreatorTool` or `Iptc4xmpExt:AISystemUsed`. |
| **Read Subject** | `psxmp_read_subject_count` / `psxmp_read_subject_at` | `ProjectService.getSubject` | Reads `dc:subject` array. |
| **Set Rating** | `psxmp_set_rating` | `ProjectService.like` | Writes `xmp:Rating` (5 = liked, 0 = not). |
| **Read Rating** | `psxmp_read_rating` | `ProjectService.getLike` | Reads `xmp:Rating`. Returns -100 if absent. |
| **Read Property** | `psxmp_read_property` | *(Internal)* | Reads arbitrary XMP property by namespace and name. |

**Metadata Mapping**:
*   **Prompt**: Stored in `dc:description` (Lang Alt) and `Iptc4xmpExt:AIPromptInformation`.
*   **Model**: Stored in `xmp:CreatorTool` and `Iptc4xmpExt:AISystemUsed`.
*   **Subject**: Stored in `dc:subject` (Bag).

### ID3 SYLT

| Operation | Rust FFI Function | Description |
| :--- | :--- | :--- |
| **Extract Lyrics** | `id3_ffi_extract_sylt` | Extracts synchronized lyrics from MP3 bytes. Returns JSON array of timed lines. |

## Key Files & Paths

*   `build-rust.sh`: The master build script.
*   `Rust/src/lib.rs`: Core library definitions, fallback assets, and client setup.
*   `Rust/include/RustFFI/RustFFI.h`: The C header file defining the FFI interface.
*   `RustFFI.xcframework/`: The compiled Apple framework.
*   `Kmp/api/src/androidMain/jniLibs/`: Android native libraries.
*   `Rust/pkg/`: WebAssembly bindings.
*   `Sources/Api/`: Swift wrapper implementations.
*   `Tests/ApiTests/`: Swift unit and integration tests.

## Testing

### Swift Tests
Run tests via Xcode or Swift Package Manager:
```bash
swift test
```
Tests cover API endpoints (using a test user account), cancellation behavior, and ProjectService XMP round-trips.

### Rust Tests
Run integration tests for the FFI layer:
```bash
cd Rust
cargo test
```
Tests verify API responses, XMP embedding/reading, and ID3 extraction.

## Non-Obvious Conventions

1.  **Memory Ownership**: Rust functions return `*mut u8`. The caller is responsible for freeing this memory using `free()` (C) or `.free` deallocator (Swift). The Swift wrappers handle this automatically via `Data(bytesNoCopy:deallocator:.free)`.
2.  **Fallback Assets**: If an API call fails or is cancelled, the Rust FFI returns embedded fallback assets (PNG/JPG/MP4) compiled into the binary. This ensures the UI always has something to display.
3.  **Cancellation Flag**: The `cancel_flag` is a pointer to a `u8`. Setting it to non-zero signals the Rust side to abort the async operation and return the fallback asset.
4.  **XMP Namespace**: The project uses standard XMP namespaces (`dc`, `xmp`, `Iptc4xmpExt`). The Apple backend uses the Adobe XMP Toolkit for smart handling of different file formats, while Android/WASM use a pure-Rust byte-based approach.
5.  **Test Users**: API tests use a unique username per run (`funded-test-<uuid>`) to ensure a funded account (50 credits) for the test duration.