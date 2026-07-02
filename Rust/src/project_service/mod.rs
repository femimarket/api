//! ProjectService XMP FFI. Every platform's `psxmp_*` takes a path.
//!
//! Split into 4 modules:
//!
//! - `shared` — namespace constants + `xmpkit_body` (bytes-based XMP logic
//!    used by both Android and WASM).
//! - `apple` — `xmp_toolkit` path API (full smart-handler set). Exposed to
//!    Swift via the C ABI defined below.
//! - `android` — inlines OS reads/writes + `xmpkit_body`; exposes JNI symbols
//!    directly to `ProjectServiceJvm.kt` (no C ABI on Android).
//! - `wasm` — inlines OPFS reads/writes + `xmpkit_body`; exposes wasm-bindgen
//!    symbols directly to Kotlin/wasm-js (no C ABI on wasm).

#![allow(clippy::missing_safety_doc)]

#[path = "share.rs"] mod shared;

#[cfg(all(target_vendor = "apple", not(target_arch = "wasm32")))]
mod apple;
#[cfg(all(target_vendor = "apple", not(target_arch = "wasm32")))]
use apple as backend;

#[cfg(target_os = "android")]
mod android;

#[cfg(target_arch = "wasm32")]
mod wasm;

// ---------- Native C ABI (Apple only; Android uses JNI, WASM uses wasm_bindgen) ----------

#[cfg(all(target_vendor = "apple", not(target_arch = "wasm32")))]
use std::ffi::{c_char, c_int, CStr};

#[cfg(all(target_vendor = "apple", not(target_arch = "wasm32")))]
unsafe fn cstr<'a>(p: *const c_char) -> Option<&'a str> {
    if p.is_null() { return None; }
    CStr::from_ptr(p).to_str().ok()
}

#[cfg(all(target_vendor = "apple", not(target_arch = "wasm32")))]
unsafe fn write_str(buf: *mut c_char, buf_len: c_int, s: &str) -> c_int {
    if buf.is_null() || buf_len <= 0 { return -1; }
    let bytes = s.as_bytes();
    let cap = (buf_len as usize).saturating_sub(1);
    let len = bytes.len().min(cap);
    std::ptr::copy_nonoverlapping(bytes.as_ptr() as *const c_char, buf, len);
    *buf.add(len) = 0;
    len as c_int
}

#[cfg(all(target_vendor = "apple", not(target_arch = "wasm32")))]
#[no_mangle]
pub unsafe extern "C" fn psxmp_embed(
    path: *const c_char,
    prompt: *const c_char,
    model: *const c_char,
    subject: *const *const c_char,
    subject_count: c_int,
) -> c_int {
    let Some(p) = cstr(path) else { return -1 };
    let pr = cstr(prompt);
    let md = cstr(model);
    let mut items: Vec<&str> = Vec::new();
    if !subject.is_null() && subject_count > 0 {
        for i in 0..subject_count {
            if let Some(s) = cstr(*subject.add(i as usize)) { items.push(s); }
        }
    }
    backend::embed(p, pr, md, &items)
}

#[cfg(all(target_vendor = "apple", not(target_arch = "wasm32")))]
#[no_mangle]
pub unsafe extern "C" fn psxmp_read_prompt(path: *const c_char, buf: *mut c_char, buf_len: c_int) -> c_int {
    let Some(p) = cstr(path) else { return -1 };
    match backend::read_prompt(p) {
        Some(s) if !s.is_empty() => write_str(buf, buf_len, &s),
        _ => 0,
    }
}

#[cfg(all(target_vendor = "apple", not(target_arch = "wasm32")))]
#[no_mangle]
pub unsafe extern "C" fn psxmp_read_model(path: *const c_char, buf: *mut c_char, buf_len: c_int) -> c_int {
    let Some(p) = cstr(path) else { return -1 };
    match backend::read_model(p) {
        Some(s) if !s.is_empty() => write_str(buf, buf_len, &s),
        _ => 0,
    }
}

#[cfg(all(target_vendor = "apple", not(target_arch = "wasm32")))]
#[no_mangle]
pub unsafe extern "C" fn psxmp_read_subject_count(path: *const c_char) -> c_int {
    let Some(p) = cstr(path) else { return -1 };
    backend::read_subject_count(p)
}

#[cfg(all(target_vendor = "apple", not(target_arch = "wasm32")))]
#[no_mangle]
pub unsafe extern "C" fn psxmp_read_subject_at(
    path: *const c_char, index: c_int, buf: *mut c_char, buf_len: c_int,
) -> c_int {
    let Some(p) = cstr(path) else { return -1 };
    match backend::read_subject_at(p, index) {
        Some(s) => write_str(buf, buf_len, &s),
        None => 0,
    }
}

#[cfg(all(target_vendor = "apple", not(target_arch = "wasm32")))]
#[no_mangle]
pub unsafe extern "C" fn psxmp_set_rating(path: *const c_char, rating: c_int) -> c_int {
    let Some(p) = cstr(path) else { return -1 };
    backend::set_rating(p, rating)
}

#[cfg(all(target_vendor = "apple", not(target_arch = "wasm32")))]
#[no_mangle]
pub unsafe extern "C" fn psxmp_read_rating(path: *const c_char) -> c_int {
    let Some(p) = cstr(path) else { return -1 };
    backend::read_rating(p)
}

#[cfg(all(target_vendor = "apple", not(target_arch = "wasm32")))]
#[no_mangle]
pub unsafe extern "C" fn psxmp_read_property(
    path: *const c_char,
    namespace_uri: *const c_char,
    property_name: *const c_char,
    buf: *mut c_char,
    buf_len: c_int,
) -> c_int {
    let Some(p) = cstr(path) else { return -1 };
    let Some(ns) = cstr(namespace_uri) else { return -1 };
    let Some(name) = cstr(property_name) else { return -1 };
    match backend::read_property(p, ns, name) {
        Some(s) if !s.is_empty() => write_str(buf, buf_len, &s),
        _ => 0,
    }
}

