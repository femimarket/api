use std::ffi::{c_char, CStr};
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU8, Ordering};
use std::time::Duration;

const URL: &str = "https://femi.market/api";

static RT: OnceLock<tokio::runtime::Runtime> = OnceLock::new();
fn rt() -> &'static tokio::runtime::Runtime {
    RT.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .expect("tokio runtime")
    })
}

static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();
fn client() -> &'static reqwest::Client {
    CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(120))
            .build()
            .expect("reqwest client")
    })
}

/// POSTs a ZImageTurbo Model to femi.market with `Authorization: Bearer {token}`.
/// `cancel_flag` (optional, NULL skips): a byte the caller flips to non-zero to
/// abort. Rust polls it every ~10ms; on cancel the in-flight HTTP future is
/// dropped, tearing down the TCP connection so the server frees its handler.
/// Writes HTTP status (0 on transport failure / cancellation) and body length;
/// returns heap pointer to body bytes (caller must `free()`; null when empty).
#[no_mangle]
pub extern "C" fn rust_ffi_z_image_turbo(
    token: *const c_char,
    prompt: *const c_char,
    cancel_flag: *const u8,
    out_status: *mut u16,
    out_len: *mut usize,
) -> *mut u8 {
    // SAFETY: caller guarantees each non-NULL `*const c_char` points to a valid
    // NUL-terminated C string that lives for the duration of this call.
    let s = |p: *const c_char| unsafe { (!p.is_null()).then(|| CStr::from_ptr(p).to_string_lossy().into_owned()) };
    let body = serde_json::json!({
        "id": uuid::Uuid::now_v7(),
        "user_id": "",
        "action": { "type": "ZImageTurbo", "prompt": s(prompt).unwrap_or_default(), "fal_request_id": "", "file": "" },
        "status": "Pending",
        "credit": 0,
    });
    let token_str = s(token);
    let cancel_addr = if cancel_flag.is_null() { 0usize } else { cancel_flag as usize };

    let (status, bytes) = rt().block_on(async move {
        let mut req = client().post(URL).json(&body);
        if let Some(t) = token_str {
            req = req.header("Authorization", format!("Bearer {t}"));
        }
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
                    // SAFETY: caller guarantees `cancel_flag` points to a valid u8
                    // for the duration of this call. AtomicU8 has size and alignment
                    // 1 byte (matches u8), so reading through `AtomicU8::from_ptr`
                    // is sound; Relaxed ordering is sufficient for a cancellation flag.
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
