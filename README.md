# Api

A Swift Package that provides a unified, type-safe interface to the **femi.market** AI API. It wraps Rust-based FFI bindings to handle asynchronous HTTP requests, cancellation, and error handling, exposing simple Swift async/await methods for image generation, video synthesis, and LLM interactions.

## Overview

This project bridges a Swift frontend with a Rust backend via C-FFI. The Rust layer handles the heavy lifting of network I/O against `https://femi.market/api`, while the Swift layer provides a clean, modern API for iOS and macOS applications.

### Key Features
- **Unified Interface**: Single entry point (`Api`) for all AI models.
- **Cancellation Support**: Native Swift `Task` cancellation propagates to the Rust FFI layer to abort in-flight requests.
- **Automatic Fallbacks**: Returns specific fallback assets (images/videos) or messages for common error states like missing credentials or unpaid accounts (HTTP 402).
- **Cross-Platform**: Supports iOS 15+ and macOS 12+ via a pre-built `xcframework`.

## Architecture

The project consists of three main layers:

1.  **Swift API (`Sources/Api/`)**: High-level Swift functions that convert inputs (e.g., `Data`, `String`) into C-compatible types, invoke the Rust FFI, and parse the JSON response to return native Swift types.
2.  **Rust FFI (`Rust/src/`)**: Low-level Rust code that performs the actual HTTP POST requests using `reqwest` and `tokio`. It manages memory allocation for response bodies, ensuring the caller (Swift) can safely read and free the data.
3.  **Build System (`build-rust.sh` & `Package.swift`)**: Automates the compilation of Rust targets for iOS (device/simulator) and macOS, packaging them into `RustFFI.xcframework`.

### File Structure

```text
.
├── Package.swift              # Swift Package Manifest
├── build-rust.sh              # Script to build Rust FFI and generate xcframework
├── RustFFI.xcframework        # Pre-built binary framework (generated)
├── Sources/
│   └── Api/
│       ├── Api.swift          # Static resource constants (fallback images/videos)
│       ├── Flux2Pro.swift     # Image generation wrapper
│       ├── Flux2DevI2I.swift  # Image-to-Image wrapper
│       ├── Flux2KleinI2I.swift# Image-to-Image with reference wrapper
│       ├── NanoBanana2.swift  # Image generation wrapper
│       ├── ZImageTurbo.swift  # Image generation wrapper
│       ├── Ltx2_3A2V.swift    # Video generation wrapper
│       ├── Qwen3AsrFlash.swift# Audio transcription wrapper
│       └── Qwen3_6_35b_a3b.swift # LLM Chat wrapper
├── Tests/
│   └── ApiTests/              # Unit and integration tests
└── Rust/
    ├── include/
    │   └── RustFFI.h          # C header for FFI bindings
    └── src/
        ├── lib.rs             # Rust entry point, runtime setup
        ├── flux2_pro.rs       # FFI implementation for Flux2Pro
        ├── flux2_dev_i2i.rs   # FFI implementation for Flux2DevI2I
        └── ...                # Other FFI implementations
```

## Installation

Add the package to your Xcode project or `Package.swift` file.

### Swift Package Manager

```swift
dependencies: [
    .package(url: "https://github.com/your-org/api.git", from: "1.0.0")
]
```

### Requirements
- **Swift**: 6.0+
- **iOS**: 15.0+
- **macOS**: 12.0+
- **Rust**: Required only if you intend to rebuild the FFI layer (see Building).

## Usage

Import the package and call the static methods on `Api`. All methods are `async` and support cancellation.

### Image Generation

```swift
let image = await Api.flux2Pro(
    user: "my_username",
    password: "my_password",
    prompt: "A futuristic cityscape at sunset"
)
// Returns Data (JPEG/PNG) or fallback image on error
```

### Image-to-Image

```swift
let inputImage = ... // Data
let referenceImage = ... // Data

let result = await Api.flux2KleinI2I(
    user: "my_username",
    password: "my_password",
    image: inputImage,
    image2: referenceImage,
    prompt: "Change the style to cyberpunk"
)
```

### Video Generation

```swift
let video = await Api.ltx2_3a2v(
    user: "my_username",
    password: "my_password",
    image: imageData,
    audio: audioData,
    prompt: "The character waves hello"
)
// Returns MP4 Data or fallback video on error
```

### LLM Chat

```swift
var messages: [(role: String, content: String)] = [
    (role: "User", content: "Hello")
]

messages = await Api.qwen3_6_35b_a3b(
    user: "my_username",
    password: "my_password",
    messages: messages
)
// Returns updated messages array with the assistant's reply appended
```

### Cancellation

You can cancel any request by cancelling the parent `Task`. The Swift layer will signal the Rust FFI to abort the HTTP request.

```swift
let task = Task {
    await Api.flux2Pro(user: "...", password: "...", prompt: "...")
}

// Cancel after 2 seconds
try await Task.sleep(nanoseconds: 2_000_000_000)
task.cancel()

// The task will resolve quickly with a fallback response
let result = await task.value
```

## Building the FFI

If you need to rebuild the Rust bindings (e.g., after modifying `Rust/src/`), run the provided shell script from the repository root.

```bash
chmod +x build-rust.sh
./build-rust.sh
```

This script:
1. Installs necessary Rust targets (iOS device/simulator, macOS).
2. Compiles the Rust crate in release mode for all targets.
3. Creates universal binaries for iOS simulator and macOS using `lipo`.
4. Packages everything into `RustFFI.xcframework`.

**Prerequisites**:
- `rustup` and `cargo` installed.
- `xcodebuild` available.

## Error Handling & Fallbacks

The API is designed to be resilient. Instead of throwing errors, it returns fallback assets or messages based on the HTTP status code returned by the server.

| Status | Behavior |
| :--- | :--- |
| **200** | Returns the generated content (Image, Video, Text). |
| **402** | Returns a "Top Up" image/video or message (e.g., `"Top up to transcribe lyrics"`). |
| **Other / Network Error** | Returns a generic "Fallback" image/video or message (e.g., `"Could not respond"`). |

### Fallback Assets

The `Api` struct exposes static `Data` constants for these fallbacks, which can be used in UI components:

- `Api.fallbackImage`: Generic error image.
- `Api.topupImage`: Payment required image.
- `Api.fallbackVideo`: Generic error video.
- `Api.topupVideo`: Payment required video.

## Testing

Tests are located in `Tests/ApiTests/`. They require valid credentials to run against the live API.

```bash
API_USER="your_username" API_PASSWORD="your_password" swift test
```

### Test Coverage
- **Unfunded User**: Verifies that invalid/unpaid credentials return the correct "Top Up" fallback.
- **Missing Credentials**: Verifies that empty credentials return the generic "Fallback" asset.
- **Cancellation**: Verifies that cancelling a task results in a quick resolution with a fallback response.
- **Funded User**: Verifies that valid credentials return actual generated content.