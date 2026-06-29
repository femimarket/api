# Api

`Api` is a Swift Package that provides a unified, async-first interface to a suite of AI models (image generation, image-to-image, video generation, speech-to-text, and LLM chat) via a Rust FFI layer. The Rust layer handles HTTP communication with `femi.market`, manages a shared Tokio runtime, and exposes a stable C ABI. The Swift layer wraps these FFI calls, handles structured cancellation, parses JSON responses, and returns user-friendly fallback assets on failure or payment errors.

## Architecture & Key Files

The project is split into a Rust FFI crate and a Swift wrapper package:

| Path | Purpose |
|------|---------|
| `Package.swift` | Swift Package manifest. Defines the `Api` target, links against `RustFFI.xcframework`, bundles fallback resources, and enforces Swift 6 language mode. |
| `build-rust.sh` | Build script that compiles the Rust crate for iOS (device + simulator) and macOS, creates universal binaries via `lipo`, and packages everything into `RustFFI.xcframework`. |
| `Rust/src/lib.rs` | Rust entry point. Initializes a shared `tokio::Runtime` and `reqwest::Client` via `OnceLock` to avoid per-call overhead. Delegates to model-specific modules. |
| `Rust/include/RustFFI/RustFFI.h` | C header exposing all FFI functions. All functions share an identical signature pattern. |
| `Rust/src/*.rs` | Rust implementations for each model endpoint. Each POSTs to `https://femi.market/api`, handles cancellation polling, and returns heap-allocated response bytes. |
| `RustFFI.xcframework/` | Pre-built (or script-generated) framework containing compiled libraries and headers for iOS and macOS. |
| `Sources/Api/Api.swift` | Enum holding fallback resources (`fallback.png`, `topup.jpg`, `could-not-generate.mp4`, `topup-video.mp4`). |
| `Sources/Api/*.swift` | Swift async wrappers. Each model has a dedicated file (e.g., `Flux2Pro.swift`, `Qwen3_6_35b_a3b.swift`). Handles C interop, cancellation, and response parsing. |
| `Tests/ApiTests/*.swift` | Swift Testing framework suite. Validates unfunded/missing credential fallbacks, cancellation behavior, and funded-user success paths. |

## Prerequisites

- macOS (required for building the xcframework and running tests)
- Rust toolchain (`cargo`, `rustup`)
- Swift 6.3+ toolchain
- `xcodebuild` (bundled with Xcode, required for xcframework creation)

## Building & Packaging

1. Ensure the build script is executable:
   ```bash
   chmod +x build-rust.sh
   ```
2. Run the build script from the repository root:
   ```bash
   ./build-rust.sh
   ```
   This will:
   - Add required `rustup` targets (`aarch64-apple-ios`, `aarch64-apple-ios-sim`, `x86_64-apple-ios`, `aarch64-apple-darwin`, `x86_64-apple-darwin`)
   - Compile `librust_ffi.a` for each target in `--release` mode
   - Create universal iOS simulator and macOS binaries using `lipo`
   - Generate `RustFFI.xcframework` in the repository root

The Swift package automatically consumes the framework. No additional configuration is needed.

## Usage

Import the package in your Swift code. All public methods are `async` and return `Data` or `String`. They require HTTP Basic Auth credentials (`user` and `password`).

### Image Generation
```swift
let img = await Api.flux2Pro(user: "your_user", password: "your_pass", prompt: "a red apple on a wooden table")
```

### Image-to-Image
```swift
let img = await Api.flux2DevI2I(user: "...", password: "...", image: inputData, prompt: "...")
let img = await Api.flux2KleinI2I(user: "...", password: "...", image: img1, image2: img2, prompt: "...")
```

### Video Generation
```swift
let video = await Api.ltx2_3a2v(user: "...", password: "...", image: imgData, audio: audioData, prompt: "...")
```

### Speech-to-Text (Lyrics)
```swift
let lyrics = await Api.qwen3AsrFlash(user: "...", password: "...", audio: audioData)
```

### LLM Chat
```swift
let messages = await Api.qwen3_6_35b_a3b(
    user: "...", 
    password: "...", 
    messages: [(role: .user, content: "say hi in one word")]
)
// Returns the original messages + the assistant's reply
```

## Cancellation & Error Handling

### Task Cancellation
Each Swift wrapper uses `withTaskCancellationHandler` to allocate a cancellation flag pointer. When the Swift `Task` is cancelled, the flag is set to `1`. The Rust side polls this flag via `AtomicU8` every 10ms and aborts the HTTP request if triggered, returning a status of `0` and an empty body. Swift then returns the appropriate fallback asset.

### Error & Fallback Behavior
- **Network/Transport Failures**: Returns `Api.fallbackImage` / `Api.fallbackVideo` (or `"Could not process lyrics"` / `"Could not respond"` for text endpoints).
- **HTTP 402 (Payment Required)**: Returns `Api.topupImage` / `Api.topupVideo` (or `"Top up to transcribe lyrics"` for ASR).
- **HTTP 200 with valid payload**: Parses the JSON response, extracts the base64-encoded file/lyrics/messages from the `action` object, and returns the decoded data.
- **Memory Management**: Rust returns heap-allocated bytes. Swift safely transfers ownership using `Data(bytesNoCopy:count:deallocator: .free)`, avoiding unnecessary copies and ensuring proper deallocation.

## Testing

Tests use the Swift Testing framework and require live credentials to verify funded-user behavior.

Run tests from the repository root:
```bash
API_USER="your_username" API_PASSWORD="your_password" swift test
```

Test coverage includes:
- Unfunded/missing credentials returning correct fallbacks across all endpoints
- Task cancellation resolving in <1s and returning fallbacks
- Funded users returning actual generated content (images, videos, lyrics, chat replies)
- Edge cases: empty prompts, empty audio, unicode prompts, empty message arrays

## Non-Obvious Conventions

- **Uniform FFI Signature**: Every Rust function follows `(user, password, inputs..., cancel_flag, out_status, out_len) -> *mut u8`. This simplifies Swift interop and allows consistent wrapper generation.
- **Shared Runtime & Client**: `Rust/src/lib.rs` uses `OnceLock` to initialize a single `tokio::Runtime` (2 workers) and `reqwest::Client` (600s timeout). This avoids per-call initialization overhead and connection churn.
- **Detached Tasks**: Swift wrappers run the FFI call on `Task.detached(priority: .userInitiated)` to prevent main-thread blocking and ensure cancellation propagates correctly.
- **JSON Payload Structure**: The Rust layer always wraps requests in a standard envelope:
  ```json
  {
    "id": "<uuid>",
    "user_id": "",
    "action": { "type": "<ModelType>", ... },
    "status": "Pending",
    "credit": 0
  }
  ```
- **Swift 6 Language Mode**: The package explicitly requires `swiftLanguageModes: [.v6]` in `Package.swift` to enforce strict concurrency checking.