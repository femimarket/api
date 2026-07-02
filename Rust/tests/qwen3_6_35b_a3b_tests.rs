#![cfg(not(target_arch = "wasm32"))]

#[path = "api_tests.rs"] mod api_tests;
use api_tests::*;
use rust_ffi::rust_ffi_qwen3_6_35b_a3b;

#[test]
fn funded_user_returns_reply() {
    let u = cs(&test_user()); let pw = cs(TEST_PASSWORD);
    let m = cs(r#"[{"role":"User","content":"say hi in one word"}]"#);
    let mut len = 0usize;
    let r = rust_ffi_qwen3_6_35b_a3b(u.as_ptr(), pw.as_ptr(), m.as_ptr(), std::ptr::null(), &mut len);
    let bytes = unsafe { take(r, len) };
    let arr = serde_json::from_slice::<serde_json::Value>(&bytes).unwrap();
    let arr = arr.as_array().unwrap();
    assert_eq!(arr.len(), 2);
    assert_eq!(arr.last().unwrap()["role"], "Assistant");
    let reply = arr.last().unwrap()["content"].as_str().unwrap();
    assert_ne!(reply, "Could not respond");
    assert!(!reply.is_empty());
}

#[test]
fn empty_messages_returns_fallback() {
    let u = cs(&test_user()); let pw = cs(TEST_PASSWORD);
    let m = cs("[]");
    let mut len = 0usize;
    let r = rust_ffi_qwen3_6_35b_a3b(u.as_ptr(), pw.as_ptr(), m.as_ptr(), std::ptr::null(), &mut len);
    let bytes = unsafe { take(r, len) };
    let arr = serde_json::from_slice::<serde_json::Value>(&bytes).unwrap();
    let arr = arr.as_array().unwrap();
    assert_eq!(arr.len(), 1);
    assert_eq!(arr.last().unwrap()["content"], "Could not respond");
}
