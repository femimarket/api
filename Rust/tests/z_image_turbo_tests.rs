#![cfg(not(target_arch = "wasm32"))]

#[path = "api_tests.rs"] mod api_tests;
use api_tests::*;
use rust_ffi::rust_ffi_z_image_turbo;

#[test]
fn funded_user_returns_real_image() {
    let u = cs(&test_user()); let pw = cs(TEST_PASSWORD);
    let p = cs("a red apple on a wooden table");
    let mut len = 0usize;
    let r = rust_ffi_z_image_turbo(u.as_ptr(), pw.as_ptr(), p.as_ptr(), std::ptr::null(), &mut len);
    assert!(!unsafe { take(r, len) }.is_empty());
}

#[test]
fn empty_prompt_returns_real_image() {
    let u = cs(&test_user()); let pw = cs(TEST_PASSWORD);
    let p = cs("");
    let mut len = 0usize;
    let r = rust_ffi_z_image_turbo(u.as_ptr(), pw.as_ptr(), p.as_ptr(), std::ptr::null(), &mut len);
    assert!(!unsafe { take(r, len) }.is_empty());
}

#[test]
fn unicode_prompt_returns_real_image() {
    let u = cs(&test_user()); let pw = cs(TEST_PASSWORD);
    let p = cs("日本語の猫 🐈 \"quoted\" \\backslash");
    let mut len = 0usize;
    let r = rust_ffi_z_image_turbo(u.as_ptr(), pw.as_ptr(), p.as_ptr(), std::ptr::null(), &mut len);
    assert!(!unsafe { take(r, len) }.is_empty());
}
