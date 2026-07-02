#![cfg(not(target_arch = "wasm32"))]

#[path = "api_tests.rs"] mod api_tests;
use api_tests::*;
use base64::Engine;
use rust_ffi::{rust_ffi_ltx2_3a2v, FALLBACK_VIDEO, TOPUP_VIDEO};

#[test]
fn funded_user_returns_real_video() {
    let audio = include_bytes!("ltx2_3a2v/ltx_audio.mp3");
    let image = include_bytes!("ltx2_3a2v/man-walking.png");
    let audio_b64 = base64::engine::general_purpose::STANDARD.encode(audio);
    let image_b64 = base64::engine::general_purpose::STANDARD.encode(image);
    let u = cs(&test_user()); let pw = cs(TEST_PASSWORD);
    let i = cs(&image_b64); let a = cs(&audio_b64);
    let p = cs("the man walks forward in time with the music");
    let mut len = 0usize;
    let r = rust_ffi_ltx2_3a2v(u.as_ptr(), pw.as_ptr(), i.as_ptr(), a.as_ptr(), p.as_ptr(), std::ptr::null(), &mut len);
    let video = unsafe { take(r, len) };
    assert!(!video.is_empty());
    assert_ne!(video, FALLBACK_VIDEO);
    assert_ne!(video, TOPUP_VIDEO);
}
