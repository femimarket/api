# Femi API

A cross-platform (iOS, macOS, Android, Web) Kotlin Multiplatform library providing access to generative AI endpoints (image, video, audio, and LLM) via a shared Rust FFI core.

The library exposes a unified `market.femi.api` package with `expect`/`actual` declarations. It handles network communication, fallback assets, cancellation, and local file storage (XMP metadata) entirely within the Rust layer, exposing clean Kotlin/Swift APIs to the host application.

## Architecture

The project is structured into three main layers:

1.  **Rust Core (`Rust/`)**: The single source of truth. It contains the business logic, network clients, and FFI bindings.
    *   **Native (iOS/Android)**: Uses `jni` (Android) and C-ABI (iOS/macOS) to expose functions.
    *   **WebAssembly**: Uses `wasm-bindgen` to expose functions to JavaScript/Kotlin/Wasm.
    *   **ProjectService**: Manages local file storage. On Apple, it uses `xmp_toolkit` for XMP metadata. On Android/Web, it uses a pure-Rust `xmpkit_body` module for bytes-in/bytes-out XMP manipulation.
2.  **Kotlin Multiplatform (`Kmp/`)**:
    *   `commonMain`: Defines `expect` functions and data classes (e.g., `ChatMessage`, `ProjectService`).
    *   `androidMain`: Implements `actual` functions using JNI via `FemiApiJvm`.
    *   `webMain`: Implements `actual` functions using `wasm-bindgen` generated JS interop.
3.  **Swift Package (`Sources/`)**:
    *   Wraps the `RustFFI.xcframework` with Swift async/await APIs.
    *   Handles memory management (deallocating raw pointers returned from Rust).

## Supported Platforms

*   **iOS**: iOS 15+ (via Swift Package Manager)
*   **macOS**: macOS 12+ (via Swift Package Manager)
*   **Android**: API 21+ (via AAR/JAR with JNI libs)
*   **Web**: Kotlin/Wasm (via npm package or embedded klib)

## Installation & Build

### Prerequisites

*   **Rust**: `rustup` installed.
*   **Android NDK**: Required for Android targets.
*   **wasm-bindgen**: Installed automatically by the build script if missing.
*   **Xcode**: Required for Apple targets.

### Build Script

Run the following command from the repository root to build all artifacts:

```bash
./build-rust.sh
```

This script performs the following:
1.  Installs necessary Rust targets (`aarch64-apple-ios`, `aarch64-apple-darwin`, `wasm32-unknown-unknown`, and Android ABIs).
2.  Builds the Rust crate for each platform.
3.  **Apple**: Creates `RustFFI.xcframework` from the static libraries.
4.  **Android**: Copies `.so` libraries into `Kmp/api/src/androidMain/jniLibs/` organized by ABI (`arm64-v8a`, `armeabi-v7a`, `x86_64`, `x86`).
5.  **Web**:
    *   Runs `wasm-bindgen` on the `.wasm` output.
    *   **Inlines the `.wasm`** into the generated JS glue code as base64. This creates a single self-contained ES module (`pkg/rust_ffi.js`) that does not require fetching a separate `.wasm` file at runtime.
    *   Encodes the resulting JS module into a base64 Kotlin constant (`RustFfiBundle.kt`) embedded in the KMP klib.

### Swift Package Manager (Apple)

The `Package.swift` references the pre-built `RustFFI.xcframework`. Ensure you have run `build-rust.sh` before building the Swift package.

```swift
// Package.swift
let package = Package(
    name: "Api",
    platforms: [
        .iOS(.v15),
        .macOS(.v12),
    ],
    targets: [
        .binaryTarget(
            name: "RustFFI",
            path: "RustFFI.xcframework"
        ),
        .target(
            name: "Api",
            dependencies: ["RustFFI"]
        ),
    ]
)
```

### Android (Gradle)

The Android artifacts are placed in `Kmp/api/src/androidMain/jniLibs/`. Ensure your Gradle configuration includes the `Kmp/api` module and that the `jniLibs` source set is correctly configured (standard for KMP projects).

### Web (Kotlin/Wasm)

The Web artifacts are embedded in the Kotlin klib. No external npm dependencies are required at runtime for the FFI module itself. The `pkg/` directory contains the intermediate build artifacts, but the final bundle is the `RustFfiBundle.kt` file.

## Usage

### Kotlin Multiplatform

#### Image Generation

```kotlin
import market.femi.api.*

// ZImageTurbo
val imageBytes = zImageTurbo("a red apple on a wooden table")

// Flux2Pro
val imageBytes = flux2Pro("a red apple on a wooden table")

// Flux2DevI2I (Image-to-Image)
val imageBytes = flux2DevI2I(imageB64 = "...", prompt = "place him in a sunlit room")

// Flux2KleinI2I (Image-to-Image with two images)
val imageBytes = flux2KleinI2I(imageB64 = "...", image2B64 = "...", prompt = "place the chair into the car interior")

// NanoBanana2
val imageBytes = nanoBanana2("a red apple on a wooden table")
```

#### Video Generation

```kotlin
import market.femi.api.*

// Ltx2_3A2V (Image + Audio to Video)
val videoBytes = ltx2_3a2v(imageB64 = "...", audioB64 = "...", prompt = "the man walks forward in time with the music")

// Ltx2_3A2V ComfyUI (Direct to ComfyUI Cloud)
val videoBytes = ltx2_3a2v_comfyui(
    comfyKey = "your_comfyui_api_key",
    imageB64 = "...",
    audioB64 = "...",
    prompt = "the man walks forward in time with the music"
)
```

#### Audio & Text

```kotlin
import market.femi.api.*

// Qwen3 ASR Flash (Audio to Lyrics)
val lyrics = qwen3AsrFlash(audioBytes)

// Qwen3 Chat (LLM)
val messages = listOf(
    ChatMessage(Role.User, "say hi in one word")
)
val response = qwen3_6_35b_a3b(messages)
// response.last.content contains the assistant's reply

// Qwen3 Prompt Generation (Authless)
val prompt = qwen3_6_35b_a3b_0GenMusicVideoPrompt()
```

#### Project Service (Local Storage)

The `ProjectService` manages local files in a sandboxed directory. Metadata (prompt, model, subject, etc.) is embedded as XMP tags in the file bytes.

```kotlin
import market.femi.api.*

// Initialize on Android (optional, handled automatically by ProjectServiceInitializer)
// ProjectService.initDocuments(context.filesDir.absolutePath)

// Save a file with metadata
ProjectService.saveFile(
    data = imageBytes,
    named = "my_image.png",
    prompt = "a red apple",
    model = "flux-2",
    subject = listOf("apple", "fruit")
)

// Read metadata
val prompt = ProjectService.getPrompt("my_image.png")
val model = ProjectService.getModel("my_image.png")
val subjects = ProjectService.getSubject("my_image.png")

// Like/Unlike
ProjectService.like("my_image.png", liked = true)
val isLiked = ProjectService.getLike("my_image.png")

// Get all generated files
val files = ProjectService.getAllGenerations()

// Character Cast (In-memory state)
ProjectService.setCharacterCast("hero.png", "villain.png")
val cast = ProjectService.getCharacterCast() // Pair<String, String>?
```

#### Cancellation

All suspend functions support cancellation. If the coroutine is cancelled, the native call is aborted, and a fallback asset (image/video) or error string is returned.

```kotlin
val task = launch {
    try {
        val image = zImageTurbo("long generation...")
    } catch (e: CancellationException) {
        // Handled automatically by the FFI layer; returns fallback image
    }
}
task.cancel()
```

### Swift (iOS/macOS)

```swift
import Api

// Image Generation
let image = await Api.zImageTurbo(prompt: "a red apple on a wooden table")

// Video Generation
let video = await Api.ltx2_3a2v(
    image: imageData,
    audio: audioData,
    prompt: "the man walks forward in time with the music"
)

// Chat
let messages: [(role: Api.Role, content: String)] = [
    (role: .user, content: "say hi in one word")
]
let response = await Api.qwen3_6_35b_a3b(messages: messages)

// Project Service
ProjectService.saveFile(imageData, named: "my_image.png", prompt: "a red apple")
let prompt = ProjectService.getPrompt("my_image.png")
```

## Key Files

*   `build-rust.sh`: Master build script for all platforms.
*   `Rust/src/lib.rs`: Rust entry point, defines fallback assets and HTTP client.
*   `Rust/src/api/`: Individual endpoint implementations (e.g., `flux2_pro.rs`, `qwen3_6_35b_a3b.rs`).
*   `Rust/src/project_service/`: Local storage logic, split by backend (`apple.rs`, `android.rs`, `wasm.rs`).
*   `Kmp/api/src/commonMain/kotlin/market/femi/api/`: Kotlin `expect` declarations.
*   `Kmp/api/src/androidMain/kotlin/market/femi/api/`: Kotlin `actual` implementations for Android.
*   `Kmp/api/src/webMain/kotlin/market/femi/api/`: Kotlin `actual` implementations for Web.
*   `Sources/Api/`: Swift wrappers for the Rust FFI.

## Non-Obvious Conventions

*   **Cancellation**: On native platforms, cancellation is implemented via a shared memory flag (`cancel_flag`). The Kotlin/Swift side allocates a 1-byte buffer and passes its address. The Rust side polls this address in a background task. If the flag is set to `1`, the request is aborted early.
*   **WebAssembly Inlining**: The Web build process inlines the `.wasm` binary into the JavaScript glue code as base64. This eliminates the need for a separate `.wasm` file download. The Kotlin/Wasm side decodes this base64 at runtime and instantiates the WebAssembly module from memory.
*   **XMP Metadata**: On Apple, XMP metadata is written using the Adobe XMP Toolkit. On Android and Web, a pure-Rust implementation (`xmpkit_body`) is used to manipulate XMP tags directly in the byte stream. This ensures consistent metadata handling across platforms without platform-specific XMP libraries on Android/Web.
*   **Fallback Assets**: If a network request fails or is cancelled, the Rust FFI returns embedded fallback assets (images/videos) or error strings. This ensures the UI always has something to display.
*   **ProjectService Initialization**: On Android, `ProjectService` is automatically initialized via `androidx.startup` to point to the app's `filesDir`. On Apple and Web, the directory is resolved automatically by Rust (`dirs::document_dir` or OPFS). Apps can override the Android path by calling `ProjectService.initDocuments(path)` manually.