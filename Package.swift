// swift-tools-version: 6.3
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "Api",
    platforms: [
        .iOS(.v15),
        .macOS(.v12),
    ],
    products: [
        .library(
            name: "Api",
            targets: ["Api"] 
        ),
    ],
    targets: [
        .binaryTarget(
            name: "RustFFI",
            path: "RustFFI.xcframework"
        ),
        .target(
            name: "Api",
            dependencies: ["RustFFI"],
            resources: [
                .process("Lib/fallback.png"),
                .process("Lib/topup.jpg"),
                .process("Lib/topup-video.mp4"),
                .process("Lib/could-not-generate.mp4"),
            ]
        ),
        .testTarget(
            name: "ApiTests",
            dependencies: ["Api"],
            resources: [
                .process("Qwen3AsrFlash/test_audio.mp3"),
                .process("Flux2DevI2I/cactus_man.png"),
                .process("Flux2KleinI2I/pink_tone_chair.png"),
                .process("Flux2KleinI2I/car_interior_white.jpeg"),
                .process("Ltx2_3A2V/ltx_audio.mp3"),
                .process("Ltx2_3A2V/man-walking.png"),
            ]
        ),
    ],
    swiftLanguageModes: [.v6]
)
