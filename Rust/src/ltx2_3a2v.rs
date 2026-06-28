use std::ffi::{c_char, CStr};
use std::sync::atomic::{AtomicU8, Ordering};
use std::time::Duration;

use crate::{client, rt, URL};

/// POSTs an Ltx2_3A2V Model to femi.market with HTTP Basic auth (`user:password`).
/// `image_b64` and `audio_b64` may be empty strings (server treats them as unused).
/// Writes HTTP status through `out_status` (0 on transport failure / cancellation).
/// Returns a heap pointer to the response body bytes (length through `out_len`);
/// caller must `free()`. Returns null when body is empty.
/// `cancel_flag` (optional, NULL skips): a byte the caller flips to non-zero to abort.
#[no_mangle]
pub extern "C" fn rust_ffi_ltx2_3a2v(
    user: *const c_char,
    password: *const c_char,
    image_b64: *const c_char,
    audio_b64: *const c_char,
    prompt: *const c_char,
    cancel_flag: *const u8,
    out_status: *mut u16,
    out_len: *mut usize,
) -> *mut u8 {
    let s = |p: *const c_char| unsafe { (!p.is_null()).then(|| CStr::from_ptr(p).to_string_lossy().into_owned()) };
    let body = serde_json::json!({
        "id": uuid::Uuid::now_v7(),
        "user_id": "",
        "action": {
            "type": "Ltx2_3A2V",
            "image": s(image_b64).unwrap_or_default(),
            "audio": s(audio_b64).unwrap_or_default(),
            "prompt": s(prompt).unwrap_or_default(),
            "comfy_request_id": "",
            "file": "",
        },
        "status": "Pending",
        "credit": 0,
    });
    let user_str = s(user).unwrap_or_default();
    let password_str = s(password);
    let cancel_addr = if cancel_flag.is_null() { 0usize } else { cancel_flag as usize };

    let (status, bytes) = rt().block_on(async move {
        let req = client().post(URL).json(&body).basic_auth(user_str, password_str);
        let do_call = async {
            match req.send().await {
                Ok(r) => {
                    let status = r.status().as_u16();
                    let bytes = r.bytes().await.unwrap_or_default();
                    (status, bytes.to_vec())
                }
                Err(e) => (e.status().map(|s| s.as_u16()).unwrap_or(0), Vec::new()),
            }
        };
        if cancel_addr == 0 {
            do_call.await
        } else {
            let watch = async move {
                let ptr = cancel_addr as *mut u8;
                loop {
                    // SAFETY: caller guarantees the pointer is valid for the duration of this call.
                    let v = unsafe { AtomicU8::from_ptr(ptr).load(Ordering::Relaxed) };
                    if v != 0 { return; }
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
            };
            tokio::select! {
                r = do_call => r,
                _ = watch => (0u16, Vec::new()),
            }
        }
    });

    let len = bytes.len();
    // SAFETY: caller guarantees `out_status` and `out_len` are valid pointers.
    unsafe {
        *out_status = status;
        *out_len = len;
    }
    if len == 0 {
        std::ptr::null_mut()
    } else {
        Box::into_raw(bytes.into_boxed_slice()) as *mut u8
    }
}
