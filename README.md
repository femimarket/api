# Rust FFI Bridge for Femi API

This repository contains the Rust-based FFI (Foreign Function Interface) layer for the **Femi** application. It provides a unified, cross-platform binary interface for consuming AI media generation APIs (image, video, audio transcription, and LLM chat) and managing local project metadata (XMP).

The crate compiles to:
1.  **`RustFFI.xcframework`**: A static library for iOS/macOS (Swift/SwiftUI).
2.  **`librust_ffi.so`**: Native shared libraries for Android (Kotlin/JNI).
3.  **`pkg/rust_ffi.js`**: A self-contained WebAssembly module for Kotlin Multiplatform (KMP) Web targets.

## Architecture

The project is structured into three main domains, each with platform-specific bindings:

### 1. API Endpoints (`Rust/src/api/`)
These functions communicate with the remote Femi server (`https://femi.market/api`). They handle authentication, request serialization, and response parsing.

*   **Image Generation**: `z_image_turbo`, `flux2_pro`, `nano_banana2`, `flux2_dev_i2i`, `flux2_klein_i2i`.
*   **Video Generation**: `ltx2_3a2v` (Image + Audio → Video).
*   **Audio/Text**: `qwen3_asr_flash` (Audio → Lyrics), `qwen3_6_35b_a3b` (Chat).

**Key Behavior**:
*   **Cancellation**: All API functions accept a `cancel_flag` pointer. If the byte at this address is set to `1` during execution, the function aborts and returns a fallback asset (embedded in the binary) instead of the result.
*   **Fallbacks**: On network errors or HTTP 402 (payment required), the Rust layer returns embedded fallback images/videos (`fallback.png`, `topup.jpg`, etc.) rather than empty data.
*   **Platform Split**:
    *   **Native (iOS/Android)**: Uses `tokio` runtime. The FFI function blocks the thread while the async task runs on the internal runtime.
    *   **WASM**: Uses `wasm-bindgen`. The function is `async` and returns a `Uint8Array` or `String`.

### 2. Project Service (`Rust/src/project_service/`)
Manages local file storage and XMP metadata embedding. Rust owns the "Documents" directory.

*   **Metadata**: Embeds Prompt, Model, and Subject tags into images/videos using XMP standards (IPTC Ext, DC, XMP namespaces).
*   **State**: Manages in-memory state for "Character Cast" and "Image Edit" contexts.
*   **Platform Split**:
    *   **Apple**: Uses `xmp_toolkit` via C ABI (`psxmp_*` functions).
    *   **Android**: Uses JNI (`Java_market_femi_api_ProjectServiceJvm_*`).
    *   **WASM**: Uses OPFS (Origin Private File System) via `wasm-bindgen`.

### 3. ID3 Processing (`Rust/src/id3/`)
Extracts synchronized lyrics (SYLT frames) from MP3 files.

*   **Native**: `id3_ffi_extract_sylt` returns a heap-allocated JSON string.
*   **Android**: JNI wrapper `Java_market_femi_api_Id3Jvm_extractSylt`.
*   **WASM**: `extract_sylt` returns a JSON string.

## Build Instructions

### Prerequisites
*   **Rust**: `rustup` installed.
*   **Android NDK**: Required for Android targets. Ensure `ANDROID_NDK_HOME` or `NDK_HOME` is set, or configure paths in `Rust/.cargo/config.toml`.
*   **Xcode**: Required for Apple targets (`xcodebuild`).
*   **Node.js**: Required for the WebAssembly inlining step.
*   **wasm-bindgen-cli**: Installed automatically by the build script if missing.

### Build Script
Run the main build script from the repository root:

```bash
./build-rust.sh
```

This script performs the following:
1.  Installs necessary Rust targets (`aarch64-apple-ios`, `aarch64-linux-android`, `wasm32-unknown-unknown`, etc.).
2.  **Apple**: Compiles static libraries and bundles them into `RustFFI.xcframework`.
3.  **Android**: Compiles `.so` files and places them in `Kmp/api/src/androidMain/jniLibs/<abi>/`.
4.  **WebAssembly**:
    *   Compiles to `.wasm`.
    *   Runs `wasm-bindgen` to generate JS glue.
    *   **Inlines** the `.wasm` binary into the JS file as Base64 (no external `.wasm` file needed).
    *   Generates `Kmp/api/src/webMain/kotlin/market/femi/api/RustFfiBundle.kt` containing the Base64-encoded JS bundle, allowing it to be shipped inside the KMP library without npm.

## Key Files & Paths

| Path | Description |
| :--- | :--- |
| `build-rust.sh` | Master build script for all platforms. |
| `Rust/src/lib.rs` | Rust crate root. Defines shared constants, fallback assets, and the `reqwest` client. |
| `Rust/include/RustFFI/RustFFI.h` | C Header file defining the public FFI API for Apple/Swift. |
| `RustFFI.xcframework/` | Output directory for the Apple binary target. |
| `Kmp/api/src/androidMain/jniLibs/` | Output directory for Android `.so` libraries. |
| `Rust/pkg/` | Output directory for WebAssembly JS/WASM glue. |
| `Sources/Api/` | Swift extensions wrapping the Rust FFI for iOS/macOS. |
| `Rust/src/project_service/android.rs` | JNI bindings for Android Project Service. |
| `Rust/src/project_service/wasm.rs` | `wasm-bindgen` bindings for Web Project Service. |

## Usage

### Swift (iOS/macOS)
Import the `Api` package. The Swift code in `Sources/Api/` provides high-level async functions.

```swift
import Api

// Generate an image
let image = await Api.flux2Pro(
    user: "username",
    password: "password",
    prompt: "a cat in space"
)

// Save to local documents with metadata
ProjectService.saveFile(image, named: "cat.png", prompt: "a cat in space", model: "flux2-pro")
```

### Kotlin (Android)
Load the library via `System.loadLibrary("rust_ffi")`. The JNI functions are exposed directly to Kotlin via the generated bindings in `FemiApiJvm`.

```kotlin
// Example usage via the generated JNI wrapper
val result = FemiApiJvm.rustFfiFlux2Pro(user, password, prompt, cancelFlag)
```

### Kotlin (Web/KMP)
The WebAssembly module is embedded in `RustFfiBundle.kt`. Initialize it at runtime.

```kotlin
// The bundle is decoded and instantiated automatically by the KMP wrapper
val result = RustFfi.wasmFlux2Pro(user, password, prompt)
```

## Testing

### Rust Tests
Run standard Cargo tests for the shared logic and API integration tests (requires network access and valid credentials).

```bash
cd Rust
cargo test
```

### Swift Tests
Run the Swift Package Manager tests. These require the `RustFFI.xcframework` to be built first.

```bash
swift test
```

## Non-Obvious Conventions

1.  **Memory Management**:
    *   **Native**: The Rust FFI functions allocate memory for the result (`Box::into_raw`). The caller (Swift/Kotlin) **must** free this memory using the provided `deallocator: .free` (Swift) or equivalent JNI byte array handling. The Rust side does *not* free it.
    *   **WASM**: Memory is managed by the WebAssembly linear memory; no explicit free is needed by the JS/Kotlin caller.

2.  **Cancellation**:
    *   The `cancel_flag` is a pointer to a single `u8`.
    *   **Native**: The Rust side polls this flag every 10ms. Setting it to `1` triggers an immediate abort.
    *   **Swift**: The `Api` extensions handle flag allocation and cleanup automatically.
    *   **Android**: The Kotlin side must manage the `jlong` pointer passed to the JNI function.

3.  **XMP Metadata**:
    *   The Rust layer uses a shared `xmpkit_body` module for Android/WASM (pure Rust bytes manipulation) and `xmp_toolkit` for Apple (native C library).
    *   Metadata is embedded *into* the file bytes before writing. Reading metadata requires parsing the file bytes.

4.  **WebAssembly Inlining**:
    *   The `build-rust.sh` script modifies the `wasm-bindgen` output to embed the `.wasm` binary as Base64 inside the JS file. This ensures the KMP Web target is a single artifact without external dependencies.