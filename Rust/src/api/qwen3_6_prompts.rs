//! The qwen3_6_35b_a3b prompt-generation family. All authless, all return a
//! single `result` string from the server's `action.result`. Each variant just
//! builds its action JSON with its inputs; `gen` does the POST + extract.

use crate::{client, URL};
use serde_json::json;

async fn gen(action: serde_json::Value) -> Vec<u8> {
    let body = json!({ "id": uuid::Uuid::now_v7(), "user_id": "", "action": action });
    let (status, bytes) = match client().post(URL).json(&body).send().await {
        Ok(r) => {
            let s = r.status().as_u16();
            (s, r.bytes().await.unwrap_or_default().to_vec())
        }
        Err(e) => (e.status().map(|s| s.as_u16()).unwrap_or(0), Vec::new()),
    };
    let result = if status == 200 {
        serde_json::from_slice::<serde_json::Value>(&bytes)
            .ok()
            .and_then(|j| j.pointer("/action/result").and_then(|v| v.as_str()).map(str::to_string))
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "Could not respond".to_string())
    } else {
        "Could not respond".to_string()
    };
    result.into_bytes()
}

// ---------- action builders (one per endpoint) ----------

fn action_0gen_music_video_prompt() -> serde_json::Value {
    json!({ "type": "Qwen3_6_35bA3b0GenMusicVideoPrompt", "result": "" })
}
fn action_1gen_new_angle(xmp_prompt: String) -> serde_json::Value {
    json!({ "type": "Qwen3_6_35bA3b1GenNewAngleFromXmpImagePrompt", "xmp_prompt": xmp_prompt, "result": "" })
}
fn action_1gen_random_idea(xmp_prompt: String) -> serde_json::Value {
    json!({ "type": "Qwen3_6_35bA3b1GenRandomIdeaFromXmpPrompt", "xmp_prompt": xmp_prompt, "result": "" })
}
fn action_1gen_3_variants(xmp_prompt: String) -> serde_json::Value {
    json!({ "type": "Qwen3_6_35bA3b1Gen3VariantsFromXmpPromptAsJsonarray", "xmp_prompt": xmp_prompt, "result": "" })
}
fn action_2gen_augment(xmp_prompt: String, text: String) -> serde_json::Value {
    json!({ "type": "Qwen3_6_35bA3b2GenAugmentIdeaFromXmpPromptAndText", "xmp_prompt": xmp_prompt, "text": text, "result": "" })
}
fn action_2gen_multishot(xmp_prompt: String, xmp_prompt2: String) -> serde_json::Value {
    json!({ "type": "Qwen3_6_35bA3b2Gen50100WordMultishotTimestampedPromptFrom2XmpImagePrompts", "xmp_prompt": xmp_prompt, "xmp_prompt2": xmp_prompt2, "result": "" })
}
fn action_3gen_multishot(xmp_prompt: String, xmp_prompt2: String, xmp_prompt3: String) -> serde_json::Value {
    json!({ "type": "Qwen3_6_35bA3b3Gen50100WordMultishotTimestampedPromptFrom3XmpImagePrompts", "xmp_prompt": xmp_prompt, "xmp_prompt2": xmp_prompt2, "xmp_prompt3": xmp_prompt3, "result": "" })
}
fn action_4gen_augment(xmp_prompt: String, xmp_prompt2: String, xmp_prompt3: String, text: String) -> serde_json::Value {
    json!({ "type": "Qwen3_6_35bA3b4GenAugmentIdeaFrom3XmpPromptsAndText", "xmp_prompt": xmp_prompt, "xmp_prompt2": xmp_prompt2, "xmp_prompt3": xmp_prompt3, "text": text, "result": "" })
}

#[cfg(not(target_arch = "wasm32"))]
pub mod native {
    use super::*;
    use std::ffi::{c_char, CStr};
    use std::sync::atomic::{AtomicU8, Ordering};
    use std::time::Duration;
    use crate::rt;

    fn cstr(p: *const c_char) -> String {
        unsafe { (!p.is_null()).then(|| CStr::from_ptr(p).to_string_lossy().into_owned()) }.unwrap_or_default()
    }

    // Run `action` through gen(), honoring the cancel flag, and hand back a
    // heap buffer the caller frees. Shared by every endpoint below.
    fn run(action: serde_json::Value, cancel_flag: *const u8, out_len: *mut usize) -> *mut u8 {
        let cancel_addr = if cancel_flag.is_null() { 0usize } else { cancel_flag as usize };
        let bytes = rt().block_on(async move {
            let do_call = gen(action);
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
                    _ = watch => b"Could not respond".to_vec(),
                }
            }
        });
        let len = bytes.len();
        unsafe { *out_len = len; }
        if len == 0 { std::ptr::null_mut() } else { Box::into_raw(bytes.into_boxed_slice()) as *mut u8 }
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn rust_ffi_qwen3_6_35b_a3b_0gen_music_video_prompt(
        cancel_flag: *const u8, out_len: *mut usize,
    ) -> *mut u8 {
        run(action_0gen_music_video_prompt(), cancel_flag, out_len)
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn rust_ffi_qwen3_6_35b_a3b_1gen_new_angle_from_xmp_image_prompt(
        xmp_prompt: *const c_char, cancel_flag: *const u8, out_len: *mut usize,
    ) -> *mut u8 {
        run(action_1gen_new_angle(cstr(xmp_prompt)), cancel_flag, out_len)
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn rust_ffi_qwen3_6_35b_a3b_1gen_random_idea_from_xmp_prompt(
        xmp_prompt: *const c_char, cancel_flag: *const u8, out_len: *mut usize,
    ) -> *mut u8 {
        run(action_1gen_random_idea(cstr(xmp_prompt)), cancel_flag, out_len)
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn rust_ffi_qwen3_6_35b_a3b_1gen_3_variants_from_xmp_prompt_as_jsonarray(
        xmp_prompt: *const c_char, cancel_flag: *const u8, out_len: *mut usize,
    ) -> *mut u8 {
        run(action_1gen_3_variants(cstr(xmp_prompt)), cancel_flag, out_len)
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn rust_ffi_qwen3_6_35b_a3b_2gen_augment_idea_from_xmp_prompt_and_text(
        xmp_prompt: *const c_char, text: *const c_char, cancel_flag: *const u8, out_len: *mut usize,
    ) -> *mut u8 {
        run(action_2gen_augment(cstr(xmp_prompt), cstr(text)), cancel_flag, out_len)
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn rust_ffi_qwen3_6_35b_a3b_2gen_50_100_word_multishot_timestamped_prompt_from_2_xmp_image_prompts(
        xmp_prompt: *const c_char, xmp_prompt2: *const c_char, cancel_flag: *const u8, out_len: *mut usize,
    ) -> *mut u8 {
        run(action_2gen_multishot(cstr(xmp_prompt), cstr(xmp_prompt2)), cancel_flag, out_len)
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn rust_ffi_qwen3_6_35b_a3b_3gen_50_100_word_multishot_timestamped_prompt_from_3_xmp_image_prompts(
        xmp_prompt: *const c_char, xmp_prompt2: *const c_char, xmp_prompt3: *const c_char, cancel_flag: *const u8, out_len: *mut usize,
    ) -> *mut u8 {
        run(action_3gen_multishot(cstr(xmp_prompt), cstr(xmp_prompt2), cstr(xmp_prompt3)), cancel_flag, out_len)
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn rust_ffi_qwen3_6_35b_a3b_4gen_augment_idea_from_3_xmp_prompts_and_text(
        xmp_prompt: *const c_char, xmp_prompt2: *const c_char, xmp_prompt3: *const c_char, text: *const c_char, cancel_flag: *const u8, out_len: *mut usize,
    ) -> *mut u8 {
        run(action_4gen_augment(cstr(xmp_prompt), cstr(xmp_prompt2), cstr(xmp_prompt3), cstr(text)), cancel_flag, out_len)
    }
}

#[cfg(target_arch = "wasm32")]
pub mod wasm {
    use super::*;
    use wasm_bindgen::prelude::*;

    async fn s(action: serde_json::Value) -> String {
        String::from_utf8(gen(action).await).unwrap_or_default()
    }

    #[wasm_bindgen]
    pub async fn wasm_qwen3_6_35b_a3b_0gen_music_video_prompt() -> String {
        s(action_0gen_music_video_prompt()).await
    }
    #[wasm_bindgen]
    pub async fn wasm_qwen3_6_35b_a3b_1gen_new_angle_from_xmp_image_prompt(xmp_prompt: String) -> String {
        s(action_1gen_new_angle(xmp_prompt)).await
    }
    #[wasm_bindgen]
    pub async fn wasm_qwen3_6_35b_a3b_1gen_random_idea_from_xmp_prompt(xmp_prompt: String) -> String {
        s(action_1gen_random_idea(xmp_prompt)).await
    }
    #[wasm_bindgen]
    pub async fn wasm_qwen3_6_35b_a3b_1gen_3_variants_from_xmp_prompt_as_jsonarray(xmp_prompt: String) -> String {
        s(action_1gen_3_variants(xmp_prompt)).await
    }
    #[wasm_bindgen]
    pub async fn wasm_qwen3_6_35b_a3b_2gen_augment_idea_from_xmp_prompt_and_text(xmp_prompt: String, text: String) -> String {
        s(action_2gen_augment(xmp_prompt, text)).await
    }
    #[wasm_bindgen]
    pub async fn wasm_qwen3_6_35b_a3b_2gen_50_100_word_multishot_timestamped_prompt_from_2_xmp_image_prompts(xmp_prompt: String, xmp_prompt2: String) -> String {
        s(action_2gen_multishot(xmp_prompt, xmp_prompt2)).await
    }
    #[wasm_bindgen]
    pub async fn wasm_qwen3_6_35b_a3b_3gen_50_100_word_multishot_timestamped_prompt_from_3_xmp_image_prompts(xmp_prompt: String, xmp_prompt2: String, xmp_prompt3: String) -> String {
        s(action_3gen_multishot(xmp_prompt, xmp_prompt2, xmp_prompt3)).await
    }
    #[wasm_bindgen]
    pub async fn wasm_qwen3_6_35b_a3b_4gen_augment_idea_from_3_xmp_prompts_and_text(xmp_prompt: String, xmp_prompt2: String, xmp_prompt3: String, text: String) -> String {
        s(action_4gen_augment(xmp_prompt, xmp_prompt2, xmp_prompt3, text)).await
    }
}
