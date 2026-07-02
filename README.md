# RustFFI

A cross-platform Rust FFI crate that provides a unified interface for AI media generation, transcription, and local file metadata management. It compiles to native libraries for iOS/macOS (via Swift Package Manager), Android (via JNI), and WebAssembly (via `wasm-bindgen`).

## Overview

This project serves as the backend engine for the **Api** Swift package and the **Kotlin** multiplatform library. It handles:

1.  **AI Generation:** Calls remote APIs (via `https://femi.market/api`) for image generation (Flux, ZImage, Nano Banana), video generation (Ltx2), and text/audio processing (Qwen3).
2.  **Local Metadata (XMP):** Embeds and reads XMP metadata (prompts, models, subjects, ratings) into local files (images, videos) using platform-specific backends (Adobe XMP Toolkit on Apple, `xmpkit` bytes manipulation on Android/WASM).
3.  **Audio Lyrics (ID3):** Extracts synchronized lyrics (SYLT frames) from MP3 files.

## Architecture

The codebase is split into two main layers:

### 1. Rust FFI (`Rust/`)
The core logic written in Rust. It uses conditional compilation to expose different interfaces per platform:
*   **Apple:** Exposes C functions via `RustFFI.h`. Consumed by Swift via `RustFFI.xcframework`.
*   **Android:** Exposes JNI functions. Consumed by Kotlin via `System.loadLibrary("rust_ffi")`.
*   **WebAssembly:** Exposes `wasm-bindgen` functions. Consumed by JS/TS via `pkg/`.

**Key Files:**
*   `Rust/src/lib.rs`: Entry point, defines shared constants (fallback images, API URL) and global singletons (Tokio runtime, HTTP client).
*   `Rust/src/api/`: Modules for each AI endpoint (e.g., `flux2_pro.rs`, `qwen3_asr_flash.rs`). Each module contains:
    *   `core_*`: Async logic for HTTP requests.
    *   `native`: C ABI wrapper for non-WASM targets.
    *   `wasm`: `wasm-bindgen` wrapper for WASM targets.
*   `Rust/src/project_service/`: XMP metadata handling.
    *   `shared/xmpkit_body.rs`: Platform-agnostic bytes-in/bytes-out logic.
    *   `apple.rs`: Uses `xmp_toolkit` for path-based operations.
    *   `android.rs`: JNI wrappers using `xmpkit_body`.
    *   `wasm.rs`: OPFS (Origin Private File System) wrappers using `xmpkit_body`.
*   `Rust/src/id3/`: ID3 tag parsing for lyrics extraction.

### 2. Platform Bindings
*   **Swift (`Sources/Api/`):** Swift extensions on `Api` enum that call the Rust FFI. Handles memory management (converting `Data` to `UInt8` pointers) and task cancellation.
*   **Kotlin (`Kmp/kotlinapi/`):** Kotlin `external` functions bound to the Rust JNI symbols.

## Build & Install

### Prerequisites
*   **Rust:** `rustup` installed.
*   **Swift:** Swift 6.3+ (for SwiftPM).
*   **Android NDK:** Installed and configured in `Rust/.cargo/config.toml`.
*   **Wasm-bindgen:** Installed on demand by the build script.

### Build Script
Run the following from the repository root to build for all platforms:

```bash
./build-rust.sh
```

This script:
1.  Installs missing Rust targets (iOS, macOS, Android, WASM).
2.  Builds `librust_ffi.a` for Apple targets and creates `RustFFI.xcframework`.
3.  Builds `librust_ffi.so` for Android ABIs and copies them to `Kmp/kotlinapi/src/androidMain/jniLibs/`.
4.  Builds `rust_ffi.wasm` and generates JS bindings in `pkg/`.

### Manual Build Steps

#### Apple (iOS/macOS)
```bash
cd Rust
# Add targets
rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios aarch64-apple-darwin x86_64-apple-darwin

# Build
export IPHONEOS_DEPLOYMENT_TARGET=14.0
export MACOSX_DEPLOYMENT_TARGET=11.0
cargo build --release --target aarch64-apple-ios
cargo build --release --target aarch64-apple-ios-sim
cargo build --release --target x86_64-apple-ios
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-apple-darwin

# Create xcframework
mkdir -p target/ios-sim-universal/release
lipo -create target/aarch64-apple-ios-sim/release/librust_ffi.a target/x86_64-apple-ios/release/librust_ffi.a -output target/ios-sim-universal/release/librust_ffi.a
mkdir -p target/macos-universal/release
lipo -create target/aarch64-apple-darwin/release/librust_ffi.a target/x86_64-apple-darwin/release/librust_ffi.a -output target/macos-universal/release/librust_ffi.a

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

# Build and copy to jniLibs
for triple in aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android; do
  cargo build --release --target $triple
  abi=$(echo $triple | sed 's/.*-//')
  mkdir -p ../Kmp/kotlinapi/src/androidMain/jniLibs/$abi
  cp target/$triple/release/librust_ffi.so ../Kmp/kotlinapi/src/androidMain/jniLibs/$abi/librust_ffi.so
done
```

#### WebAssembly
```bash
cd Rust
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli # if not installed
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/rust_ffi.wasm --out-dir pkg --target web
```

## Usage

### Swift (iOS/macOS)
Import the `Api` package via Swift Package Manager. The `RustFFI.xcframework` is included as a binary target.

```swift
import Api

// Generate an image
let imageData = await Api.flux2Pro(
    user: "username",
    password: "password",
    prompt: "a red apple"
)

// Save with metadata
ProjectService.saveFile(
    imageData,
    named: "apple.png",
    prompt: "a red apple",
    model: "flux-2",
    subject: ["fruit", "red"]
)
```

### Kotlin (Android/JVM/WASM)
Use the `rust_ffi` library loaded via JNI.

```kotlin
// Android
System.loadLibrary("rust_ffi")

// Call FFI
val result = FemiApiJvm.rustFfiFlux2Pro(user, password, prompt, cancelFlag)
```

### WebAssembly
Import the generated JS module.

```javascript
import init, { wasm_flux2_pro } from './pkg/rust_ffi.js';

await init();
const result = await wasm_flux2_pro(user, password, prompt);
```

## API Endpoints

The Rust FFI exposes the following functions (names vary slightly by platform binding):

| Function | Description | Inputs | Output |
| :--- | :--- | :--- | :--- |
| `rust_ffi_z_image_turbo` | Image generation | user, password, prompt | Image bytes |
| `rust_ffi_flux2_pro` | Image generation | user, password, prompt | Image bytes |
| `rust_ffi_flux2_dev_i2i` | Image-to-Image | user, password, image_b64, prompt | Image bytes |
| `rust_ffi_flux2_klein_i2i` | Image-to-Image (2 images) | user, password, image1_b64, image2_b64, prompt | Image bytes |
| `rust_ffi_nano_banana2` | Image generation | user, password, prompt | Image bytes |
| `rust_ffi_ltx2_3a2v` | Video generation | user, password, image_b64, audio_b64, prompt | Video bytes |
| `rust_ffi_qwen3_asr_flash` | Audio transcription | user, password, audio_b64 | Text (lyrics) |
| `rust_ffi_qwen3_6_35b_a3b` | Chat completion | user, password, messages_json | JSON (messages + reply) |
| `psxmp_embed` | Embed XMP metadata | path, prompt, model, subjects | 0 on success |
| `psxmp_read_prompt` | Read prompt | path, buf, buf_len | Length written |
| `psxmp_read_model` | Read model | path, buf, buf_len | Length written |
| `psxmp_read_subject_count` | Read subject count | path | Count |
| `psxmp_read_subject_at` | Read subject by index | path, index, buf, buf_len | Length written |
| `psxmp_set_rating` | Set rating (like) | path, rating | 0 on success |
| `psxmp_read_rating` | Read rating | path | Rating |
| `psxmp_read_property` | Read arbitrary XMP prop | path, ns, name, buf, buf_len | Length written |
| `id3_ffi_extract_sylt` | Extract lyrics from MP3 | bytes, bytes_len | JSON string |

## Cancellation
All AI generation functions accept a `cancel_flag` pointer. If the byte at this address is set to `1` while the operation is running, the Rust side will abort the request and return a fallback image/video/text.

## Testing

### Rust Tests
Run integration tests against the live server:
```bash
cd Rust
cargo test
```
*Note: Tests create a temporary user account (`funded-test-<uuid>`) with auto-funded credits.*

### Swift Tests
Run Swift tests via Xcode or `swift test`. Tests use the same `testUser`/`testPassword` pattern.

### Kotlin Tests
Run via Gradle. Tests use the JNI bindings to verify Android-specific behavior.