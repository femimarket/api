# Api

A Swift Package that provides a high-level, async API for interacting with the **femi.market** AI service. It wraps Rust-based FFI bindings to handle HTTP requests, authentication, and cancellation, exposing a clean Swift interface for image generation, video synthesis, and text/audio processing.

## Features

- **Multi-Platform Support**: Built for iOS 15+ and macOS 12+ via a universal `xcframework`.
- **Async/Cancellation**: Fully integrated with Swift Concurrency. Tasks can be cancelled mid-flight, and the underlying Rust layer respects cancellation flags.
- **Unified Error Handling**: Returns fallback assets (images/videos) or specific error strings on failure, simplifying UI logic.
- **Model Support**:
  - **Image Generation**: `ZImageTurbo`, `NanoBanana2`, `Flux2Pro`, `Flux2DevI2I` (Image-to-Image), `Flux2KleinI2I` (Multi-Image).
  - **Video Synthesis**: `Ltx2_3A2V` (Image + Audio to Video).
  - **Text/Chat**: `Qwen3_6_35b_a3b` (LLM chat with history).
  - **Audio**: `Qwen3AsrFlash` (Speech-to-text/Lyrics).

## Installation

Add the package to your `Package.swift` or Xcode project. Since this repository *is* the package, you can include it directly.

### Swift Package Manager

```swift
// Package.swift
dependencies: [
    .package(path: "./Api") // Or remote URL if hosted
]
```

### Requirements

- **Swift 6.0+** (Declared in `Package.swift` via `swiftLanguageModes: [.v6]`)
- **Rust Toolchain**: Required only if you need to rebuild the FFI layer (`build-rust.sh`).
- **Xcode**: Required to build the `xcframework`.

## Usage

Import the `Api` module and call the static methods on the `Api` enum. All methods are `async` and return `Data` (for images/videos) or `String` (for text/lyrics).

### Image Generation

```swift
import Api

// Text-to-Image
let image = await Api.flux2Pro(
    user: "my_user",
    password: "my_password",
    prompt: "a cyberpunk city at night"
)

// Image-to-Image
let inputImage = // ... Data object
let result = await Api.flux2DevI2I(
    user: "my_user",
    password: "my_password",
    image: inputImage,
    prompt: "make it rainy"
)
```

### Video Synthesis

```swift
let video = await Api.ltx2_3a2v(
    user: "my_user",
    password: "my_password",
    image: imageData,
    audio: audioData,
    prompt: "the character speaks"
)
```

### Chat (LLM)

```swift
var messages: [(role: String, content: String)] = [
    (role: "User", content: "Hello")
]

messages = await Api.qwen3_6_35b_a3b(
    user: "my_user",
    password: "my_password",
    messages: messages
)
// messages now contains the original + the assistant's reply
```

### Cancellation

All API calls support cancellation via `Task`. If the task is cancelled, the request is aborted, and a fallback response is returned immediately.

```swift
let task = Task {
    await Api.flux2Pro(user: "...", password: "...", prompt: "...")
}

// Cancel after 2 seconds
try await Task.sleep(nanoseconds: 2_000_000_000)
task.cancel()

let result = await task.value // Returns fallback image
```

## Architecture

The project consists of two main layers:

1.  **Rust FFI (`Rust/`)**:
    -   Written in Rust, exposing C-compatible functions (`extern "C"`).
    -   Handles HTTP requests to `https://femi.market/api` using `reqwest`.
    -   Manages a shared Tokio runtime for async operations.
    -   Supports cancellation via a shared atomic flag pointer.
    -   Builds into static libraries (`librust_ffi.a`) for iOS (arm64, x86_64, arm64-sim) and macOS (arm64, x86_64).

2.  **Swift Wrapper (`Sources/Api/`)**:
    -   Imports the `RustFFI` binary target.
    -   Provides Swift-native async functions.
    -   Handles memory management (converting Swift `String`/`Data` to C pointers and back).
    -   Parses JSON responses from the Rust layer.
    -   Implements fallback logic (e.g., returning `Api.topupImage` on HTTP 402).

### Key Files

-   `Package.swift`: Defines the Swift package, dependencies, and resources.
-   `build-rust.sh`: Script to compile the Rust crate into `RustFFI.xcframework`.
-   `Rust/include/RustFFI.h`: The C header file defining the FFI interface.
-   `Rust/src/lib.rs`: Rust entry point, initializes the Tokio runtime and HTTP client.
-   `Sources/Api/Api.swift`: Contains static resources (fallback images/videos).
-   `Sources/Api/*.swift`: Individual Swift wrappers for each AI model.

## Building

### Prerequisites

Ensure you have Rust and Cargo installed.

```bash
rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios aarch64-apple-darwin x86_64-apple-darwin
```

### Build the Framework

Run the build script from the repository root:

```bash
chmod +x build-rust.sh
./build-rust.sh
```

This will:
1.  Compile the Rust crate for all target platforms.
2.  Create universal binaries for iOS Simulator and macOS.
3.  Package everything into `RustFFI.xcframework`.

### Build the Swift Package

```bash
swift build
```

## Testing

Tests are located in `Tests/ApiTests/`. They require valid credentials to run against the live API.

```bash
API_USER="your_user" API_PASSWORD="your_password" swift test
```

### Test Coverage

-   **Unfunded User**: Verifies that invalid/empty credentials return the appropriate fallback assets (`topupImage`, `topupVideo`, or error strings).
-   **Missing Credentials**: Verifies generic fallback behavior.
-   **Cancellation**: Verifies that cancelling a task returns a fallback response quickly (<1s).
-   **Funded User**: Verifies that valid credentials return real generated content (images, videos, text).

## Error Handling & Fallbacks

The API is designed to be resilient. Instead of throwing errors, it returns fallback content:

-   **HTTP 402 (Payment Required)**: Returns `Api.topupImage` (for image models) or `Api.topupVideo` (for video models). For text/audio, returns `"Top up to transcribe lyrics"`.
-   **Other Errors (Network, Auth, etc.)**: Returns `Api.fallbackImage` or `Api.fallbackVideo`. For text/audio, returns `"Could not respond"` or `"Could not process lyrics"`.

This allows UIs to display consistent placeholder content without complex error handling logic.