//! Direct-to-ComfyUI variant of ltx2_3a2v. Unlike `ltx2_3a2v` (which posts to
//! femi.market with basic-auth), this talks to ComfyUI Cloud directly using a
//! caller-supplied API key: upload image+audio → patch the embedded workflow →
//! POST /api/prompt → poll /api/jobs → GET /api/view → raw video bytes.
//!
//! Port of femi server `comfyui::comfyui_ltx2_3a2v`, minus the DB/charging.
//! Any failure returns the embedded FALLBACK_VIDEO, matching `core_ltx2_3a2v`.

use crate::{client, FALLBACK_VIDEO};
use base64::Engine;
use serde_json::json;
use uuid::Uuid;

const COMFY_BASE_URL: &str = "https://cloud.comfy.org";
const WORKFLOW: &str = include_str!("../../../Assets/ltx23a2v.json");

/// 2s poll interval, using the right timer for the target.
async fn sleep_poll() {
    #[cfg(not(target_arch = "wasm32"))]
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    #[cfg(target_arch = "wasm32")]
    gloo_timers::future::TimeoutFuture::new(2000).await;
}

pub(crate) async fn core_ltx2_3a2v_comfyui(
    comfy_key: String,
    image_b64: String,
    audio_b64: String,
    prompt: String,
) -> Vec<u8> {
    run(&comfy_key, &image_b64, &audio_b64, &prompt)
        .await
        .unwrap_or_else(|_| FALLBACK_VIDEO.to_vec())
}

async fn run(key: &str, image_b64: &str, audio_b64: &str, prompt: &str) -> Result<Vec<u8>, ()> {
    let image = upload_input(key, image_b64).await?;
    let audio = upload_input(key, audio_b64).await?;

    let mut wf: serde_json::Value = serde_json::from_str(WORKFLOW).map_err(|_| ())?;
    wf["269"]["inputs"]["image"] = json!(image);
    wf["276"]["inputs"]["audio"] = json!(audio);
    wf["276"]["inputs"]["audioUI"] =
        json!(format!("/api/view?filename={audio}&type=input&subfolder=&"));
    wf["340:319"]["inputs"]["value"] = json!(prompt);
    // Fresh seed per request — the exported workflow's fixed seed makes comfy
    // serve a cached (empty) result otherwise.
    wf["340:285"]["inputs"]["noise_seed"] = json!(Uuid::now_v7().as_u128() as u64);
    wf["340:286"]["inputs"]["noise_seed"] = json!(Uuid::now_v7().as_u128() as u64);

    let queued: serde_json::Value = client()
        .post(format!("{COMFY_BASE_URL}/api/prompt"))
        .header("X-API-Key", key)
        .json(&json!({ "prompt": wf, "extra_data": { "api_key_comfy_org": key } }))
        .send()
        .await
        .map_err(|_| ())?
        .json()
        .await
        .map_err(|_| ())?;
    let req_id = queued.get("prompt_id").and_then(|v| v.as_str()).ok_or(())?.to_string();

    loop {
        let job: serde_json::Value = client()
            .get(format!("{COMFY_BASE_URL}/api/jobs/{req_id}"))
            .header("X-API-Key", key)
            .send()
            .await
            .and_then(|r| r.error_for_status())
            .map_err(|_| ())?
            .json()
            .await
            .map_err(|_| ())?;

        match job.get("status").and_then(|v| v.as_str()) {
            Some("completed") => {
                let output_filename = job
                    .pointer("/preview_output/filename")
                    .and_then(|v| v.as_str())
                    .ok_or(())?;
                let bytes = client()
                    .get(format!(
                        "{COMFY_BASE_URL}/api/view?filename={output_filename}&type=output"
                    ))
                    .header("X-API-Key", key)
                    .send()
                    .await
                    .map_err(|_| ())?
                    .bytes()
                    .await
                    .map_err(|_| ())?;
                return Ok(bytes.to_vec());
            }
            Some("error") | Some("cancelled") => return Err(()),
            _ => sleep_poll().await,
        }
    }
}

/// Decode base64 (data-URI on web, raw on mobile), sniff the real type, upload
/// under a fresh name, and return comfy's stored filename.
async fn upload_input(key: &str, b64: &str) -> Result<String, ()> {
    let b64 = b64.rsplit(',').next().unwrap_or(b64);
    let bytes = base64::engine::general_purpose::STANDARD.decode(b64).map_err(|_| ())?;
    let ext = infer::get(&bytes).ok_or(())?.extension();
    let file_name = format!("{}.{ext}", Uuid::now_v7());
    let part = reqwest::multipart::Part::bytes(bytes).file_name(file_name);
    let form = reqwest::multipart::Form::new().part("image", part);
    let payload: serde_json::Value = client()
        .post(format!("{COMFY_BASE_URL}/api/upload/image"))
        .header("X-API-Key", key)
        .multipart(form)
        .send()
        .await
        .and_then(|r| r.error_for_status())
        .map_err(|_| ())?
        .json()
        .await
        .map_err(|_| ())?;
    payload.get("name").and_then(|v| v.as_str()).map(str::to_string).ok_or(())
}

#[cfg(not(target_arch = "wasm32"))]
pub mod native {
    use super::*;
    use crate::rt;
    use std::ffi::{c_char, CStr};
    use std::sync::atomic::{AtomicU8, Ordering};
    use std::time::Duration;

    #[unsafe(no_mangle)]
    pub extern "C" fn rust_ffi_ltx2_3a2v_comfyui(
        comfy_key: *const c_char,
        image_b64: *const c_char,
        audio_b64: *const c_char,
        prompt: *const c_char,
        cancel_flag: *const u8,
        out_len: *mut usize,
    ) -> *mut u8 {
        let s = |p: *const c_char| unsafe {
            (!p.is_null()).then(|| CStr::from_ptr(p).to_string_lossy().into_owned())
        };
        let key = s(comfy_key).unwrap_or_default();
        let i = s(image_b64).unwrap_or_default();
        let a = s(audio_b64).unwrap_or_default();
        let pr = s(prompt).unwrap_or_default();
        let cancel_addr = if cancel_flag.is_null() { 0usize } else { cancel_flag as usize };

        let bytes = rt().block_on(async move {
            let do_call = core_ltx2_3a2v_comfyui(key, i, a, pr);
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
                    _ = watch => crate::FALLBACK_VIDEO.to_vec(),
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
    pub async fn wasm_ltx2_3a2v_comfyui(
        comfy_key: String,
        image_b64: String,
        audio_b64: String,
        prompt: String,
    ) -> js_sys::Uint8Array {
        let bytes = core_ltx2_3a2v_comfyui(comfy_key, image_b64, audio_b64, prompt).await;
        js_sys::Uint8Array::from(bytes.as_slice())
    }
}
