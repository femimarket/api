#!/usr/bin/env bash
# Builds the Rust FFI crate for iOS device + simulator, plus macOS, then
# packages everything into RustFFI.xcframework consumed by SwiftPM.
set -euo pipefail

export PATH="$HOME/.cargo/bin:$PATH"

cd "$(dirname "$0")/Rust"

TARGETS=(
  aarch64-apple-ios
  aarch64-apple-ios-sim
  x86_64-apple-ios
  aarch64-apple-darwin
  x86_64-apple-darwin
)

rustup target add "${TARGETS[@]}"

for t in "${TARGETS[@]}"; do
  cargo build --release --target "$t"
done

mkdir -p target/ios-sim-universal/release
lipo -create \
  target/aarch64-apple-ios-sim/release/librust_ffi.a \
  target/x86_64-apple-ios/release/librust_ffi.a \
  -output target/ios-sim-universal/release/librust_ffi.a

mkdir -p target/macos-universal/release
lipo -create \
  target/aarch64-apple-darwin/release/librust_ffi.a \
  target/x86_64-apple-darwin/release/librust_ffi.a \
  -output target/macos-universal/release/librust_ffi.a

rm -rf ../RustFFI.xcframework

xcodebuild -create-xcframework \
  -library target/aarch64-apple-ios/release/librust_ffi.a       -headers include \
  -library target/ios-sim-universal/release/librust_ffi.a       -headers include \
  -library target/macos-universal/release/librust_ffi.a         -headers include \
  -output ../RustFFI.xcframework

echo "✓ RustFFI.xcframework built"
