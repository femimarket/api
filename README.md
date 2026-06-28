# Api

A Swift package that provides a high-level, async API for interacting with the **femi.market** AI service. It wraps a Rust FFI crate to handle HTTP requests, authentication, and cancellation, exposing clean Swift interfaces for image generation, video synthesis, and LLM chat.

## Overview

This project bridges Swift and Rust to provide performant, cancellable network calls to an AI backend. The Rust layer (`Rust/`) handles the heavy lifting: building a static library for iOS/macOS, managing the Tokio async runtime, and exposing C-compatible functions. The Swift layer (`Sources/Api/`) wraps these FFI calls in idiomatic Swift async/await functions, handling memory management, base64 encoding, and error fallbacks.

### Supported Models

The package exposes endpoints for the following models via `Api` static methods:

*   **Image Generation:**
    *   `zImageTurbo`: Text-to-image.
    *   `nanoBanana2`: Text-to-image.
    *   `flux2Pro`: Text-to-image.
    *   `flux2DevI2I`: Image-to-image (single input).
    *   `flux2KleinI2I`: Image-to-image (dual input/reference).
*   **Video Generation:**
    *   `ltx2_3a2v`: Text/Audio/Image-to-video.
*   **Audio/Text:**
    *   `qwen3AsrFlash`: Audio transcription/lyrics extraction.
    *   `qwen3_6_35b_a3b`: Chat completion (LLM).

## Architecture

The project consists of two main parts:

1.  **Rust FFI Crate (`Rust/`)**:
    *   Written in Rust using `tokio` for async HTTP requests via `reqwest`.
    *   Exposes functions prefixed with `rust_ffi_` in `Rust/include/RustFFI/RustFFI.h`.
    *   Builds into a static library (`librust_ffi.a`) for multiple architectures.
    *   Handles cancellation via a shared atomic flag pointer passed from Swift.
    *   Memory management: Returns heap-allocated bytes (`*mut u8`) that the caller must `free()`.

2.  **Swift Package (`Sources/Api/`)**:
    *   Depends on the `RustFFI` binary target.
    *   Provides `Api` enum with static async methods.
    *   Handles string interop (`withCString`), base64 encoding, and JSON parsing.
    *   Implements cancellation using `withTaskCancellationHandler` to flip the Rust-side cancel flag.
    *   Returns fallback assets (images/videos) or error strings on failure (e.g., HTTP 402 for unpaid, or transport errors).

## Installation

Add this package to your Swift project using Swift Package Manager.

```swift
dependencies: [
    .package(url: "https://github.com/your-org/api.git", from: "1.0.0")
]
```

### Requirements

*   **Swift**: 6.0+ (declared as `.v6` in `Package.swift`)
*   **Platforms**: iOS 15+, macOS 12+
*   **Rust**: Required only if you need to rebuild the FFI crate. Ensure `rustup` and `cargo` are installed.

## Building the FFI Crate

The pre-built `RustFFI.xcframework` is included in the repository. To rebuild it (e.g., after modifying Rust code), run the provided build script from the repository root:

```bash
chmod +x build-rust.sh
./build-rust.sh
```

This script:
1.  Adds necessary Rust targets (`aarch64-apple-ios`, `x86_64-apple-ios`, `aarch64-apple-darwin`, etc.).
2.  Compiles the Rust crate in release mode for each target.
3.  Creates universal binaries for iOS Simulator and macOS using `lipo`.
4.  Packages everything into `RustFFI.xcframework`.

## Usage

### Basic Image Generation

```swift
import Api

// Generate an image using Flux2Pro
let imageData = await Api.flux2Pro(
    user: "your_username",
    password: "your_password",
    prompt: "a futuristic cityscape at sunset"
)

// imageData contains the generated PNG/JPEG bytes, or a fallback image if the request failed.
```

### Image-to-Image

```swift
import Api

let inputImage = ... // Data object
let referenceImage = ... // Data object

let result = await Api.flux2KleinI2I(
    user: "your_username",
    password: "your_password",
    image: inputImage,
    image2: referenceImage,
    prompt: "transform the style to cyberpunk"
)
```

### Chat Completion

```swift
import Api

var messages: [(role: String, content: String)] = [
    (role: "User", content: "Hello, who are you?")
]

// The API appends the assistant's response to the messages array
messages = await Api.qwen3_6_35b_a3b(
    user: "your_username",
    password: "your_password",
    messages: messages
)

// messages.last?.content now contains the reply
```

### Cancellation

All API methods support cancellation. If the Swift `Task` is cancelled, the Rust side will abort the HTTP request and return a fallback response.

```swift
let task = Task {
    await Api.flux2Pro(user: "u", password: "p", prompt: "long generation...")
}

// Cancel after 1 second
try await Task.sleep(nanoseconds: 1_000_000_000)
task.cancel()

// The task will resolve quickly with a fallback image instead of waiting for the full generation.
```

## Error Handling & Fallbacks

The Swift layer is designed to be resilient. It never throws; instead, it returns fallback data or strings:

*   **HTTP 402 (Payment Required)**: Returns `Api.topupImage` (for image/video models) or `"Top up to transcribe lyrics"` (for ASR).
*   **Other Errors (Network, Auth, Server 5xx)**: Returns `Api.fallbackImage`, `Api.fallbackVideo`, or `"Could not respond"` / `"Could not process lyrics"`.
*   **Cancellation**: Returns the same fallbacks as other errors.

You can check if the result is a fallback by comparing it to `Api.fallbackImage` or `Api.fallbackVideo`.

## Project Structure

```
.
├── Package.swift                  # Swift Package Manifest
├── build-rust.sh                  # Script to build Rust FFI crate
├── RustFFI.xcframework/           # Pre-built binary framework
│   ├── ios-arm64/                 # iOS Device
│   ├── ios-arm64_x86_64-simulator/ # iOS Simulator
│   └── macos-arm64_x86_64/        # macOS
├── Rust/                          # Rust Source Code
│   ├── include/RustFFI/RustFFI.h  # C Header for FFI
│   └── src/
│       ├── lib.rs                 # Main Rust lib, runtime/client setup
│       ├── flux2_pro.rs           # Flux2Pro implementation
│       ├── flux2_dev_i2i.rs       # Flux2DevI2I implementation
│       ├── flux2_klein_i2i.rs     # Flux2KleinI2I implementation
│       ├── z_image_turbo.rs       # ZImageTurbo implementation
│       ├── nano_banana2.rs        # NanoBanana2 implementation
│       ├── ltx2_3a2v.rs           # Ltx2_3A2V implementation
│       ├── qwen3_asr_flash.rs     # Qwen3 ASR implementation
│       └── qwen3_6_35b_a3b.rs     # Qwen3 Chat implementation
├── Sources/
│   └── Api/
│       ├── Api.swift              # Resource constants (fallback images/videos)
│       ├── Flux2Pro.swift         # Swift wrapper for Flux2Pro
│       ├── Flux2DevI2I.swift      # Swift wrapper for Flux2DevI2I
│       ├── Flux2KleinI2I.swift    # Swift wrapper for Flux2KleinI2I
│       ├── ZImageTurbo.swift      # Swift wrapper for ZImageTurbo
│       ├── NanoBanana2.swift      # Swift wrapper for NanoBanana2
│       ├── Ltx2_3A2V.swift        # Swift wrapper for Ltx2_3A2V
│       ├── Qwen3AsrFlash.swift    # Swift wrapper for Qwen3 ASR
│       └── Qwen3_6_35b_a3b.swift  # Swift wrapper for Qwen3 Chat
└── Tests/
    └── ApiTests/
        ├── ApiTests.swift         # General tests (cancellation, unfunded users)
        ├── Flux2ProTests.swift    # Endpoint-specific tests
        └── ...                    # Other endpoint tests
```

## Testing

Tests require valid credentials to hit the live API. Set the environment variables before running tests:

```bash
API_USER="your_username" API_PASSWORD="your_password" swift test
```

The tests verify:
1.  **Funded User**: Returns real generated content (non-empty, not fallback).
2.  **Unfunded User**: Returns top-up fallback assets.
3.  **Missing Credentials**: Returns generic fallback assets.
4.  **Cancellation**: Resolves quickly (<1s) with fallback content when the task is cancelled.