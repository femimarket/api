//! WASM backend — path is an OPFS filename in the origin root. Reads/writes
//! go through `web-sys` bindings; the metadata mutation itself runs through
//! the same `xmpkit_body` as Android.

use super::shared::xmpkit_body;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

async fn opfs_root() -> Result<web_sys::FileSystemDirectoryHandle, JsValue> {
    let win = web_sys::window().ok_or_else(|| JsValue::from_str("no window"))?;
    let storage = win.navigator().storage();
    let dir = JsFuture::from(storage.get_directory()).await?;
    dir.dyn_into()
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

#[wasm_bindgen]
pub async fn psxmp_embed(
    path: String,
    prompt: Option<String>,
    model: Option<String>,
    subjects: Vec<String>,
) -> i32 {
    let Ok(bytes) = opfs_read(&path).await else { return -1 };
    let s: Vec<&str> = subjects.iter().map(|x| x.as_str()).collect();
    let Some(out) = xmpkit_body::embed(&bytes, prompt.as_deref(), model.as_deref(), &s) else { return -1 };
    if opfs_write(&path, &out).await.is_err() { return -1 }
    0
}

#[wasm_bindgen]
pub async fn psxmp_read_prompt(path: String) -> Option<String> {
    let bytes = opfs_read(&path).await.ok()?;
    xmpkit_body::read_prompt(&bytes)
}

#[wasm_bindgen]
pub async fn psxmp_read_model(path: String) -> Option<String> {
    let bytes = opfs_read(&path).await.ok()?;
    xmpkit_body::read_model(&bytes)
}

#[wasm_bindgen]
pub async fn psxmp_read_subject_count(path: String) -> i32 {
    let Ok(bytes) = opfs_read(&path).await else { return -1 };
    xmpkit_body::read_subject_count(&bytes)
}

#[wasm_bindgen]
pub async fn psxmp_read_subject_at(path: String, index: i32) -> Option<String> {
    let bytes = opfs_read(&path).await.ok()?;
    xmpkit_body::read_subject_at(&bytes, index)
}

#[wasm_bindgen]
pub async fn psxmp_set_rating(path: String, rating: i32) -> i32 {
    let Ok(bytes) = opfs_read(&path).await else { return -1 };
    let Some(out) = xmpkit_body::set_rating(&bytes, rating) else { return -1 };
    if opfs_write(&path, &out).await.is_err() { return -1 }
    0
}

#[wasm_bindgen]
pub async fn psxmp_read_rating(path: String) -> i32 {
    let Ok(bytes) = opfs_read(&path).await else { return -1 };
    xmpkit_body::read_rating(&bytes)
}

#[wasm_bindgen]
pub async fn psxmp_read_property(path: String, namespace_uri: String, property_name: String) -> Option<String> {
    let bytes = opfs_read(&path).await.ok()?;
    xmpkit_body::read_property(&bytes, &namespace_uri, &property_name)
}


