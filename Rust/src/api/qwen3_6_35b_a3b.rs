use crate::{client, URL};

pub(crate) async fn core_qwen3_6_35b_a3b(user: String, password: String, messages_json: String) -> Vec<u8> {
    let original: serde_json::Value = serde_json::from_str(&messages_json)
        .unwrap_or_else(|_| serde_json::Value::Array(Vec::new()));
    let body = serde_json::json!({
        "id": uuid::Uuid::now_v7(),
        "user_id": "",
        "action": { "type": "Qwen3_6_35bA3b", "messages": original.clone() },
    });
    let req = client().post(URL).json(&body).basic_auth(user, Some(password));
    let (status, bytes) = match req.send().await {
        Ok(r) => {
            let status = r.status().as_u16();
            let bytes = r.bytes().await.unwrap_or_default();
            (status, bytes.to_vec())
        }
        Err(e) => (e.status().map(|s| s.as_u16()).unwrap_or(0), Vec::new()),
    };
    let reply = if status == 200 {
        serde_json::from_slice::<serde_json::Value>(&bytes)
            .ok()
            .and_then(|j| j.pointer("/action/messages").and_then(|v| v.as_array()).cloned())
            .and_then(|msgs| msgs.last().cloned())
            .and_then(|last| last.get("content").and_then(|v| v.as_str()).map(str::to_string))
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "Could not respond".to_string())
    } else {
        "Could not respond".to_string()
    };
    let mut arr = original.as_array().cloned().unwrap_or_default();
    arr.push(serde_json::json!({ "role": "Assistant", "content": reply }));
    serde_json::to_vec(&serde_json::Value::Array(arr)).unwrap_or_default()
}

#[cfg(not(target_arch = "wasm32"))]
pub mod native {
    use super::*;
    use std::ffi::{c_char, CStr};
    use std::sync::atomic::{AtomicU8, Ordering};
    use std::time::Duration;
    use crate::rt;

    #[unsafe(no_mangle)]
    pub extern "C" fn rust_ffi_qwen3_6_35b_a3b(
        user: *const c_char,
        password: *const c_char,
        messages_json: *const c_char,
        cancel_flag: *const u8,
        out_len: *mut usize,
    ) -> *mut u8 {
        let s = |p: *const c_char| unsafe { (!p.is_null()).then(|| CStr::from_ptr(p).to_string_lossy().into_owned()) };
        let u = s(user).unwrap_or_default();
        let pw = s(password).unwrap_or_default();
        let m = s(messages_json).unwrap_or_default();
        let cancel_addr = if cancel_flag.is_null() { 0usize } else { cancel_flag as usize };

        let bytes = rt().block_on(async move {
            let m_for_cancel = m.clone();
            let do_call = core_qwen3_6_35b_a3b(u, pw, m);
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
                    _ = watch => {
                        // On cancel: return input messages + "Could not respond" assistant turn.
                        let original: serde_json::Value = serde_json::from_str(&m_for_cancel)
                            .unwrap_or_else(|_| serde_json::Value::Array(Vec::new()));
                        let mut arr = original.as_array().cloned().unwrap_or_default();
                        arr.push(serde_json::json!({ "role": "Assistant", "content": "Could not respond" }));
                        serde_json::to_vec(&serde_json::Value::Array(arr)).unwrap_or_default()
                    }
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
    pub async fn wasm_qwen3_6_35b_a3b(user: String, password: String, messages_json: String) -> String {
        let bytes = core_qwen3_6_35b_a3b(user, password, messages_json).await;
        String::from_utf8(bytes).unwrap_or_default()
    }
}
