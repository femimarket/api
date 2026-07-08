use crate::{client, resolve_image, URL};

pub(crate) async fn core_flux2_dev_i2i(image_b64: String, prompt: String) -> Vec<u8> {
    let body = serde_json::json!({
        "id": uuid::Uuid::now_v7(),
        "user_id": "",
        "action": {
            "type": "Flux2DevI2I",
            "image": image_b64,
            "prompt": prompt,
            "comfy_request_id": "",
            "file": "",
        },
    });
    let req = client().post(URL).json(&body);
    let (status, bytes) = match req.send().await {
        Ok(r) => {
            let status = r.status().as_u16();
            let bytes = r.bytes().await.unwrap_or_default();
            (status, bytes.to_vec())
        }
        Err(e) => (e.status().map(|s| s.as_u16()).unwrap_or(0), Vec::new()),
    };
    resolve_image(status, &bytes)
}

#[cfg(not(target_arch = "wasm32"))]
pub mod native {
    use super::*;
    use std::ffi::{c_char, CStr};
    use std::sync::atomic::{AtomicU8, Ordering};
    use std::time::Duration;
    use crate::rt;

    #[unsafe(no_mangle)]
    pub extern "C" fn rust_ffi_flux2_dev_i2i(
        image_b64: *const c_char,
        prompt: *const c_char,
        cancel_flag: *const u8,
        out_len: *mut usize,
    ) -> *mut u8 {
        let s = |p: *const c_char| unsafe { (!p.is_null()).then(|| CStr::from_ptr(p).to_string_lossy().into_owned()) };
        let i = s(image_b64).unwrap_or_default();
        let pr = s(prompt).unwrap_or_default();
        let cancel_addr = if cancel_flag.is_null() { 0usize } else { cancel_flag as usize };

        let bytes = rt().block_on(async move {
            let do_call = core_flux2_dev_i2i(i, pr);
            if cancel_addr == 0 {
                do_call.await
            } else {
                let watch = async move {
                    let ptr = cancel_addr as *mut u8;
                    loop {
                        let v = unsafe { AtomicU8::from_ptr(ptr).load(Ordering::Relaxed) };
                        if v != 0 { return; }
                        tokio::time::sleep(Duration::from_millis(10)).await;
                    }
                };
                tokio::select! {
                    r = do_call => r,
                    _ = watch => crate::FALLBACK_IMAGE.to_vec(),
                }
            }
        });

        let len = bytes.len();
        unsafe { *out_len = len; }
        if len == 0 { std::ptr::null_mut() } else { Box::into_raw(bytes.into_boxed_slice()) as *mut u8 }
    }
}

#[cfg(target_arch = "wasm32")]
pub mod wasm {
    use super::*;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub async fn wasm_flux2_dev_i2i(image_b64: String, prompt: String) -> js_sys::Uint8Array {
        let bytes = core_flux2_dev_i2i(image_b64, prompt).await;
        js_sys::Uint8Array::from(bytes.as_slice())
    }
}
