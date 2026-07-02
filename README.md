# Api (Rust FFI & Swift Wrapper)

A cross-platform FFI crate (`rust_ffi`) that exposes AI generation endpoints, XMP metadata manipulation, and ID3 tag parsing to Swift (iOS/macOS), Kotlin (Android), and WebAssembly. The project provides a unified Rust backend with platform-specific bindings and a Swift wrapper (`Api`) that consumes the native library.

## Architecture Overview

The project is split into two main layers:

1.  **Rust FFI Crate (`Rust/`)**: A `cdylib` compiled for multiple targets. It handles network requests to `https://femi.market/api`, XMP metadata embedding/reading, and ID3 lyric extraction.
2.  **Swift Package (`Sources/Api/`)**: A Swift wrapper that links against the compiled `RustFFI.xcframework`. It provides idiomatic Swift async APIs, handles memory management of FFI pointers, and manages local file storage via `ProjectService`.

### Supported Platforms & Outputs

| Platform | Output Path | Consumption Method |
| :--- | :--- | :--- |
| **iOS / macOS** | `RustFFI.xcframework` | Swift Package Manager `binaryTarget` |
| **Android** | `Kmp/kotlinapi/src/androidMain/jniLibs/*` | `System.loadLibrary("rust_ffi")` |
| **WebAssembly** | `Rust/pkg/` | `npm("rust_ffi", ...)` in `build.gradle.kts` |

## Key Files & Directories

*   `build-rust.sh`: The primary build script. Compiles the Rust crate for all supported targets (Apple, Android, WASM) and places outputs in the correct directories.
*   `Rust/src/lib.rs`: The Rust entry point. Defines shared constants (fallback images/videos), the global `reqwest` client, and the Tokio runtime.
*   `Rust/src/api/`: Contains the core business logic for AI endpoints (Flux, Qwen, etc.). Each endpoint has a `native` module (C ABI) and a `wasm` module.
*   `Rust/src/project_service/`: Handles XMP metadata. Uses `xmp_toolkit` on Apple and `xmpkit` (bytes-based) on Android/WASM.
*   `Rust/src/id3/`: Extracts synchronized lyrics (SYLT) from MP3 files.
*   `Sources/Api/`: Swift source files. `ProjectService.swift` manages local file I/O and XMP embedding via the FFI. `Flux2Pro.swift`, `ZImageTurbo.swift`, etc., wrap the FFI calls with Swift concurrency.
*   `Package.swift`: Swift Package Manifest. Defines the `Api` product, links `RustFFI.xcframework`, and includes test resources.

## Build & Installation

### Prerequisites

*   **Rust**: `rustup` installed.
*   **Android NDK**: Required for Android targets. Ensure `Rust/.cargo/config.toml` is configured correctly.
*   **Wasm-bindgen**: Installed automatically by the build script if missing (`cargo install wasm-bindgen-cli`).
*   **Apple Deployment Targets**: The script sets `IPHONEOS_DEPLOYMENT_TARGET=14.0` and `MACOSX_DEPLOYMENT_TARGET=11.0` to match Swift Package floors.

### Building

Run the build script from the repository root:

```bash
./build-rust.sh
```

This script will:
1.  Install necessary Rust targets (`aarch64-apple-ios`, `x86_64-apple-darwin`, `aarch64-linux-android`, `wasm32-unknown-unknown`, etc.).
2.  Build the `rust_ffi` crate for each target.
3.  Create `RustFFI.xcframework` from Apple binaries.
4.  Copy `.so` files to `Kmp/kotlinapi/src/androidMain/jniLibs/`.
5.  Generate WebAssembly bindings in `Rust/pkg/`.

## Usage

### Swift (iOS/macOS)

Import the `Api` package. The Swift wrapper handles memory safety and cancellation.

```swift
import Api

// Generate an image
let imageData = await Api.flux2Pro(
    user: "my_user",
    password: "my_pass",
    prompt: "a red apple on a wooden table"
)

// Generate video from image + audio
let videoData = await Api.ltx2_3a2v(
    user: "my_user",
    password: "my_pass",
    image: imageBytes,
    audio: audioBytes,
    prompt: "the man walks forward"
)

// Save file with XMP metadata
ProjectService.saveFile(
    imageData,
    named: "apple.png",
    prompt: "red apple",
    model: "flux-pro",
    subject: ["fruit", "table"]
)

// Read metadata
let prompt = ProjectService.getPrompt("apple.png")
let rating = ProjectService.getLike("apple.png")
```

### Android (Kotlin)

The Rust crate exposes JNI functions directly. The Kotlin side typically loads the library and calls native methods.

```kotlin
// In your Kotlin code
class FemiApiJvm {
    companion object {
        init { System.loadLibrary("rust_ffi") }
    }

    external fun rustFfiFlux2Pro(
        user: String,
        password: String,
        prompt: String,
        cancelFlag: Long
    ): ByteArray
}
```

### WebAssembly

The crate exposes `wasm_bindgen` functions. Use the generated JS bindings in `Rust/pkg/`.

```javascript
import init, { wasm_flux2_pro } from './pkg/rust_ffi.js';

await init();
const result = await wasm_flux2_pro(user, password, prompt);
// result is a Uint8Array
```

## API Endpoints

The Rust FFI exposes the following AI endpoints. All return `uint8_t*` (bytes) and `size_t` (length). On failure or cancellation, they return embedded fallback assets (images/videos) or error strings.

| Function | Description | Inputs |
| :--- | :--- | :--- |
| `rust_ffi_z_image_turbo` | Text-to-Image | `user`, `password`, `prompt` |
| `rust_ffi_flux2_pro` | Text-to-Image | `user`, `password`, `prompt` |
| `rust_ffi_flux2_dev_i2i` | Image-to-Image (Dev) | `user`, `password`, `image_b64`, `prompt` |
| `rust_ffi_flux2_klein_i2i` | Image-to-Image (Klein) | `user`, `password`, `image_b64`, `image2_b64`, `prompt` |
| `rust_ffi_nano_banana2` | Text-to-Image | `user`, `password`, `prompt` |
| `rust_ffi_ltx2_3a2v` | Image+Audio-to-Video | `user`, `password`, `image_b64`, `audio_b64`, `prompt` |
| `rust_ffi_qwen3_asr_flash` | Audio-to-Text (Lyrics) | `user`, `password`, `audio_b64` |
| `rust_ffi_qwen3_6_35b_a3b` | Chat Completion | `user`, `password`, `messages_json` |

### Cancellation

All AI FFI functions accept a `cancel_flag: *const u8`. If the value at this address is non-zero, the operation cancels and returns a fallback asset. The Swift wrapper manages this flag automatically via `withTaskCancellationHandler`.

## XMP Metadata (ProjectService)

The `ProjectService` module embeds and reads XMP metadata into images and videos using the Adobe XMP Toolkit (Apple) or `xmpkit` (Android/WASM).

### Embedded Properties

*   **Prompt**: Stored in `dc:description` (Lang Alt) and `Iptc4xmpExt:AIPromptInformation`.
*   **Model**: Stored in `xmp:CreatorTool` and `Iptc4xmpExt:AISystemUsed`.
*   **Subject**: Stored in `dc:subject` (Bag).
*   **Rating**: Stored in `xmp:Rating` (5 = liked, 0 = not liked, -100 = absent).

### API

*   `psxmp_embed(path, prompt, model, subjects, count)`: Embeds metadata into a file.
*   `psxmp_read_prompt(path, buf, len)`: Reads prompt.
*   `psxmp_read_model(path, buf, len)`: Reads model.
*   `psxmp_read_subject_count(path)`: Returns number of subjects.
*   `psxmp_read_subject_at(path, index, buf, len)`: Reads subject at index.
*   `psxmp_set_rating(path, rating)`: Sets rating.
*   `psxmp_read_rating(path)`: Reads rating.
*   `psxmp_read_property(path, ns, name, buf, len)`: Reads arbitrary XMP property.

## ID3 Lyrics Extraction

Extracts synchronized lyrics (SYLT) from MP3 files.

*   **Native**: `id3_ffi_extract_sylt(bytes, len, out_len)` returns a heap-allocated UTF-8 JSON string.
*   **WASM**: `extract_sylt(bytes)` returns a JSON string.

## Testing

### Swift Tests

Located in `Tests/ApiTests/`. Run via Swift Package Manager:

```bash
swift test
```

Tests use a live server with auto-funded test accounts.

### Rust Tests

Located in `Rust/tests/`. Run via Cargo:

```bash
cd Rust
cargo test
```

Tests include integration tests for all AI endpoints and round-trip tests for XMP metadata.