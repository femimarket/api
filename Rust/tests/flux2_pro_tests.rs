#![cfg(not(target_arch = "wasm32"))]

#[path = "api_tests.rs"] mod api_tests;
use api_tests::*;
use rust_ffi::rust_ffi_flux2_pro;

#[test]
fn funded_user_returns_real_image() {
    let u = cs(&test_user()); let pw = cs(TEST_PASSWORD);
    let p = cs("a red apple on a wooden table");
    let mut len = 0usize;
    let r = rust_ffi_flux2_pro(u.as_ptr(), pw.as_ptr(), p.as_ptr(), std::ptr::null(), &mut len);
    assert!(!unsafe { take(r, len) }.is_empty());
}
