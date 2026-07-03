//! WASM backend — Rust owns the OPFS root implicitly (per-origin flat namespace).
//! Filename IS the OPFS key. XMP mutation via shared bytes-based `xmpkit_body`.

use super::shared::xmpkit_body;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

const AUDIO_EXTS: &[&str] = &[
    "mp3", "m4a", "wav", "aac", "caf", "aiff", "aif", "flac", "ogg", "opus",
];

static CHARACTER_CAST: Mutex<Option<(String, String)>> = Mutex::new(None);
static IMAGE_EDIT: Mutex<Option<String>> = Mutex::new(None);

fn is_audio(name: &str) -> bool {
    match name.rsplit_once('.') {
        Some((_, ext)) => AUDIO_EXTS.iter().any(|a| a.eq_ignore_ascii_case(ext)),
        None => false,
    }
}

// ---------- OPFS I/O ----------

async fn opfs_root() -> Result<web_sys::FileSystemDirectoryHandle, JsValue> {
    let win = web_sys::window().ok_or_else(|| JsValue::from_str("no window"))?;
    let storage = win.navigator().storage();
    JsFuture::from(storage.get_directory()).await?.dyn_into()
}

async fn opfs_read(path: &str) -> Result<Vec<u8>, JsValue> {
    let root = opfs_root().await?;
    let handle_js = JsFuture::from(root.get_file_handle(path)).await?;
    let handle: web_sys::FileSystemFileHandle = handle_js.dyn_into()?;
    let file_js = JsFuture::from(handle.get_file()).await?;
    let file: web_sys::File = file_js.dyn_into()?;
    let ab = JsFuture::from(file.array_buffer()).await?;
    let u8array = js_sys::Uint8Array::new(&ab);
    let mut v = vec![0u8; u8array.length() as usize];
    u8array.copy_to(&mut v);
    Ok(v)
}

async fn opfs_write(path: &str, bytes: &[u8]) -> Result<(), JsValue> {
    let root = opfs_root().await?;
    let opts = web_sys::FileSystemGetFileOptions::new();
    opts.set_create(true);
    let handle_js = JsFuture::from(root.get_file_handle_with_options(path, &opts)).await?;
    let handle: web_sys::FileSystemFileHandle = handle_js.dyn_into()?;
    let writable_js = JsFuture::from(handle.create_writable()).await?;
    let writable: web_sys::FileSystemWritableFileStream = writable_js.dyn_into()?;
    let u8array = js_sys::Uint8Array::from(bytes);
    JsFuture::from(writable.write_with_buffer_source(&u8array)?).await?;
    JsFuture::from(writable.close()).await?;
    Ok(())
}

async fn opfs_delete(path: &str) -> Result<(), JsValue> {
    let root = opfs_root().await?;
    JsFuture::from(root.remove_entry(path)).await?;
    Ok(())
}

async fn opfs_list() -> Result<Vec<String>, JsValue> {
    let root = opfs_root().await?;
    let iter = js_sys::Reflect::get(root.as_ref(), &JsValue::from_str("keys"))?;
    let iter_fn: js_sys::Function = iter.dyn_into()?;
    let async_iter = iter_fn.call0(root.as_ref())?;
    let next_fn: js_sys::Function = js_sys::Reflect::get(&async_iter, &JsValue::from_str("next"))?.dyn_into()?;
    let mut names: Vec<String> = Vec::new();
    loop {
        let result_promise: js_sys::Promise = next_fn.call0(&async_iter)?.dyn_into()?;
        let result = JsFuture::from(result_promise).await?;
        let done = js_sys::Reflect::get(&result, &JsValue::from_str("done"))?
            .as_bool()
            .unwrap_or(true);
        if done { break; }
        let value = js_sys::Reflect::get(&result, &JsValue::from_str("value"))?;
        if let Some(s) = value.as_string() { names.push(s); }
    }
    Ok(names)
}

// ---------- Save ----------

#[wasm_bindgen]
pub async fn psxmp_save_file(
    name: String, bytes: Vec<u8>,
    prompt: Option<String>, model: Option<String>, subjects: Vec<String>,
) {
    let out = if prompt.is_none() && model.is_none() && subjects.is_empty() {
        bytes
    } else {
        let refs: Vec<&str> = subjects.iter().map(|s| s.as_str()).collect();
        xmpkit_body::embed(&bytes, prompt.as_deref(), model.as_deref(), &refs)
    };
    let _ = opfs_write(&name, &out).await;
}

#[wasm_bindgen]
pub async fn psxmp_save_audio(name: String, bytes: Vec<u8>) {
    if let Ok(existing) = opfs_list().await {
        for n in existing {
            if is_audio(&n) { let _ = opfs_delete(&n).await; }
        }
    }
    let _ = opfs_write(&name, &bytes).await;
}

#[wasm_bindgen]
pub async fn psxmp_like(file: String, liked: bool) {
    let Ok(bytes) = opfs_read(&file).await else { return };
    let out = xmpkit_body::set_rating(&bytes, if liked { 5 } else { 0 });
    let _ = opfs_write(&file, &out).await;
}

// ---------- Read ----------

#[wasm_bindgen]
pub async fn psxmp_get_all_generations() -> String {
    serde_json::to_string(&opfs_list().await.unwrap_or_default()).unwrap()
}

#[wasm_bindgen]
pub async fn psxmp_get_audio() -> Option<String> {
    opfs_list().await.ok()?.into_iter().find(|n| is_audio(n))
}

#[wasm_bindgen]
pub async fn psxmp_get_prompt(file: String) -> Option<String> {
    let bytes = opfs_read(&file).await.ok()?;
    xmpkit_body::read_prompt(&bytes).filter(|s| !s.is_empty())
}

#[wasm_bindgen]
pub async fn psxmp_get_model(file: String) -> Option<String> {
    let bytes = opfs_read(&file).await.ok()?;
    xmpkit_body::read_model(&bytes).filter(|s| !s.is_empty())
}

#[wasm_bindgen]
pub async fn psxmp_get_subject(file: String) -> Option<String> {
    let bytes = opfs_read(&file).await.ok()?;
    let n = xmpkit_body::read_subject_count(&bytes);
    if n <= 0 { return None; }
    let items: Vec<String> = (0..n).filter_map(|i| xmpkit_body::read_subject_at(&bytes, i)).collect();
    if items.is_empty() { None } else { Some(serde_json::to_string(&items).unwrap()) }
}

#[wasm_bindgen]
pub async fn psxmp_get_like(file: String) -> bool {
    let Ok(bytes) = opfs_read(&file).await else { return false };
    (1..=5).contains(&xmpkit_body::read_rating(&bytes))
}

#[wasm_bindgen]
pub fn psxmp_get_url(file: String) -> String {
    // OPFS is a flat per-origin namespace — the "URL" of a file IS its name.
    file
}

// ---------- Character cast / image edit (in-memory) ----------

#[wasm_bindgen]
pub fn psxmp_set_character_cast(a: String, b: String) {
    *CHARACTER_CAST.lock().unwrap() = Some((a, b));
}

#[wasm_bindgen]
pub fn psxmp_get_character_cast() -> Option<String> {
    CHARACTER_CAST.lock().unwrap().clone().map(|(a, b)| serde_json::to_string(&[a, b]).unwrap())
}

#[wasm_bindgen]
pub fn psxmp_clear_character_cast() {
    *CHARACTER_CAST.lock().unwrap() = None;
}

#[wasm_bindgen]
pub fn psxmp_set_image_edit(file: String) {
    *IMAGE_EDIT.lock().unwrap() = Some(file);
}

#[wasm_bindgen]
pub fn psxmp_get_image_edit() -> Option<String> {
    IMAGE_EDIT.lock().unwrap().clone()
}

#[wasm_bindgen]
pub fn psxmp_clear_image_edit() {
    *IMAGE_EDIT.lock().unwrap() = None;
}
