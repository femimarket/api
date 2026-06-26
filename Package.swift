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
            dependencies: ["RustFFI"]
        ),
        .testTarget(
            name: "ApiTests",
            dependencies: ["Api"]
        ),
    ],
    swiftLanguageModes: [.v6]
)
