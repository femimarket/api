#![cfg(not(target_arch = "wasm32"))]

#[path = "api_tests.rs"] mod api_tests;
use api_tests::*;
use base64::Engine;
use rust_ffi::{rust_ffi_flux2_klein_i2i, FALLBACK_IMAGE, TOPUP_IMAGE};

#[test]
fn funded_user_returns_real_image() {
    let chair = include_bytes!("flux2_klein_i2i/pink_tone_chair.png");
    let car = include_bytes!("flux2_klein_i2i/car_interior_white.jpeg");
    let chair_b64 = base64::engine::general_purpose::STANDARD.encode(chair);
    let car_b64 = base64::engine::general_purpose::STANDARD.encode(car);
    let u = cs(&test_user()); let pw = cs(TEST_PASSWORD);
    let i1 = cs(&chair_b64); let i2 = cs(&car_b64);
    let p = cs("place the chair into the car interior");
    let mut len = 0usize;
    let r = rust_ffi_flux2_klein_i2i(u.as_ptr(), pw.as_ptr(), i1.as_ptr(), i2.as_ptr(), p.as_ptr(), std::ptr::null(), &mut len);
    let img = unsafe { take(r, len) };
    assert!(!img.is_empty());
    assert_ne!(img, FALLBACK_IMAGE);
    assert_ne!(img, TOPUP_IMAGE);
}
