# Kotlin API

A multiplatform Kotlin library providing access to various AI models (image generation, text chat, audio transcription) via a unified API. The library abstracts the underlying implementation details, allowing the same Kotlin code to run on Android (via JNI to Rust) and Web (via WebAssembly).

## Features

- **Multiplatform Support**: Runs on Android and Web (Wasm).
- **Unified Interface**: Common `expect`/`actual` pattern hides platform-specific FFI details.
- **Cancellation Support**: Android implementation supports coroutine cancellation for long-running native calls.
- **Model Support**:
  - **Image Generation**: `zImageTurbo`, `nanoBanana2`, `flux2Pro`, `flux2DevI2I`, `flux2KleinI2I`.
  - **Video Generation**: `ltx2_3a2v` (Image + Audio to Video).
  - **Text Chat**: `qwen3_6_35b_a3b` (Qwen 3.6 35B A3B).
  - **Audio Transcription**: `qwen3AsrFlash` (ASR Flash).

## Architecture

The project uses Kotlin Multiplatform (KMP) to share business logic while delegating native interactions to platform-specific implementations.

### Directory Structure

- `kotlinapi/src/commonMain/`: Contains the shared API definitions (`expect` functions) and data classes.
- `kotlinapi/src/androidMain/`: Android-specific implementation using JNI to call a Rust library (`rust_ffi`).
- `kotlinapi/src/webMain/`: Web-specific implementation using Kotlin/JS interop to call a WebAssembly module (`rust_ffi`).
- `webDemo/`: A demo application running in the browser to showcase functionality.

### Key Files

- **`kotlinapi/src/commonMain/kotlin/market/femi/kotlinapi/`**:
  - `ZImageTurbo.kt`, `NanoBanana2.kt`, `Flux2Pro.kt`, etc.: Define the `expect` suspend functions.
  - `Qwen3_6_35b_a3b.kt`: Defines `ChatMessage` and `Role` data classes used for chat interactions.
  - `Qwen3AsrFlash.kt`: Defines the audio transcription interface.

- **`kotlinapi/src/androidMain/kotlin/market/femi/kotlinapi/`**:
  - `FemiApiJvm.kt`: Declares `external` functions that map to the native Rust library.
  - `CancelFlag.kt`: Implements cancellation logic for Android by passing a pointer to a native byte buffer. If the coroutine is cancelled, the byte is flipped to signal the native code to abort.
  - `*.kt` files (e.g., `ZImageTurbo.kt`): Implement the `actual` functions, wrapping the JNI calls with cancellation support.

- **`kotlinapi/src/webMain/kotlin/market/femi/kotlinapi/`**:
  - `RustFfi.kt`: Declares the WebAssembly module interface using `@JsModule("rust_ffi")`.
  - `*.kt` files: Implement the `actual` functions, awaiting Promises from the WASM module and converting results to Kotlin types.

- **`webDemo/src/wasmJsMain/kotlin/Main.kt`**:
  - A simple browser-based demo that demonstrates calling the chat and image generation APIs.

## Installation

### Prerequisites

- Kotlin 1.9+ (or compatible version with KMP support)
- Android SDK (for Android builds)
- Node.js and npm (for Web/Wasm builds)
- Rust toolchain (if building the native/WASM backend from source)

### Adding to Your Project

Include the library in your `build.gradle.kts` (or `build.gradle`) for the common module:

```kotlin
dependencies {
    implementation("market.femi:kotlinapi:1.0.0") // Replace with actual version
}
```

For Android, ensure you have the necessary NDK setup. For Web, ensure your build system supports Kotlin/JS with Wasm.

## Usage

### Image Generation

Generate an image from a text prompt:

```kotlin
import market.femi.kotlinapi.zImageTurbo

val user = "your_username"
val pass = "your_password"
val prompt = "a futuristic city at sunset"

val imageBytes = zImageTurbo(user, pass, prompt)
// imageBytes contains the raw image data (PNG/JPEG)
```

### Image-to-Image

Transform an existing image:

```kotlin
import market.femi.kotlinapi.flux2DevI2I

val imageB64 = "base64_encoded_image_string"
val prompt = "make it cyberpunk"

val resultBytes = flux2DevI2I(user, pass, imageB64, prompt)
```

### Text Chat

Interact with the Qwen model:

```kotlin
import market.femi.kotlinapi.qwen3_6_35b_a3b
import market.femi.kotlinapi.ChatMessage
import market.femi.kotlinapi.Role

val messages = listOf(
    ChatMessage(Role.User, "Hello, how are you?")
)

val response = qwen3_6_35b_a3b(user, pass, messages)
val lastReply = response.lastOrNull()?.content
```

### Audio Transcription

Transcribe audio data:

```kotlin
import market.femi.kotlinapi.qwen3AsrFlash

val audioData: ByteArray = /* your audio bytes */
val transcript = qwen3AsrFlash(user, pass, audioData)
```

## Building

### Android

1. Ensure the Rust native library (`librust_ffi.so`) is built for Android targets (arm64-v8a, armeabi-v7a, x86_64).
2. The library is loaded automatically via `System.loadLibrary("rust_ffi")` in `FemiApiJvm.kt`.
3. Build the Android module using Gradle:
   ```bash
   ./gradlew :kotlinapi:assembleRelease
   ```

### Web (Wasm)

1. Build the Rust WASM module. Ensure it exports the functions declared in `RustFfi.kt`.
2. The Kotlin/JS compiler will link against the generated WASM module.
3. Build the web module:
   ```bash
   ./gradlew :kotlinapi:wasmJsBrowserDistribution
   ```

### Running the Demo

To run the web demo:

1. Build the WASM module.
2. Run the demo server:
   ```bash
   ./gradlew :webDemo:wasmJsBrowserRun
   ```
3. Open the provided URL in your browser. The demo will auto-generate a user account and allow you to test chat and image generation.

## Cancellation

On Android, long-running native calls can be cancelled by cancelling the coroutine. This is implemented in `CancelFlag.kt` by passing a pointer to a native byte buffer. If the coroutine is cancelled, the byte is set to `1`, signaling the native Rust code to abort the operation early.

On Web, cancellation is handled by the JavaScript runtime and the WASM module's promise resolution.

## Non-Obvious Conventions

- **Base64 Encoding**: Image and audio inputs are typically passed as Base64 strings in the API calls, except for `qwen3AsrFlash` which takes raw `ByteArray` (and encodes it internally).
- **Chat Messages**: The `ChatMessage` class uses `Role` enum with `@SerialName` annotations to match the wire format expected by the backend.
- **Error Handling**: In chat responses, if the backend fails to respond, the library returns the original messages plus a fallback assistant message: `"Could not respond"`.
- **User Credentials**: The demo uses a simple `user` and `pass` string. In production, these should be handled securely. The demo auto-funds new users with 50 credits.