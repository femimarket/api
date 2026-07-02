#!/usr/bin/env bash
# Builds the Rust FFI crate for every platform the workspace consumes:
#
#   RustFFI.xcframework                        (SwiftPM binaryTarget path)
#   Kmp/kotlinapi/src/androidMain/jniLibs/*    (System.loadLibrary("rust_ffi"))
#   Rust/pkg/                                  (npm("rust_ffi", ...) in build.gradle.kts)
#
# NDK linker/env vars for the android targets come from Rust/.cargo/config.toml.
# Requires `wasm-bindgen` CLI; installed on demand if missing.
set -euo pipefail

export PATH="$HOME/.cargo/bin:$PATH"

# xmp_toolkit's C++ code emits `__chkstk_darwin`, only present on iOS 12+ /
# macOS 10.14+. Match Swift Package's Package.swift platform floors here.
export IPHONEOS_DEPLOYMENT_TARGET=14.0
export MACOSX_DEPLOYMENT_TARGET=11.0

ROOT="$(cd "$(dirname "$0")" && pwd)"
cd "$ROOT/Rust"

APPLE_TARGETS=(
  aarch64-apple-ios
  aarch64-apple-ios-sim
  x86_64-apple-ios
  aarch64-apple-darwin
  x86_64-apple-darwin
)

ANDROID_PAIRS=(
  aarch64-linux-android:arm64-v8a
  armv7-linux-androideabi:armeabi-v7a
  x86_64-linux-android:x86_64
  i686-linux-android:x86
)

rustup target add "${APPLE_TARGETS[@]}"
rustup target add wasm32-unknown-unknown
for pair in "${ANDROID_PAIRS[@]}"; do
  rustup target add "${pair%%:*}"
done

command -v wasm-bindgen >/dev/null || cargo install wasm-bindgen-cli

########################################
# Apple
########################################
echo ">> apple targets"
for t in "${APPLE_TARGETS[@]}"; do
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

rm -rf "$ROOT/RustFFI.xcframework"
xcodebuild -create-xcframework \
  -library target/aarch64-apple-ios/release/librust_ffi.a       -headers include \
  -library target/ios-sim-universal/release/librust_ffi.a       -headers include \
  -library target/macos-universal/release/librust_ffi.a         -headers include \
  -output "$ROOT/RustFFI.xcframework"

########################################
# Android
########################################
echo ">> android targets"
JNI_LIBS="$ROOT/Kmp/kotlinapi/src/androidMain/jniLibs"
rm -rf "$JNI_LIBS"
for pair in "${ANDROID_PAIRS[@]}"; do
  triple="${pair%%:*}"
  abi="${pair##*:}"
  cargo build --release --target "$triple"
  mkdir -p "$JNI_LIBS/$abi"
  cp "target/$triple/release/librust_ffi.so" "$JNI_LIBS/$abi/librust_ffi.so"
done

########################################
# WebAssembly
########################################
echo ">> wasm target"
cargo build --release --target wasm32-unknown-unknown
rm -rf pkg
wasm-bindgen "target/wasm32-unknown-unknown/release/rust_ffi.wasm" \
  --out-dir pkg \
  --target web

echo
echo "✓ RustFFI.xcframework      → $ROOT/RustFFI.xcframework"
echo "✓ android jniLibs          → $JNI_LIBS"
echo "✓ wasm-bindgen pkg         → $ROOT/Rust/pkg"
