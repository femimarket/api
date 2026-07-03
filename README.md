# Rust FFI for Femi API

A cross-platform Rust library providing a C-compatible FFI layer for the Femi API. It exposes AI generation endpoints (image, video, text, audio), a local Project Service for XMP metadata management, and ID3 lyric extraction.

The library is compiled into three distinct artifacts to support the project's multi-platform architecture:
1.  **`RustFFI.xcframework`**: For Apple platforms (iOS/macOS), consumed via Swift Package Manager.
2.  **`jniLibs`**: For Android, consumed via `System.loadLibrary("rust_ffi")`.
3.  **`pkg/`**: For WebAssembly (WASM), consumed via Kotlin/JS in the KMP module.

## Architecture

The codebase is split into three main domains, each with platform-specific bindings:

### 1. API Endpoints (`Rust/src/api/`)
These functions call the remote Femi server (`https://femi.market/api`). They handle authentication, JSON serialization, and response parsing.
*   **Cancellation**: All API functions accept a `cancel_flag` pointer. If the byte at this address is non-zero, the Rust side cancels the request and returns a fallback asset (image/video/text) immediately.
*   **Fallbacks**: Embedded assets (e.g., `fallback.png`, `topup.jpg`) are included in the binary to handle errors or cancellations gracefully.

**Supported Endpoints:**
*   `z_image_turbo`: Text-to-Image
*   `flux2_pro`: Text-to-Image
*   `flux2_dev_i2i`: Image-to-Image (Single Input)
*   `flux2_klein_i2i`: Image-to-Image (Dual Input)
*   `nano_banana2`: Text-to-Image
*   `ltx2_3a2v`: Image+Audio-to-Video
*   `qwen3_6_35b_a3b`: Text Chat (LLM)
*   `qwen3_asr_flash`: Audio-to-Text (Lyrics)

### 2. Project Service (`Rust/src/project_service/`)
A local file management system that owns the Documents directory. It handles saving files, embedding XMP metadata (prompt, model, subjects), and managing in-memory state (character cast, image edit targets).
*   **Apple**: Uses `xmp_toolkit` for smart XMP handling.
*   **Android/WASM**: Uses a pure-Rust bytes-based XMP library (`xmpkit_body`) to avoid native dependencies.
*   **WASM**: Uses the Origin Private File System (OPFS).

### 3. ID3 Extraction (`Rust/src/id3/`)
Extracts synchronized lyrics (SYLT frames) from MP3 files.
*   **Android**: JNI entry point `Java_market_femi_api_Id3Jvm_extractSylt`.
*   **Apple/WASM**: C/WASM bindings `id3_ffi_extract_sylt` / `extract_sylt`.

## Build Instructions

### Prerequisites
*   **Rust**: `rustup` installed.
*   **Android NDK**: Required for Android targets.
*   **Xcode**: Required for Apple targets (`xcodebuild`).
*   **Node.js**: Required for WASM inlining.
*   **wasm-bindgen-cli**: Installed automatically if missing.

### Build Script
Run the main build script from the repository root:

```bash
./build-rust.sh
```

This script performs the following steps:
1.  Installs necessary Rust targets (`aarch64-apple-ios`, `aarch64-apple-darwin`, `wasm32-unknown-unknown`, and Android ABIs).
2.  **Apple**: Compiles static libraries and bundles them into `RustFFI.xcframework`.
3.  **Android**: Compiles `.so` libraries and places them in `Kmp/api/src/androidMain/jniLibs/<abi>/`.
4.  **WASM**: Compiles to `.wasm`, runs `wasm-bindgen`, and inlines the WASM binary into the generated JS glue code as base64. It also generates `RustFfiBundle.kt` containing the base64-encoded JS module for KMP consumption.

### Manual Build Steps

If you need to rebuild only specific parts:

#### Apple
```bash
cd Rust
cargo build --release --target aarch64-apple-ios
cargo build --release --target aarch64-apple-ios-sim
cargo build --release --target aarch64-apple-darwin

xcodebuild -create-xcframework \
  -library target/aarch64-apple-ios/release/librust_ffi.a -headers include \
  -library target/aarch64-apple-ios-sim/release/librust_ffi.a -headers include \
  -library target/aarch64-apple-darwin/release/librust_ffi.a -headers include \
  -output ../RustFFI.xcframework
```

#### Android
```bash
cd Rust
# Build for each ABI
cargo build --release --target aarch64-linux-android
cargo build --release --target armv7-linux-androideabi
cargo build --release --target x86_64-linux-android
cargo build --release --target i686-linux-android

# Copy to jniLibs structure
mkdir -p Kmp/api/src/androidMain/jniLibs/arm64-v8a
cp target/aarch64-linux-android/release/librust_ffi.so Kmp/api/src/androidMain/jniLibs/arm64-v8a/
# ... repeat for other ABIs
```

#### WebAssembly
```bash
cd Rust
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/rust_ffi.wasm --out-dir pkg --target web

# Inline WASM into JS (requires Node.js)
node - pkg/rust_ffi.js pkg/rust_ffi_bg.wasm <<'NODE'
const fs = require('fs');
const [, , jsPath, wasmPath] = process.argv;
const b64 = fs.readFileSync(wasmPath).toString('base64');
let src = fs.readFileSync(jsPath, 'utf8');
const needle = /new URL\(['"]rust_ffi_bg\.wasm['"], import\.meta\.url\)/;
if (!needle.test(src)) {
  console.error('inline-wasm: could not find the wasm URL in the glue');
  process.exit(1);
}
const prelude =
  `const __WASM_B64 = "${b64}";\n` +
  `function __wasmBytes(){const s=atob(__WASM_B64),u=new Uint8Array(s.length);` +
  `for(let i=0;i<s.length;i++)u[i]=s.charCodeAt(i);return u;}\n`;
src = prelude + src.replace(needle, '__wasmBytes()');
fs.writeFileSync(jsPath, src);
NODE
```

## Usage

### Swift (Apple)
Import the `Api` package. The `Api` enum exposes async methods for each endpoint.

```swift
import Api

// Image Generation
let image = await Api.flux2Pro(
    user: "username",
    password: "password",
    prompt: "A red apple"
)

// Project Service
ProjectService.saveFile(imageData, named: "apple.png", prompt: "A red apple")
let prompt = ProjectService.getPrompt("apple.png")
```

### Kotlin/Android
The Rust library is loaded as a native library. Access via generated Kotlin wrappers (e.g., `FemiApiJvm`).

```kotlin
// Example usage pattern (actual wrapper code is in Kmp module)
val result = FemiApiJvm.rustFfiFlux2Pro(user, password, prompt, cancelFlag)
```

### Kotlin/WASM
The WASM module is embedded in the KMP library. Initialize via `RustFfi.kt` which decodes the embedded base64 JS module.

```kotlin
// Initialization happens automatically via RustFfiBundle
val result = RustFfi.wasmFlux2Pro(user, password, prompt)
```

## Key Files

*   `build-rust.sh`: Master build script.
*   `Rust/src/lib.rs`: Root Rust module, defines fallback assets and shared client logic.
*   `Rust/include/RustFFI/RustFFI.h`: C header defining the FFI API.
*   `Sources/Api/*.swift`: Swift wrappers for the FFI.
*   `Rust/src/project_service/`: Platform-specific implementations of the local file service.
*   `Rust/src/api/`: Platform-specific implementations of the remote API calls.

## Testing

### Rust Tests
Run standard Cargo tests:
```bash
cd Rust
cargo test
```
Tests cover API endpoints (using a test user), XMP metadata embedding, and ID3 extraction.

### Swift Tests
Run via Xcode or `swift test`. Tests use `Bundle.module` resources for fallback assets and verify cancellation behavior.

## Conventions

*   **Memory Management**: FFI functions return `*mut u8` with a length. The caller is responsible for freeing the memory (using `free()` in C/Swift, or `Box::from_raw` in Rust).
*   **Cancellation**: A `cancel_flag` is a pointer to a `u8`. Setting it to `1` signals cancellation. The Rust side polls this flag every 10ms.
*   **XMP Metadata**:
    *   `dc:description`: Prompt
    *   `xmp:CreatorTool`: Model
    *   `dc:subject`: Subject list
    *   `xmp:Rating`: Like status (1-5)
    *   `Iptc4xmpExt:AIPromptInformation`: Prompt (duplicate)
    *   `Iptc4xmpExt:AISystemUsed`: Model (duplicate)