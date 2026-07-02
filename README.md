# Femi Rust FFI & Swift API

A cross-platform Rust library exposing AI generation endpoints, XMP metadata handling, and ID3 tag parsing via FFI. It provides a unified binary interface for iOS/macOS (via `xcframework`), Android (via JNI), and Web (via WebAssembly).

The project also includes a Swift wrapper (`Api`) that exposes these capabilities to native iOS/macOS applications through a clean, async-first API.

## Architecture Overview

The codebase is split into two main layers:

1.  **Rust Core (`Rust/`)**: The source of truth. Contains the FFI bindings, network clients, and platform-specific backends.
2.  **Swift Wrapper (`Sources/Api/`)**: A thin layer over the Rust FFI that provides idiomatic Swift async/await interfaces, cancellation support, and file management utilities.

### Platform Targets

The Rust crate (`rust_ffi`) is compiled for three distinct platforms, producing different outputs:

| Platform | Output Path | Consumption Method |
| :--- | :--- | :--- |
| **Apple** (iOS/macOS) | `RustFFI.xcframework` | Swift Package Manager (`Package.swift`) |
| **Android** | `Kmp/kotlinapi/src/androidMain/jniLibs/` | `System.loadLibrary("rust_ffi")` |
| **Web** | `pkg/` | npm package (`rust_ffi`) |

## Key Files & Directories

*   `build-rust.sh`: The master build script. Compiles the Rust crate for all targets and produces the final artifacts.
*   `Package.swift`: Swift Package Manifest. Defines the `Api` target, links against `RustFFI.xcframework`, and includes test resources.
*   `Rust/src/lib.rs`: Rust entry point. Defines shared constants, the HTTP client, and fallback assets.
*   `Rust/src/api/`: Contains implementations for AI endpoints (Flux, Qwen, etc.). Each endpoint has a `native` (C/FFI) and `wasm` module.
*   `Rust/src/project_service/`: Handles XMP metadata embedding and reading. Uses `xmp_toolkit` on Apple and `xmpkit` (bytes-based) on Android/WASM.
*   `Rust/src/id3/`: Handles ID3v2 SYLT (synchronized lyrics) extraction.
*   `Sources/Api/`: Swift source files. Each AI endpoint has a corresponding file (e.g., `Flux2Pro.swift`) that wraps the FFI call.
*   `Tests/ApiTests/`: Swift unit and integration tests using the Swift Testing framework.

## Installation & Build

### Prerequisites

*   **Rust**: `rustup` installed.
*   **Android NDK**: Required for Android targets. Ensure `Rust/.cargo/config.toml` points to the correct NDK path.
*   **Wasm-bindgen**: Installed automatically by the build script if missing.
*   **Xcode Command Line Tools**: Required for `xcodebuild` and `lipo`.

### Building

Run the build script from the repository root:

```bash
chmod +x build-rust.sh
./build-rust.sh
```

This script will:
1.  Install necessary Rust targets (Apple, Android, WASM).
2.  Build the Rust crate for each target.
3.  Create `RustFFI.xcframework` containing universal binaries for iOS (device/simulator) and macOS.
4.  Copy `.so` libraries to the Android `jniLibs` directory.
5.  Generate the WebAssembly package in `pkg/`.

## Usage

### Swift (iOS/macOS)

Import the `Api` module in your Swift code. The API is exposed via the `Api` enum and `ProjectService` struct.

#### AI Endpoints

All AI endpoints are async and support cancellation. They return `Data` (bytes) on success.

```swift
import Api

// Example: Generate an image
let imageData = await Api.flux2Pro(
    user: "my_user",
    password: "my_password",
    prompt: "a futuristic city"
)

// Example: Image-to-Image
let inputImage = ... // Data
let resultImage = await Api.flux2DevI2I(
    user: "my_user",
    password: "my_password",
    image: inputImage,
    prompt: "make it rainy"
)

// Example: Chat
let messages: [(Api.Role, String)] = [
    (.user, "Hello")
]
let response = await Api.qwen3_6_35b_a3b(
    user: "my_user",
    password: "my_password",
    messages: messages
)
// response contains the original messages + the assistant's reply
```

#### Project Service (XMP Metadata)

`ProjectService` handles saving files to the app's Documents directory and embedding/reading XMP metadata.

```swift
import Api

// Save a file with metadata
let fileData = ... // Data
ProjectService.saveFile(
    fileData,
    named: "generated.png",
    prompt: "A cat",
    model: "flux-2",
    subject: ["cat", "outdoor"]
)

// Read metadata
let prompt = ProjectService.getPrompt("generated.png") // "A cat"
let model = ProjectService.getModel("generated.png")   // "flux-2"
let subjects = ProjectService.getSubject("generated.png") // ["cat", "outdoor"]

// Like/Unlike
ProjectService.like("generated.png", true)
let isLiked = ProjectService.getLike("generated.png") // true
```

### Android (Kotlin/JNI)

The Rust library is exposed via JNI. The Kotlin side should load the library and call the native methods defined in `Rust/src/api/jni_api.rs`.

```kotlin
// Kotlin side example
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

    // ... other FFI bindings
}
```

*Note: The `cancelFlag` is a pointer to an atomic boolean. Set it to 1 to cancel the operation.*

### WebAssembly

The WASM module is built using `wasm-bindgen`. It exposes async functions that return `Uint8Array` or `String`.

```javascript
import init, { wasm_flux2_pro } from './pkg/rust_ffi.js';

await init();
const result = await wasm_flux2_pro("user", "pass", "prompt");
// result is a Uint8Array
```

## API Endpoints

The following AI endpoints are supported. All return `Data` (bytes) on success.

| Endpoint | Function | Description |
| :--- | :--- | :--- |
| **ZImageTurbo** | `Api.zImageTurbo` | Text-to-Image generation. |
| **Flux2Pro** | `Api.flux2Pro` | Text-to-Image generation (Flux Pro). |
| **Flux2DevI2I** | `Api.flux2DevI2I` | Image-to-Image generation (Flux Dev). |
| **Flux2KleinI2I** | `Api.flux2KleinI2I` | Image-to-Image generation with two input images (Flux Klein). |
| **NanoBanana2** | `Api.nanoBanana2` | Text-to-Image generation (Nano Banana). |
| **Ltx2_3A2V** | `Api.ltx2_3a2v` | Image + Audio to Video generation. |
| **Qwen3AsrFlash** | `Api.qwen3AsrFlash` | Audio-to-Text (Lyrics transcription). Returns a `String`. |
| **Qwen3_6_35b_a3b** | `Api.qwen3_6_35b_a3b` | Chat completion. Returns updated message history. |

## Cancellation

All AI endpoints support cancellation via a `cancelFlag` pointer.

1.  Allocate an `UnsafeMutablePointer<UInt8>` and initialize it to `0`.
2.  Pass the pointer's address to the FFI function.
3.  To cancel, set the value at the pointer to `1`.
4.  The FFI function will detect the change and return a fallback asset (image/video/text) immediately.

In Swift, this is handled automatically by the `Api` wrapper using `withTaskCancellationHandler`.

## Fallback Assets

If an API call fails or is cancelled, the Rust FFI returns embedded fallback assets:

*   **Images**: `fallback.png` (generic error) or `topup.jpg` (payment required).
*   **Videos**: `could-not-generate.mp4` (generic error) or `topup-video.mp4` (payment required).

These assets are compiled into the Rust binary using `include_bytes!`.

## Testing

### Swift Tests

Run Swift tests using Xcode or the Swift CLI:

```bash
swift test
```

Tests are located in `Tests/ApiTests/`. They use a live server with auto-funded test accounts.

### Rust Tests

Run Rust integration tests:

```bash
cd Rust
cargo test
```

Tests are located in `Rust/tests/`. They verify FFI behavior and XMP round-trips.

## Non-Obvious Conventions

*   **Memory Management**: FFI functions return heap-allocated pointers (`*mut u8`). The caller is responsible for freeing this memory. In Swift, the wrapper uses `Data(bytesNoCopy:count:deallocator:)` to handle this automatically. In Rust tests, use the `take` helper to reclaim memory.
*   **XMP Namespaces**: The project uses specific XMP namespaces for AI metadata:
    *   `dc:description`: Prompt (Lang Alt).
    *   `Iptc4xmpExt:AIPromptInformation`: Prompt (String).
    *   `xmp:CreatorTool`: Model name.
    *   `Iptc4xmpExt:AISystemUsed`: Model name (String).
    *   `dc:subject`: Subject keywords (Bag).
    *   `xmp:Rating`: Like state (5 = liked, 0 = not).
*   **Android JNI Naming**: JNI function names follow the pattern `Java_market_femi_kotlinapi_<ClassName>_<methodName>`.
*   **WASM OPFS**: On Web, `ProjectService` uses the Origin Private File System (OPFS) for persistence.