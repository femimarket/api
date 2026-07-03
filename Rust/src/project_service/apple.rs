//! Apple backend — Rust owns the Documents root. Every entry takes a
//! filename (not a path); Rust resolves the sandboxed Documents/ dir via
//! `dirs::document_dir` and prepends it internally. XMP metadata mutation
//! goes through `xmp_toolkit`'s smart handlers.

use super::shared::{NS_DC, NS_IPTC_EXT, NS_XMP};
use std::path::PathBuf;
use std::sync::Mutex;
use xmp_toolkit::{OpenFileOptions, XmpFile, XmpMeta, XmpValue};

const AUDIO_EXTS: &[&str] = &[
    "mp3", "m4a", "wav", "aac", "caf", "aiff", "aif", "flac", "ogg", "opus",
];

fn documents_dir() -> PathBuf {
    dirs::document_dir().unwrap_or_else(|| PathBuf::from("."))
}

pub fn get_url(name: &str) -> String {
    let leaf = PathBuf::from(name).file_name().map(|s| s.to_os_string()).unwrap_or_default();
    documents_dir().join(leaf).to_string_lossy().into_owned()
}

fn is_audio(name: &str) -> bool {
    PathBuf::from(name)
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| AUDIO_EXTS.iter().any(|a| a.eq_ignore_ascii_case(e)))
        .unwrap_or(false)
}

// ---------- Save ----------

pub fn save_file(name: &str, bytes: &[u8], prompt: Option<&str>, model: Option<&str>, subject: &[&str]) {
    let path = get_url(name);
    std::fs::write(&path, bytes).expect("save_file: write failed");
    if prompt.is_none() && model.is_none() && subject.is_empty() {
        return;
    }
    embed_xmp(&path, prompt, model, subject);
}

pub fn save_audio(name: &str, bytes: &[u8]) {
    for existing in list_generations() {
        if is_audio(&existing) {
            let _ = std::fs::remove_file(documents_dir().join(&existing));
        }
    }
    let path = get_url(name);
    std::fs::write(&path, bytes).expect("save_audio: write failed");
}

// ---------- Read ----------

pub fn list_generations() -> Vec<String> {
    let Ok(entries) = std::fs::read_dir(documents_dir()) else { return Vec::new() };
    entries
        .filter_map(|e| e.ok())
        .filter_map(|e| e.file_name().into_string().ok())
        .collect()
}

pub fn get_audio() -> Option<String> {
    list_generations().into_iter().find(|n| is_audio(n))
}

pub fn get_prompt(file: &str) -> Option<String> {
    let mut f = open_for_read(&get_url(file))?;
    let m = load_meta(&mut f);
    if let Some(v) = m.property(NS_IPTC_EXT, "AIPromptInformation") { return Some(v.value); }
    m.localized_text(NS_DC, "description", None, "x-default").map(|(v, _)| v.value)
}

pub fn get_model(file: &str) -> Option<String> {
    let mut f = open_for_read(&get_url(file))?;
    let m = load_meta(&mut f);
    if let Some(v) = m.property(NS_IPTC_EXT, "AISystemUsed") { return Some(v.value); }
    m.property(NS_XMP, "CreatorTool").map(|v| v.value)
}

pub fn get_subject(file: &str) -> Vec<String> {
    let Some(mut f) = open_for_read(&get_url(file)) else { return Vec::new() };
    let m = load_meta(&mut f);
    let n = m.array_len(NS_DC, "subject");
    (0..n).filter_map(|i| m.array_item(NS_DC, "subject", (i + 1) as i32).map(|v| v.value)).collect()
}

pub fn read_property(file: &str, ns: &str, name: &str) -> Option<String> {
    let mut f = open_for_read(&get_url(file))?;
    let m = load_meta(&mut f);
    m.property(ns, name).map(|v| v.value)
}

pub fn get_like(file: &str) -> bool {
    let Some(mut f) = open_for_read(&get_url(file)) else { return false };
    let m = load_meta(&mut f);
    match m.property_i32(NS_XMP, "Rating") {
        Some(v) => (1..=5).contains(&v.value),
        None => false,
    }
}

// ---------- Rating (like) ----------

pub fn like(file: &str, liked: bool) {
    let rating = if liked { 5 } else { 0 };
    let path = get_url(file);
    let Ok(mut file) = open_for_update(&path) else { return };
    let mut meta = load_meta(&mut file);
    if meta.set_property_i32(NS_XMP, "Rating", &XmpValue::new(rating)).is_err() { return; }
    if !file.can_put_xmp(&meta) { return; }
    let _ = file.put_xmp(&meta);
    file.close();
}

// ---------- In-memory character cast / image edit ----------

static CHARACTER_CAST: Mutex<Option<(String, String)>> = Mutex::new(None);
static IMAGE_EDIT: Mutex<Option<String>> = Mutex::new(None);

pub fn set_character_cast(a: &str, b: &str) {
    *CHARACTER_CAST.lock().unwrap() = Some((a.to_owned(), b.to_owned()));
}
pub fn get_character_cast() -> Option<(String, String)> {
    CHARACTER_CAST.lock().unwrap().clone()
}
pub fn clear_character_cast() {
    *CHARACTER_CAST.lock().unwrap() = None;
}

pub fn set_image_edit(file: &str) {
    *IMAGE_EDIT.lock().unwrap() = Some(file.to_owned());
}
pub fn get_image_edit() -> Option<String> {
    IMAGE_EDIT.lock().unwrap().clone()
}
pub fn clear_image_edit() {
    *IMAGE_EDIT.lock().unwrap() = None;
}

// ---------- XMP internals ----------

fn open_for_update(path: &str) -> Result<XmpFile, ()> {
    let mut f = XmpFile::new().map_err(|_| ())?;
    f.open_file(path, OpenFileOptions::default().for_update().use_smart_handler())
        .map_err(|_| ())?;
    Ok(f)
}

fn open_for_read(path: &str) -> Option<XmpFile> {
    let mut f = XmpFile::new().ok()?;
    f.open_file(path, OpenFileOptions::default().for_read().use_smart_handler()).ok()?;
    Some(f)
}

fn load_meta(f: &mut XmpFile) -> XmpMeta {
    f.xmp().unwrap_or_else(|| XmpMeta::new().unwrap())
}

fn embed_xmp(path: &str, prompt: Option<&str>, model: Option<&str>, subject: &[&str]) {
    let Ok(mut file) = open_for_update(path) else { return };
    let mut meta = load_meta(&mut file);
    if let Some(s) = prompt {
        let _ = meta.set_localized_text(NS_DC, "description", None, "x-default", s);
        let _ = meta.set_property(NS_IPTC_EXT, "AIPromptInformation", &XmpValue::new(s.to_owned()));
    }
    if let Some(s) = model {
        let _ = meta.set_property(NS_XMP, "CreatorTool", &XmpValue::new(s.to_owned()));
        let _ = meta.set_property(NS_IPTC_EXT, "AISystemUsed", &XmpValue::new(s.to_owned()));
    }
    if !subject.is_empty() {
        let _ = meta.delete_property(NS_DC, "subject");
        let arr = XmpValue::new("subject".to_owned()).set_is_array(true);
        for it in subject {
            let _ = meta.append_array_item(NS_DC, &arr, &XmpValue::new((*it).to_owned()));
        }
    }
    if !file.can_put_xmp(&meta) { return; }
    let _ = file.put_xmp(&meta);
    file.close();
}
