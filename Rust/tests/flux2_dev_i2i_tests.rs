#![cfg(not(target_arch = "wasm32"))]

#[path = "api_tests.rs"] mod api_tests;
use api_tests::*;
use base64::Engine;
use rust_ffi::{rust_ffi_flux2_dev_i2i, FALLBACK_IMAGE, TOPUP_IMAGE};

#[test]
fn funded_user_returns_real_image() {
    let image_bytes = include_bytes!("flux2_dev_i2i/cactus_man.png");
    let image_b64 = base64::engine::general_purpose::STANDARD.encode(image_bytes);
    let u = cs(&test_user()); let pw = cs(TEST_PASSWORD);
    let i = cs(&image_b64); let p = cs("place him in a sunlit room");
    let mut len = 0usize;
    let r = rust_ffi_flux2_dev_i2i(u.as_ptr(), pw.as_ptr(), i.as_ptr(), p.as_ptr(), std::ptr::null(), &mut len);
    let img = unsafe { take(r, len) };
    assert!(!img.is_empty());
    assert_ne!(img, FALLBACK_IMAGE);
    assert_ne!(img, TOPUP_IMAGE);
}
