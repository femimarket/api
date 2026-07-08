# Femi API

A cross-platform, multi-language library that provides a unified interface to AI media generation endpoints (image, video, audio transcription, LLM chat) and local project management with XMP metadata storage. The core logic is written in Rust and compiled to native binaries for iOS/macOS and Android, and to WebAssembly for the web. Kotlin Multiplatform (KMP) and Swift act as platform-specific wrappers.

## Architecture & Platform Support

| Platform | Language | FFI / Binding | Storage Backend | Metadata Engine |
|----------|----------|---------------|-----------------|-----------------|
| **iOS / macOS** | Swift | `RustFFI.xcframework` (C ABI) | `dirs::document_dir()` | `xmp_toolkit` (smart handlers) |
| **Android** | Kotlin | JNI (`librust_ffi.so`) | `context.filesDir` (via `androidx.startup`) | `xmpkit` (bytes-in/bytes-out) |
| **Web** | Kotlin/Wasm | `wasm-bindgen` (JS interop) | OPFS (per-origin flat namespace) | `xmpkit_body` (pure Rust) |
| **Common** | Kotlin | `expect` declarations | N/A | N/A |

The Rust crate (`Rust/`) is the single source of truth. It shares async HTTP clients, AI endpoint logic, and cancellation handling across all platforms. Platform-specific code paths are gated by `#[cfg(target_...)]` and `#[cfg(target_arch = "wasm32")]`.

## Build & Installation

### Prerequisites
- Rust toolchain (`rustup`, `cargo`)
- `wasm-bindgen-cli` (installed automatically by the build script if missing)
- Android NDK & `ANDROID_HOME` (for Android targets)
- Xcode / Apple SDKs (for iOS/macOS targets)
- Node.js (for WASM post-processing)

### One-Command Build
Run the master build script from the repository root:
```bash
./build-rust.sh
```
This script:
1. Installs required `rustup` targets (`aarch64-apple-ios`, `aarch64-apple-ios-sim`, `aarch64-apple-darwin`, `wasm32-unknown-unknown`, and 4 Android ABIs).
2. Compiles release binaries for each target.
3. Packages Apple targets into `RustFFI.xcframework`.
4. Copies Android `.so` files into `Kmp/api/src/androidMain/jniLibs/<abi>/`.
5. Runs `wasm-bindgen` on the WASM output, then inlines the `.wasm` into the JS glue as base64, and finally embeds the JS as base64 into `Kmp/api/src/webMain/kotlin/market/femi/api/RustFfiBundle.kt`.

No manual Gradle, SPM, or npm steps are required for the FFI layer. The consumer project simply references the generated artifacts.

## Usage

### Kotlin Multiplatform
Define your API calls using the `expect` functions in `Kmp/api/src/commonMain/kotlin/market/femi/api/`. The `actual` implementations handle platform-specific FFI marshaling, cancellation, and async execution.

```kotlin
// commonMain
expect suspend fun flux2Pro(user: String, pass: String, prompt: String): ByteArray

// androidMain / webMain
actual suspend fun flux2Pro(...) = runCancelable { addr ->
    FemiApiJvm.rustFfiFlux2Pro(user, pass, prompt, addr)
}
```

### Swift (Apple)
Import the SPM package and call async methods on the `Api` or `ProjectService` enums.

```swift
import Api

let image = await Api.flux2Pro(user: "user", password: "pass", prompt: "a red apple")
ProjectService.saveFile(image, named: "apple.png", prompt: "a red apple")
```

### Rust
Use as a library or run integration tests against a live server:
```bash
cd Rust
cargo test
```
Tests auto-generate a funded username per run (`funded-test-<uuid>`) and use a static password (`abc123`).

## Key Files & Directory Structure

- `build-rust.sh` — Master build script. Handles cross-compilation, xcframework packaging, JNI distribution, and WASM inlining.
- `Rust/src/lib.rs` — Crate entry point. Sets up `tokio` runtime, `reqwest` client, and defines fallback/topup assets.
- `Rust/src/api/*.rs` — Endpoint implementations (`z_image_turbo.rs`, `flux2_pro.rs`, `ltx2_3a2v.rs`, etc.). Each contains `core_*` async logic and platform-specific `native`/`wasm` FFI wrappers.
- `Rust/src/project_service/*.rs` — File I/O and XMP metadata handling. `shared/xmpkit_body.rs` contains the pure-Rust bytes-in/bytes-out logic used by Android and WASM. `apple.rs` uses `xmp_toolkit`. `android.rs` uses JNI. `wasm.rs` uses OPFS.
- `Kmp/api/src/commonMain/kotlin/market/femi/api/ProjectService.kt` — KMP API contract. Defines suspend functions for file saving, metadata retrieval, and in-memory state.
- `Kmp/api/src/webMain/kotlin/market/femi/api/RustFfi.kt` — WASM module loader. Decodes `RUST_FFI_JS_B64`, creates a Blob URL, and dynamically imports the module. Caches the result.
- `Kmp/api/src/androidMain/kotlin/market/femi/api/ProjectServiceInitializer.kt` — `androidx.startup.Initializer` that calls `ProjectService.initDocuments()` with `context.filesDir` at process start.
- `Sources/Api/ProjectService.swift` — Swift FFI wrapper. Marshals arguments to C ABI and handles buffer sizing for string returns.
- `Package.swift` — SPM manifest. Declares `RustFFI.xcframework` as a binary target and `Api` as the Swift wrapper.

## Non-Obvious Conventions

### Automatic Cancellation
All AI endpoint FFI calls accept a `cancel_flag: *const u8` pointer. Kotlin allocates a 1-byte direct `ByteBuffer`, extracts its native address via reflection (`Buffer.address`), and passes it. On coroutine cancellation, the flag is flipped to `1`. Rust polls the flag via `AtomicU8::load` inside a `tokio::select!` block. If flipped, the call aborts immediately and returns an embedded fallback asset (image/video/text) instead of waiting for the network.

### Self-Contained WASM Bundle
The web build produces zero external assets. `build-rust.sh` strips `pkg/rust_ffi_bg.wasm` after embedding it as base64 into `pkg/rust_ffi.js`. That JS file is then base64-encoded into a Kotlin constant (`RUST_FFI_JS_B64`). At runtime, `RustFfi.kt` decodes the constant, creates a `Blob` URL, and uses `new Function('u', 'return import(u);')` to dynamically import it. This bypasses webpack/static bundler rewrite issues and ensures the module ships entirely inside the published klib.

### XMP Metadata Namespaces
Metadata is written to standard XMP namespaces:
- `dc:description` / `iptc4xmpext:AIPromptInformation` → Prompt
- `xmp:CreatorTool` / `iptc4xmpext:AISystemUsed` → Model
- `xmpDM:projectName`, `xmpDM:lyrics`, `xmpDM:shotNumber` → Project metadata
- `dc:subject` → Subject tags
- `xmp:Rating` → Like/unlike state (0 or 5)

Apple uses Adobe's `xmp_toolkit` smart handlers for read/write. Android and WASM use a pure-Rust `xmpkit_body` module that operates directly on file bytes, avoiding platform-specific native libraries.

### ProjectService Root Resolution
Rust owns the `Documents/` root. The path is resolved differently per platform:
- **Apple**: `dirs::document_dir()` at runtime.
- **Android**: `ProjectServiceInitializer` passes `context.filesDir.absolutePath` once at startup via `psxmpInitDocuments()`.
- **Web**: OPFS provides a flat, per-origin namespace. Filenames are used directly as keys.

### C ABI String Handling (Apple)
Apple string-returning functions use a two-phase query pattern. The first call passes a `null` buffer and `0` length to request the required byte count. The caller allocates exactly that size + 1, then calls again to fill it. This prevents truncation for arbitrarily long metadata strings.

## Testing

- **Rust**: `cargo test` in `Rust/` runs integration tests against the live `femi.market` API. Tests verify endpoint responses, fallback behavior, and XMP metadata embedding (`Rust/tests/xmpkit_body.rs`).
- **Swift**: `Tests/ApiTests/` contains Xcode/SPM tests covering all endpoints, cancellation timing (<1s), and the full `ProjectService` workflow (save, read, like, audio handling, character cast, image edit).
- **Android/Web**: Tested via the KMP consumer application. Android uses the emulator/device runtime; Web uses a browser with OPFS support.