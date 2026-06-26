# Api

A Swift package that provides a high-level API for interacting with the **femi.market** image generation service (`ZImageTurbo` model). It wraps a Rust FFI crate to handle HTTP requests, cancellation, and memory management efficiently across iOS and macOS platforms.

## Overview

This project bridges Swift and Rust to provide a performant, cancellation-safe interface for AI image generation.

- **Rust Layer**: Handles the heavy lifting of HTTP communication using `reqwest` and `tokio`. It manages a shared runtime and client, supports cancellation via atomic flags, and returns heap-allocated memory to Swift.
- **Swift Layer**: Provides a clean, async/await-native API (`Api.zImageTurbo`) that hides the complexity of FFI calls, memory allocation, and cancellation handling.
- **Platform Support**: iOS 15+ and macOS 12+.

## Architecture

The project consists of three main components:

1.  **`Rust/`**: Contains the Rust source code (`src/lib.rs`) and headers (`include/RustFFI.h`). This crate is compiled into static libraries for various Apple architectures.
2.  **`RustFFI.xcframework`**: A pre-built (or build-time generated) XCFramework containing the compiled Rust libraries for iOS (device/simulator) and macOS (universal). This is consumed by Swift Package Manager.
3.  **`Sources/Api/`**: The Swift package source code that imports `RustFFI` and exposes the `Api` struct.

### Key Files

-   `Package.swift`: Defines the Swift package, dependencies, and platform constraints.
-   `build-rust.sh`: A bash script to compile the Rust FFI crate for all required targets and package it into `RustFFI.xcframework`.
-   `Rust/src/lib.rs`: The Rust implementation of the FFI boundary.
-   `Sources/Api/Api.swift`: The Swift wrapper providing the public API.

## Installation

### Prerequisites

-   **Swift 6.3+**: Ensure you are using a Swift toolchain that supports Swift 6 language modes.
-   **Rust Toolchain**: `rustup` and `cargo` must be installed and available in your `PATH`.
-   **Xcode Command Line Tools**: Required for `xcodebuild` and `lipo`.

### Building the Rust FFI

Before using the package in a Swift project, you must build the `RustFFI.xcframework`. Run the following script from the project root:

```bash
chmod +x build-rust.sh
./build-rust.sh
```

This script will:
1.  Add necessary Rust targets (`aarch64-apple-ios`, `aarch64-apple-ios-sim`, `x86_64-apple-ios`, `aarch64-apple-darwin`, `x86_64-apple-darwin`).
2.  Compile the Rust crate for each target.
3.  Create universal binaries for iOS simulator and macOS using `lipo`.
4.  Package everything into `RustFFI.xcframework`.

## Usage

Add the package to your Swift project via Swift Package Manager. Then, use the `Api` struct to generate images.

### Basic Example

```swift
import Api

// Generate an image
let (status, body) = await Api.zImageTurbo(
    token: "your-api-token",
    prompt: "a red apple on a wooden table"
)

if status == 200 {
    print("Success! Received \(body.count) bytes of data.")
    // Process body (e.g., decode JSON or save image)
} else {
    print("Failed with status: \(status)")
}
```

### Cancellation

The API supports cancellation. If the calling task is cancelled, the underlying HTTP request is aborted.

```swift
let task = Task {
    await Api.zImageTurbo(
        token: "your-api-token",
        prompt: "a complex scene..."
    )
}

// Cancel after 100ms
try await Task.sleep(nanoseconds: 100_000_000)
task.cancel()

let (status, _) = await task.value
// status will be 0 if cancelled
```

## API Reference

### `Api.zImageTurbo(token:prompt:)`

Sends a request to the `ZImageTurbo` model on femi.market.

-   **Parameters**:
    -   `token`: The Bearer token for authentication. If empty or missing, the server may return a 401 error.
    -   `prompt`: The text prompt for image generation.
-   **Returns**: A tuple `(status: UInt16, body: Data)`.
    -   `status`: The HTTP status code (e.g., 200, 401, 402). Returns `0` if the request was cancelled or failed at the transport level.
    -   `body`: The response body as `Data`. Empty if the request failed or was cancelled.

## Testing

Run the tests using Swift Package Manager:

```bash
swift test
```

The test suite (`Tests/ApiTests/ApiTests.swift`) includes:
-   Successful requests with various prompts (empty, unicode).
-   Error handling for unfunded accounts (402) and missing tokens (401).
-   Cancellation behavior verification.

## Non-Obvious Conventions

-   **Memory Management**: The Rust function `rust_ffi_z_image_turbo` returns a pointer to heap-allocated memory. The Swift wrapper automatically handles deallocation using `.free` when creating the `Data` object. Do not manually free the pointer returned by the FFI function.
-   **Cancellation Mechanism**: Cancellation is implemented via a shared atomic flag. The Swift side sets this flag when the task is cancelled, and the Rust side polls it every 10ms. This is a lightweight, cross-platform approach that avoids complex FFI callbacks.
-   **Runtime Initialization**: The Rust side uses `OnceLock` to initialize the Tokio runtime and `reqwest::Client` only once. This ensures efficient resource usage across multiple calls.
-   **Error Handling**: Transport errors (e.g., network failure) result in a status code of `0`. HTTP errors (e.g., 4xx, 5xx) return the actual HTTP status code.