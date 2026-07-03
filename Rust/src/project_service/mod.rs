//! ProjectService XMP FFI. Rust owns the Documents root; every call takes
//! a filename, never a path.
//!
//! - `shared` — namespace constants + `xmpkit_body` (bytes-based XMP logic
//!    used by both Android and WASM).
//! - `apple` — `xmp_toolkit` smart-handler API + `dirs::document_dir()`.
//!    Exposed to Swift via the C ABI defined below.
//! - `android` — inlines OS reads/writes + `xmpkit_body`; exposes JNI symbols
//!    directly to `ProjectServiceJvm.kt` (no C ABI on Android).
//! - `wasm` — inlines OPFS reads/writes + `xmpkit_body`; exposes wasm-bindgen
//!    symbols directly to Kotlin/wasm-js (no C ABI on wasm).

#![allow(clippy::missing_safety_doc)]

#[path = "share.rs"] pub mod shared;

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
pub unsafe extern "C" fn psxmp_save_file(
    name: *const c_char,
    bytes: *const u8, len: usize,
    prompt: *const c_char,
    model: *const c_char,
    subject: *const *const c_char, subject_count: c_int,
) {
    let Some(n) = cstr(name) else { return };
    if bytes.is_null() { return }
    let data = std::slice::from_raw_parts(bytes, len);
    let pr = cstr(prompt);
    let md = cstr(model);
    let mut items: Vec<&str> = Vec::new();
    if !subject.is_null() && subject_count > 0 {
        for i in 0..subject_count {
            if let Some(s) = cstr(*subject.add(i as usize)) { items.push(s); }
        }
    }
    backend::save_file(n, data, pr, md, &items);
}

#[cfg(all(target_vendor = "apple", not(target_arch = "wasm32")))]
#[no_mangle]
pub unsafe extern "C" fn psxmp_save_audio(name: *const c_char, bytes: *const u8, len: usize) {
    let Some(n) = cstr(name) else { return };
    if bytes.is_null() { return }
    let data = std::slice::from_raw_parts(bytes, len);
    backend::save_audio(n, data);
}

#[cfg(all(target_vendor = "apple", not(target_arch = "wasm32")))]
#[no_mangle]
pub unsafe extern "C" fn psxmp_like(file: *const c_char, liked: c_int) {
    let Some(f) = cstr(file) else { return };
    backend::like(f, liked != 0);
}

#[cfg(all(target_vendor = "apple", not(target_arch = "wasm32")))]
#[no_mangle]
pub unsafe extern "C" fn psxmp_get_all_generations(buf: *mut c_char, buf_len: c_int) -> c_int {
    let json = serde_json::to_string(&backend::list_generations()).unwrap();
    write_str(buf, buf_len, &json)
}

#[cfg(all(target_vendor = "apple", not(target_arch = "wasm32")))]
#[no_mangle]
pub unsafe extern "C" fn psxmp_get_audio(buf: *mut c_char, buf_len: c_int) -> c_int {
    match backend::get_audio() {
        Some(s) => write_str(buf, buf_len, &s),
        None => 0,
    }
}

#[cfg(all(target_vendor = "apple", not(target_arch = "wasm32")))]
#[no_mangle]
pub unsafe extern "C" fn psxmp_get_prompt(file: *const c_char, buf: *mut c_char, buf_len: c_int) -> c_int {
    let Some(f) = cstr(file) else { return -1 };
    match backend::get_prompt(f) {
        Some(s) if !s.is_empty() => write_str(buf, buf_len, &s),
        _ => 0,
    }
}

#[cfg(all(target_vendor = "apple", not(target_arch = "wasm32")))]
#[no_mangle]
pub unsafe extern "C" fn psxmp_get_model(file: *const c_char, buf: *mut c_char, buf_len: c_int) -> c_int {
    let Some(f) = cstr(file) else { return -1 };
    match backend::get_model(f) {
        Some(s) if !s.is_empty() => write_str(buf, buf_len, &s),
        _ => 0,
    }
}

#[cfg(all(target_vendor = "apple", not(target_arch = "wasm32")))]
#[no_mangle]
pub unsafe extern "C" fn psxmp_get_subject(file: *const c_char, buf: *mut c_char, buf_len: c_int) -> c_int {
    let Some(f) = cstr(file) else { return -1 };
    let items = backend::get_subject(f);
    if items.is_empty() { return 0; }
    let json = serde_json::to_string(&items).unwrap();
    write_str(buf, buf_len, &json)
}

#[cfg(all(target_vendor = "apple", not(target_arch = "wasm32")))]
#[no_mangle]
pub unsafe extern "C" fn psxmp_read_property(
    file: *const c_char, ns: *const c_char, name: *const c_char,
    buf: *mut c_char, buf_len: c_int,
) -> c_int {
    let (Some(f), Some(n), Some(nm)) = (cstr(file), cstr(ns), cstr(name)) else { return -1 };
    match backend::read_property(f, n, nm) {
        Some(s) if !s.is_empty() => write_str(buf, buf_len, &s),
        _ => 0,
    }
}

#[cfg(all(target_vendor = "apple", not(target_arch = "wasm32")))]
#[no_mangle]
pub unsafe extern "C" fn psxmp_get_like(file: *const c_char) -> c_int {
    let Some(f) = cstr(file) else { return -1 };
    if backend::get_like(f) { 1 } else { 0 }
}

#[cfg(all(target_vendor = "apple", not(target_arch = "wasm32")))]
#[no_mangle]
pub unsafe extern "C" fn psxmp_get_url(file: *const c_char, buf: *mut c_char, buf_len: c_int) -> c_int {
    let Some(f) = cstr(file) else { return -1 };
    write_str(buf, buf_len, &backend::get_url(f))
}

#[cfg(all(target_vendor = "apple", not(target_arch = "wasm32")))]
#[no_mangle]
pub unsafe extern "C" fn psxmp_set_character_cast(a: *const c_char, b: *const c_char) {
    let (Some(a), Some(b)) = (cstr(a), cstr(b)) else { return };
    backend::set_character_cast(a, b);
}

#[cfg(all(target_vendor = "apple", not(target_arch = "wasm32")))]
#[no_mangle]
pub unsafe extern "C" fn psxmp_get_character_cast(buf: *mut c_char, buf_len: c_int) -> c_int {
    match backend::get_character_cast() {
        Some((a, b)) => {
            let json = serde_json::to_string(&[a, b]).unwrap();
            write_str(buf, buf_len, &json)
        }
        None => 0,
    }
}

#[cfg(all(target_vendor = "apple", not(target_arch = "wasm32")))]
#[no_mangle]
pub unsafe extern "C" fn psxmp_clear_character_cast() {
    backend::clear_character_cast();
}

#[cfg(all(target_vendor = "apple", not(target_arch = "wasm32")))]
#[no_mangle]
pub unsafe extern "C" fn psxmp_set_image_edit(file: *const c_char) {
    let Some(f) = cstr(file) else { return };
    backend::set_image_edit(f);
}

#[cfg(all(target_vendor = "apple", not(target_arch = "wasm32")))]
#[no_mangle]
pub unsafe extern "C" fn psxmp_get_image_edit(buf: *mut c_char, buf_len: c_int) -> c_int {
    match backend::get_image_edit() {
        Some(s) => write_str(buf, buf_len, &s),
        None => 0,
    }
}

#[cfg(all(target_vendor = "apple", not(target_arch = "wasm32")))]
#[no_mangle]
pub unsafe extern "C" fn psxmp_clear_image_edit() {
    backend::clear_image_edit();
}
