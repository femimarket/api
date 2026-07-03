# Rust FFI Bridge

A cross-platform Rust library providing a unified FFI (Foreign Function Interface) for the **Femi** application. It exposes AI generation endpoints (image, video, audio, LLM) and a local Project Service (XMP metadata management) to Swift (iOS/macOS), Kotlin (Android), and WebAssembly (Web).

## Overview

This crate compiles to native libraries for Apple platforms and Android, and a WebAssembly module for the web. It serves as the backend for the `Api` Swift package and the `Kmp` Kotlin Multiplatform module.

### Key Features
- **Multi-Platform Support**: Builds for `aarch64-apple-ios`, `aarch64-apple-darwin`, `aarch64-linux-android`, and `wasm32-unknown-unknown`.
- **AI Endpoints**: Wraps HTTP calls to `https://femi.market/api` for Flux, ZImage, Qwen, and LTX models.
- **Project Service**: Manages local file storage and XMP metadata embedding (prompt, model, subject, likes) using `xmp_toolkit` on Apple and a pure-Rust byte-based implementation on Android/Web.
- **Cancellation**: Supports cooperative cancellation via atomic flags for all long-running API calls.
- **Self-Contained Web**: The WebAssembly output is inlined into a single JavaScript module and embedded as a base64 Kotlin constant for distribution without external assets.

## Architecture

The project is structured into three main layers:

1.  **Rust Core (`Rust/`)**: The source of truth. Contains the FFI definitions, API clients, and platform-specific backends.
2.  **Swift Package (`Sources/Api/`)**: A Swift Package that consumes the `RustFFI.xcframework`. It provides a clean Swift API (`Api.flux2Pro`, `ProjectService.saveFile`, etc.) over the C ABI.
3.  **Kotlin Multiplatform (`Kmp/`)**: Consumes the Android `.so` files and the inlined WebAssembly bundle.

### Platform Backends

| Platform | Backend Implementation | File I/O | Metadata Library |
| :--- | :--- | :--- | :--- |
| **Apple** | `Rust/src/project_service/apple.rs` | `std::fs` (Documents dir) | `xmp_toolkit` (Smart Handler) |
| **Android** | `Rust/src/project_service/android.rs` | `std::fs` (Context files dir) | `xmpkit` (Byte-based) |
| **Web** | `Rust/src/project_service/wasm.rs` | OPFS (Origin Private File System) | `xmpkit` (Byte-based) |

## Installation & Build

### Prerequisites

- **Rust**: `rustup` installed.
- **Targets**:
  ```bash
  rustup target add aarch64-apple-ios aarch64-apple-ios-sim aarch64-apple-darwin wasm32-unknown-unknown
  rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android
  ```
- **Android NDK**: Ensure `ANDROID_NDK_HOME` or `NDK_HOME` is set, or that the NDK is installed via Android Studio. The build script reads config from `Rust/.cargo/config.toml`.
- **Wasm-bindgen**: `cargo install wasm-bindgen-cli` (installed automatically if missing).
- **Xcode**: Required for creating the `xcframework`.

### Build Script

Run the build script from the repository root:

```bash
./build-rust.sh
```

This script performs the following:
1.  Installs missing Rust targets and `wasm-bindgen-cli`.
2.  Compiles the library for all Apple, Android, and Web targets.
3.  **Apple**: Creates `RustFFI.xcframework` containing static libraries for iOS (device/simulator) and macOS.
4.  **Android**: Copies `.so` files to `Kmp/api/src/androidMain/jniLibs/<abi>/`.
5.  **Web**:
    -   Runs `wasm-bindgen` on the `.wasm` output.
    -   Inlines the `.wasm` binary into `pkg/rust_ffi.js` as base64.
    -   Base64-encodes the resulting JS glue code and writes it to `Kmp/api/src/webMain/kotlin/market/femi/api/RustFfiBundle.kt`.

### Output Artifacts

-   `RustFFI.xcframework`: Used by the Swift Package.
-   `Kmp/api/src/androidMain/jniLibs/`: Used by the Android Gradle build.
-   `Kmp/api/src/webMain/kotlin/market/femi/api/RustFfiBundle.kt`: Used by the Kotlin Web module.

## API Reference

### C FFI (`Rust/include/RustFFI/RustFFI.h`)

The C header defines the public interface. All functions return `uint8_t*` (heap-allocated bytes) or `void`/`int32_t` for status codes.

#### AI Endpoints
All AI functions take `user`, `password`, and a `cancel_flag` (pointer to an atomic `u8`; `0` = no cancel, `1` = cancel). They return a pointer to the result bytes and set `out_len`.

-   `rust_ffi_z_image_turbo`: Text-to-Image.
-   `rust_ffi_flux2_pro`: Text-to-Image (Flux Pro).
-   `rust_ffi_flux2_dev_i2i`: Image-to-Image (Flux Dev).
-   `rust_ffi_flux2_klein_i2i`: Image-to-Image (Flux Klein, dual image).
-   `rust_ffi_nano_banana2`: Text-to-Image (Nano Banana).
-   `rust_ffi_ltx2_3a2v`: Image+Audio-to-Video.
-   `rust_ffi_qwen3_asr_flash`: Audio-to-Text (Lyrics).
-   `rust_ffi_qwen3_6_35b_a3b`: Text-to-Text (LLM Chat).

#### Project Service (XMP)
Functions prefixed with `psxmp_` manage local files. Rust owns the Documents directory.

-   `psxmp_save_file`: Save file with optional XMP metadata (prompt, model, subjects).
-   `psxmp_save_audio`: Save audio file (replaces any existing audio).
-   `psxmp_like`: Set/unset "like" status (writes XMP Rating).
-   `psxmp_get_*`: Retrieve metadata (prompt, model, subject, like status, URL).
-   `psxmp_get_all_generations`: List all files in Documents.
-   `psxmp_set_character_cast` / `psxmp_get_character_cast`: In-memory state for character casting.
-   `psxmp_set_image_edit` / `psxmp_get_image_edit`: In-memory state for image editing context.

#### ID3 SYLT
-   `id3_ffi_extract_sylt`: Extract synchronized lyrics from MP3 bytes. Returns JSON array.

### Swift API (`Sources/Api/`)

The Swift package wraps the C FFI into async/await functions.

```swift
import Api

// Image Generation
let image = await Api.flux2Pro(user: "user", password: "pass", prompt: "cat")

// Project Service
ProjectService.saveFile(data, named: "image.png", prompt: "my prompt")
let prompt = ProjectService.getPrompt("image.png")
```

### Kotlin API (`Kmp/`)

Kotlin uses JNI for Android and `RustFfiBundle` for Web.

```kotlin
// Android
val result = FemiApiJvm.rustFfiFlux2Pro(user, password, prompt, cancelFlag)

// Web
val jsBundle = RustFfiBundle.RUST_FFI_JS_B64
// Decoded and initialized at runtime
```

## Key Files

-   `build-rust.sh`: The master build script.
-   `Rust/src/lib.rs`: Entry point, defines fallback assets and shared client logic.
-   `Rust/src/api/`: Individual endpoint implementations (e.g., `flux2_pro.rs`). Each file contains `native` (C FFI) and `wasm` (JS bindings) modules.
-   `Rust/src/project_service/`: Platform-specific file/XMP handling.
    -   `shared/xmpkit_body.rs`: Pure Rust XMP logic used by Android and Web.
    -   `apple.rs`: Uses `xmp_toolkit` for Apple.
    -   `android.rs`: JNI wrappers for Android.
    -   `wasm.rs`: OPFS wrappers for Web.
-   `Package.swift`: Swift Package manifest, references `RustFFI.xcframework`.

## Testing

### Rust Tests
Run with `cargo test` inside `Rust/`. Tests require a funded test account on the server.
-   `Rust/tests/api_tests.rs`: Shared helpers.
-   `Rust/tests/xmpkit_body.rs`: Unit tests for the shared XMP logic.

### Swift Tests
Run in Xcode or via `swift test`.
-   `Tests/ApiTests/`: Integration tests for the Swift API.
-   Uses `TestAssets` for fallback images/videos.

## Non-Obvious Conventions

1.  **Memory Management**: FFI functions return heap-allocated `uint8_t*`. The caller **must** free this memory using `free()` (C) or the equivalent in Swift/Kotlin. The Swift wrapper uses `Data(bytesNoCopy:count:deallocator:)` to handle this automatically.
2.  **Cancellation**: The `cancel_flag` is a pointer to an `AtomicU8`. The Rust side polls this flag every 10ms. Setting the byte to `1` triggers a graceful shutdown, returning a fallback asset (image/video/text) instead of the result.
3.  **WebAssembly Inlining**: The Web build process modifies the `wasm-bindgen` output to embed the `.wasm` binary directly into the JS file as base64. This eliminates the need for a separate `.wasm` file download at runtime.
4.  **XMP Namespaces**: The project uses specific namespaces for AI metadata:
    -   `dc:description`: Prompt text.
    -   `iptc4xmpext:AIPromptInformation`: Prompt text (duplicate).
    -   `xmp:CreatorTool`: Model name.
    -   `iptc4xmpext:AISystemUsed`: Model name (duplicate).
    -   `dc:subject`: Subject tags.
    -   `xmp:Rating`: Like status (1-5 for liked, 0 for unliked).