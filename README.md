# Api

## Overview
A cross-platform API client library for Femi Market's AI generation services. The project wraps a Rust FFI crate to provide synchronous/async endpoints for image generation, video generation, AI chat, audio transcription, and local file management with XMP metadata. It targets iOS, macOS, Android, and Web (Kotlin/WASM) through a unified `expect`/`actual` Kotlin API and native Swift wrappers.

## Architecture & Platform Support
| Platform | FFI Binding | Runtime | Key Entry Points |
|----------|-------------|---------|------------------|
| iOS/macOS | C ABI (`RustFFI.xcframework`) | `libdispatch` / Swift Concurrency | `Sources/Api/*.swift` |
| Android | JNI (`librust_ffi.so`) | Kotlin Coroutines | `Kmp/api/src/androidMain/...` |
| Web (WASM) | `wasm-bindgen` | Kotlin/Wasm JS Interop | `Kmp/api/src/webMain/...` |

The Rust core (`Rust/`) is compiled once per target. It shares a common async HTTP client, cancellation polling logic, and fallback assets, while delegating platform-specific I/O and metadata handling to platform-specific modules.

## Key Components & File Layout
- `build-rust.sh` — Master build script. Compiles Rust for all targets, generates `RustFFI.xcframework`, places Android `.so` files, inlines WASM, and embeds the JS bundle into Kotlin.
- `Rust/src/lib.rs` — Rust entry point. Configures `reqwest` client, defines fallback assets (`include_bytes!`), and exposes `client()` / `rt()` (Tokio runtime).
- `Rust/src/api/*.rs` — Endpoint implementations. Each file contains a `core_*` async function, a `native` module (C ABI + cancellation polling), and a `wasm` module (`wasm_bindgen`).
- `Rust/src/project_service/*.rs` — Cross-platform file storage. `shared/xmpkit_body.rs` handles bytes-in/bytes-out XMP mutation. `apple.rs` uses `xmp_toolkit`, while `android.rs` and `wasm.rs` use the shared bytes-based logic.
- `Kmp/api/src/commonMain/kotlin/market/femi/api/` — Kotlin `expect` declarations for all API endpoints and `ProjectService`.
- `Kmp/api/src/androidMain/kotlin/market/femi/api/` — Kotlin `actual` implementations. Uses `FemiApiJvm` JNI bridge and `runCancelable` for coroutine cancellation.
- `Kmp/api/src/webMain/kotlin/market/femi/api/` — Kotlin `actual` implementations. Uses inline WASM JS bundle and `kotlinx.coroutines.await`.
- `Sources/Api/*.swift` — Swift wrappers over `RustFFI.xcframework`. Use `withTaskCancellationHandler` for Swift concurrency.
- `Package.swift` — Swift Package Manager manifest. Declares `RustFFI.xcframework` as a binary target.

## Building & Installation
### Prerequisites
- Rust toolchain (`rustup`)
- `wasm-bindgen-cli` (installed automatically if missing)
- Android NDK & `cargo ndk` (configured in `Rust/.cargo/config.toml`)
- Xcode / `xcodebuild` (for Apple targets)
- Node.js (for WASM inlining step)

### Build Command
Run from the repository root:
```bash
./build-rust.sh
```

### Outputs
| Output | Path | Consumed By |
|--------|------|-------------|
| Apple Framework | `RustFFI.xcframework` | Swift Package Manager / Xcode |
| Android Libraries | `Kmp/api/src/androidMain/jniLibs/*` | Gradle (`System.loadLibrary("rust_ffi")`) |
| WebAssembly Bundle | `pkg/rust_ffi.js` (inlined) | Kotlin/Wasm via `RustFfiBundle.kt` |
| Kotlin WASM Constant | `Kmp/api/src/webMain/kotlin/market/femi/api/RustFfiBundle.kt` | Web runtime (base64 JS) |

## Usage
### Kotlin Multiplatform
Call the `expect` functions directly. They are suspended and handle platform-specific FFI marshaling:
```kotlin
val imageBytes = zImageTurbo(user = "me", pass = "secret", prompt = "a cyberpunk city")
ProjectService.saveFile(imageBytes, named = "city.png", prompt = "a cyberpunk city", model = "z-turbo")
```

### Swift
Use the `Api` enum and `ProjectService` singleton:
```swift
let image = await Api.zImageTurbo(user: "me", password: "secret", prompt: "a cyberpunk city")
ProjectService.saveFile(image, named: "city.png", prompt: "a cyberpunk city", model: "z-turbo")
```

### Rust (Direct FFI)
Call functions from `Rust/include/RustFFI/RustFFI.h`. All API functions return `uint8_t*` and write the length to `size_t*`. The caller must free the pointer via `Box::from_raw` (Rust) or `free()` (C).

## Cancellation & Error Handling
All generation endpoints support cooperative cancellation via a shared memory flag:
- **Kotlin**: `runCancelable` allocates a 1-byte `ByteBuffer.allocateDirect(1)`, extracts its native address via reflection, and passes it to Rust. On coroutine cancellation, the byte is flipped to `1`.
- **Swift**: `withTaskCancellationHandler` flips the flag in the `onCancel` closure.
- **Rust**: Each `native` module spawns a `tokio::select!` loop that polls `AtomicU8::from_ptr(cancel_flag)`. If the flag becomes `1`, the request aborts and returns the embedded fallback asset (e.g., `FALLBACK_IMAGE`, `FALLBACK_VIDEO`).

Network or parsing failures also return fallback assets. The Rust `resolve_image` / `resolve_video` functions map HTTP `402` to topup assets and other errors to fallback assets.

## Local Storage & XMP Metadata
`ProjectService` manages a sandboxed `Documents/` directory. Rust owns the root path:
- **Apple**: `dirs::document_dir()`
- **Android**: `context.filesDir` (auto-initialized via `ProjectServiceInitializer` using `androidx.startup`)
- **Web**: Origin-scoped OPFS (`navigator.storage.getDirectory()`)

Every call takes a filename (not a path). Metadata (prompt, model, subjects, rating) is embedded directly into the media bytes using XMP:
- Apple: `xmp_toolkit` smart handlers
- Android/Web: `xmpkit_body` (pure Rust bytes-in/bytes-out)

Audio files are stored separately; saving a new audio file automatically deletes any existing audio in the directory.

## WebAssembly / Web Support
The WASM build is fully self-contained:
1. `cargo build --target wasm32-unknown-unknown` produces `rust_ffi.wasm`.
2. `wasm-bindgen` generates glue JS.
3. `build-rust.sh` inlines the `.wasm` as base64 into `pkg/rust_ffi.js`.
4. The JS file is base64-encoded and written to `RustFfiBundle.kt`.
5. At runtime, `rustFfi()` decodes the base64, creates a Blob URL, and dynamically imports it. No external `.wasm` file, no npm dependency, and no runtime `fetch()` is required.

## Testing
- **Rust**: `cargo test` in `Rust/`. Uses live server endpoints with auto-funded test accounts (`testUser` / `testPassword`). Tests cover real API responses, cancellation fallbacks, and XMP metadata embedding (`Rust/tests/`).
- **Swift**: XCTest/Testing framework in `Tests/ApiTests/`. Validates generation, cancellation timing (<1s), and `ProjectService` I/O/metadata behavior.
- **Web/Android**: Platform-specific runtime tests are run via browser emulators and Android emulators respectively, as they require OPFS or JNI contexts.

## Notes & Conventions
- All string arguments are passed as C strings (`*const c_char`). Null pointers are treated as empty strings.
- FFI functions that return strings use output buffers (`buf`, `buf_len`) and return the written length. Swift/Kotlin handle buffer allocation and decoding.
- The `ProjectService` character cast and image edit state is process-lifetime in-memory state (`Mutex<Option<...>>` in Rust).
- Android's `ProjectServiceInitializer` is registered in the library's manifest via `androidx.startup`. Consumer apps do not need to call `initDocuments()` unless they want a custom root.
- WASM JS interop avoids `@JsModule` and npm by using `js()` inline helpers and `@JsFun` glue for `mediabunny` and OPFS operations.