//! Shared helpers for the endpoint integration tests. Mirrors the top-level
//! `testUser` / `testPassword` in `Api2/Tests/ApiTests/ApiTests.swift`.
//!
//! Cargo compiles this file as its own test binary (with zero tests) in
//! addition to being pulled into every endpoint test file via
//! `#[path = "api_tests.rs"] mod api_tests;` — that's the only way to share
//! helpers across integration test crates without a subdirectory module.

#![cfg(not(target_arch = "wasm32"))]
#![allow(dead_code)]

use std::ffi::CString;

/// Fresh per-run username. The server auto-funds any new account with 50
/// credits, so this gives every test binary a funded user for its lifetime.
pub fn test_user() -> String {
    format!("funded-test-{}", uuid::Uuid::now_v7())
}

pub const TEST_PASSWORD: &str = "abc123";

pub fn cs(s: &str) -> CString { CString::new(s).unwrap() }

/// Reclaims the heap-allocated bytes an FFI call returned into an owned Vec.
pub unsafe fn take(ptr: *mut u8, len: usize) -> Vec<u8> {
    if ptr.is_null() || len == 0 { return Vec::new(); }
    unsafe { Box::from_raw(std::slice::from_raw_parts_mut(ptr, len)).to_vec() }
}
