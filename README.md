# RustFFI

A cross-platform Rust FFI crate that exposes AI generation APIs, XMP metadata manipulation, and ID3 lyric extraction to Swift (iOS/macOS), Kotlin (Android), and WebAssembly.

## Overview

This project serves as the binary bridge for the `Api` Swift package and `kotlinapi` Kotlin module. It consolidates three distinct capabilities into a single Rust library (`librust_ffi`):

1.  **AI Generation Endpoints**: Async HTTP clients that call the `femi.market` API for image generation (Flux, ZImage, Nano), video generation (Ltx2), and text/audio processing (Qwen3).
2.  **ProjectService XMP FFI**: Embeds and reads XMP metadata (prompts, models, subjects, ratings) into image and video files. Uses the Adobe XMP Toolkit on Apple platforms and a pure-Rust `xmpkit` implementation on Android/Web.
3.  **ID3 SYLT Extraction**: Parses MP3 files to extract synchronized lyrics (SYLT frames) as JSON.

### Supported Platforms

| Platform | Output Artifact | Consumption Path |
| :--- | :--- | :--- |
| **iOS / macOS** | `RustFFI.xcframework` | Swift Package Manager (`Package.swift`) |
| **Android** | `librust_ffi.so` | `Kmp/kotlinapi/src/androidMain/jniLibs/` |
| **WebAssembly** | `pkg/` (JS/WASM) | `Rust/pkg/` (consumed via `build.gradle.kts`) |

## Architecture

The codebase is split into three main domains within `Rust/src/`:

### 1. API Endpoints (`Rust/src/api/`)
Each endpoint (e.g., `flux2_pro`, `qwen3_asr_flash`) follows a consistent pattern:
-   **Core Logic**: A `pub(crate) async fn core_...` function that builds the JSON payload, sends it via `reqwest`, and resolves the response (handling fallback images/videos for errors).
-   **Native FFI**: A `#[no_mangle] pub extern "C" fn` that handles string conversion, memory allocation, and cancellation flags. It returns a raw pointer to a heap-allocated byte slice.
-   **WASM Bindgen**: A `#[wasm_bindgen] pub async fn` that returns a `Uint8Array` or `String`.

**Cancellation**: All native FFI functions accept a `cancel_flag: *const u8`. If non-null, the Rust side polls this memory location. If the value becomes non-zero, the operation is aborted, and a fallback asset (image/video/text) is returned immediately.

### 2. ProjectService XMP (`Rust/src/project_service/`)
This module handles metadata embedding. It uses conditional compilation to switch backends:
-   **Apple (`apple.rs`)**: Uses the `xmp_toolkit` crate with smart handlers to support JPEG, PNG, TIFF, MP4, MOV, etc. Exposed via C ABI (`psxmp_*`).
-   **Android (`android.rs`)**: Uses `jni` to read/write files directly and delegates to `xmpkit_body` (pure Rust bytes-in/bytes-out logic). Exposed via JNI symbols.
-   **WebAssembly (`wasm.rs`)**: Uses the Origin Private File System (OPFS) via `web-sys` to read/write files. Exposed via `wasm_bindgen`.

### 3. ID3 Lyrics (`Rust/src/id3/`)
-   **Core**: `sylt.rs` parses MP3 bytes using the `id3` crate to extract synchronized lyrics.
-   **Native**: `id3_ffi_extract_sylt` returns a heap-allocated UTF-8 JSON string.
-   **Android**: JNI wrapper `Java_market_femi_kotlinapi_Id3Jvm_extractSylt`.
-   **WASM**: `extract_sylt` returns a JSON string.

## Build Instructions

### Prerequisites
-   **Rust**: `rustup` installed.
-   **Android NDK**: Required for Android targets. Ensure `ANDROID_NDK_HOME` or `NDK_HOME` is set, or that the NDK is installed via Android Studio.
-   **Xcode Command Line Tools**: Required for Apple targets (`xcodebuild`, `lipo`).
-   **wasm-bindgen-cli**: Installed automatically by the build script if missing.

### Build Script
Run the provided shell script to build for all platforms:

```bash
./build-rust.sh
```

This script:
1.  Installs necessary Rust targets (`aarch64-apple-ios`, `x86_64-apple-darwin`, `aarch64-linux-android`, `wasm32-unknown-unknown`, etc.).
2.  Builds static libraries for Apple, creates universal binaries, and packages them into `RustFFI.xcframework`.
3.  Builds shared libraries for Android and copies them to `Kmp/kotlinapi/src/androidMain/jniLibs/`.
4.  Builds the WASM target and runs `wasm-bindgen` to output JS bindings to `Rust/pkg/`.

### Manual Build Steps

#### Apple
```bash
cd Rust
rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios aarch64-apple-darwin x86_64-apple-darwin
export IPHONEOS_DEPLOYMENT_TARGET=14.0
export MACOSX_DEPLOYMENT_TARGET=11.0

# Build individual targets
cargo build --release --target aarch64-apple-ios
cargo build --release --target aarch64-apple-darwin
# ... (other targets)

# Create xcframework
xcodebuild -create-xcframework \
  -library target/aarch64-apple-ios/release/librust_ffi.a -headers include \
  -library target/ios-sim-universal/release/librust_ffi.a -headers include \
  -library target/macos-universal/release/librust_ffi.a -headers include \
  -output ../RustFFI.xcframework
```

#### Android
```bash
cd Rust
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android

for triple in aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android; do
  cargo build --release --target $triple
  mkdir -p ../Kmp/kotlinapi/src/androidMain/jniLibs/$(echo $triple | cut -d- -f1)
  cp target/$triple/release/librust_ffi.so ../Kmp/kotlinapi/src/androidMain/jniLibs/$(echo $triple | cut -d- -f1)/librust_ffi.so
done
```

#### WebAssembly
```bash
cd Rust
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/rust_ffi.wasm --out-dir pkg --target web
```

## Usage

### Swift (iOS/macOS)
Import the `Api` package. The Rust FFI is linked as a binary target.

```swift
import Api

// Generate an image
let imageData = await Api.flux2Pro(
    user: "my_user",
    password: "my_pass",
    prompt: "a cat in space"
)

// Embed XMP metadata
ProjectService.saveFile(
    imageData,
    named: "cat.png",
    prompt: "a cat in space",
    model: "flux-pro",
    subject: ["cat", "space"]
)
```

### Kotlin (Android)
The Rust library is loaded via `System.loadLibrary("rust_ffi")`. The Kotlin side calls JNI functions generated by the `jni_api.rs` macros.

```kotlin
import market.femi.kotlinapi.FemiApiJvm

// Generate an image
val bytes = FemiApiJvm.rustFfiFlux2Pro(
    user = "my_user",
    password = "my_pass",
    prompt = "a cat in space",
    cancelFlag = 0L
)

// Extract lyrics from MP3
import market.femi.kotlinapi.Id3Jvm
val lyricsJson = Id3Jvm.extractSylt(mp3Bytes)
```

### WebAssembly
Import the generated JS bindings from `pkg/rust_ffi.js`.

```javascript
import init, { wasm_flux2_pro } from './pkg/rust_ffi.js';

await init();
const bytes = await wasm_flux2_pro("user", "pass", "prompt");
```

## Key Files

-   `build-rust.sh`: Master build script for all platforms.
-   `Rust/src/lib.rs`: Entry point, defines fallback assets, global HTTP client, and runtime.
-   `Rust/src/api/mod.rs`: Module structure for API endpoints.
-   `Rust/src/project_service/mod.rs`: Module structure for XMP FFI.
-   `Rust/src/id3/mod.rs`: Module structure for ID3 parsing.
-   `Package.swift`: Swift Package definition, links `RustFFI.xcframework`.
-   `RustFFI.xcframework/`: Pre-built framework for Apple platforms.

## Testing

### Rust Tests
Run native Rust tests (Apple/Android host):
```bash
cd Rust
cargo test
```

### Swift Tests
Run Swift tests via Xcode or `swift test`:
```bash
swift test
```

### Kotlin Tests
Run Android instrumentation tests or unit tests via Gradle:
```bash
./gradlew :kotlinapi:testDebugUnitTest
```