#![cfg(not(target_arch = "wasm32"))]

#[path = "api_tests.rs"] mod api_tests;
use api_tests::*;
use base64::Engine;
use rust_ffi::rust_ffi_qwen3_asr_flash;

#[test]
fn funded_user_returns_lyrics() {
    let audio = include_bytes!("qwen3_asr_flash/test_audio.mp3");
    let audio_b64 = base64::engine::general_purpose::STANDARD.encode(audio);
    let u = cs(&test_user()); let pw = cs(TEST_PASSWORD);
    let a = cs(&audio_b64);
    let mut len = 0usize;
    let r = rust_ffi_qwen3_asr_flash(u.as_ptr(), pw.as_ptr(), a.as_ptr(), std::ptr::null(), &mut len);
    let lyrics = String::from_utf8(unsafe { take(r, len) }).unwrap();
    assert_ne!(lyrics, "Could not process lyrics");
    assert_ne!(lyrics, "Top up to transcribe lyrics");
    assert!(!lyrics.is_empty());
}

#[test]
fn empty_audio_returns_fallback() {
    let u = cs(&test_user()); let pw = cs(TEST_PASSWORD);
    let a = cs("");
    let mut len = 0usize;
    let r = rust_ffi_qwen3_asr_flash(u.as_ptr(), pw.as_ptr(), a.as_ptr(), std::ptr::null(), &mut len);
    let lyrics = String::from_utf8(unsafe { take(r, len) }).unwrap();
    assert_eq!(lyrics, "Could not process lyrics");
}
