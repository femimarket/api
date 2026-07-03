//! Android backend — JNI wrappers consumed by `ProjectServiceJvm.kt`.
//! Rust owns the Documents root: Kotlin/androidx.startup calls
//! `psxmpInitDocuments(context.filesDir.absolutePath)` once at app start,
//! then all subsequent calls take just a filename. XMP mutation goes
//! through the shared bytes-based `xmpkit_body`.

use super::shared::xmpkit_body;
use jni::objects::{JByteArray, JClass, JObjectArray, JString};
use jni::sys::{jint, jstring};
use jni::JNIEnv;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

const AUDIO_EXTS: &[&str] = &[
    "mp3", "m4a", "wav", "aac", "caf", "aiff", "aif", "flac", "ogg", "opus",
];

static DOCUMENTS: Mutex<Option<String>> = Mutex::new(None);
static CHARACTER_CAST: Mutex<Option<(String, String)>> = Mutex::new(None);
static IMAGE_EDIT: Mutex<Option<String>> = Mutex::new(None);

fn documents_dir() -> PathBuf {
    PathBuf::from(DOCUMENTS.lock().unwrap().clone().unwrap_or_else(|| ".".into()))
}

fn full_path(name: &str) -> PathBuf {
    let leaf = Path::new(name).file_name().map(|s| s.to_os_string()).unwrap_or_default();
    documents_dir().join(leaf)
}

fn is_audio(name: &str) -> bool {
    Path::new(name)
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| AUDIO_EXTS.iter().any(|a| a.eq_ignore_ascii_case(e)))
        .unwrap_or(false)
}

fn list_names() -> Vec<String> {
    let Ok(entries) = fs::read_dir(documents_dir()) else { return Vec::new() };
    entries
        .filter_map(|e| e.ok())
        .filter_map(|e| e.file_name().into_string().ok())
        .collect()
}

fn jstr(env: &mut JNIEnv, s: JString) -> Option<String> {
    if s.is_null() { return None; }
    env.get_string(&s).ok().map(Into::into)
}

fn new_jstring<'l>(env: &JNIEnv<'l>, s: &str) -> jstring {
    env.new_string(s).map(|j| j.into_raw()).unwrap_or(std::ptr::null_mut())
}

// ---------- Init ----------

#[no_mangle]
pub extern "system" fn Java_market_femi_api_ProjectServiceJvm_psxmpInitDocuments<'l>(
    mut env: JNIEnv<'l>, _class: JClass<'l>, path: JString<'l>,
) {
    if let Some(p) = jstr(&mut env, path) {
        *DOCUMENTS.lock().unwrap() = Some(p);
    }
}

// ---------- Save ----------

#[no_mangle]
pub extern "system" fn Java_market_femi_api_ProjectServiceJvm_psxmpSaveFile<'l>(
    mut env: JNIEnv<'l>, _class: JClass<'l>,
    name: JString<'l>, bytes: JByteArray<'l>,
    prompt: JString<'l>, model: JString<'l>, subject: JObjectArray<'l>,
) {
    let Some(n) = jstr(&mut env, name) else { return };
    let Ok(data) = env.convert_byte_array(&bytes) else { return };
    let pr = jstr(&mut env, prompt);
    let md = jstr(&mut env, model);
    let mut items: Vec<String> = Vec::new();
    if !subject.is_null() {
        if let Ok(len) = env.get_array_length(&subject) {
            for i in 0..len {
                if let Ok(obj) = env.get_object_array_element(&subject, i) {
                    if let Some(s) = jstr(&mut env, JString::from(obj)) { items.push(s); }
                }
            }
        }
    }
    let path = full_path(&n);
    let bytes_out = if pr.is_none() && md.is_none() && items.is_empty() {
        data
    } else {
        let refs: Vec<&str> = items.iter().map(|s| s.as_str()).collect();
        xmpkit_body::embed(&data, pr.as_deref(), md.as_deref(), &refs)
    };
    let _ = fs::write(&path, bytes_out);
}

#[no_mangle]
pub extern "system" fn Java_market_femi_api_ProjectServiceJvm_psxmpSaveAudio<'l>(
    mut env: JNIEnv<'l>, _class: JClass<'l>,
    name: JString<'l>, bytes: JByteArray<'l>,
) {
    let Some(n) = jstr(&mut env, name) else { return };
    let Ok(data) = env.convert_byte_array(&bytes) else { return };
    for existing in list_names() {
        if is_audio(&existing) {
            let _ = fs::remove_file(documents_dir().join(&existing));
        }
    }
    let _ = fs::write(full_path(&n), data);
}

#[no_mangle]
pub extern "system" fn Java_market_femi_api_ProjectServiceJvm_psxmpLike<'l>(
    mut env: JNIEnv<'l>, _class: JClass<'l>, file: JString<'l>, liked: jint,
) {
    let Some(f) = jstr(&mut env, file) else { return };
    let Ok(data) = fs::read(full_path(&f)) else { return };
    let out = xmpkit_body::set_rating(&data, if liked != 0 { 5 } else { 0 });
    let _ = fs::write(full_path(&f), out);
}

// ---------- Read ----------

#[no_mangle]
pub extern "system" fn Java_market_femi_api_ProjectServiceJvm_psxmpGetAllGenerations<'l>(
    env: JNIEnv<'l>, _class: JClass<'l>,
) -> jstring {
    let json = serde_json::to_string(&list_names()).unwrap();
    new_jstring(&env, &json)
}

#[no_mangle]
pub extern "system" fn Java_market_femi_api_ProjectServiceJvm_psxmpGetAudio<'l>(
    env: JNIEnv<'l>, _class: JClass<'l>,
) -> jstring {
    match list_names().into_iter().find(|n| is_audio(n)) {
        Some(s) => new_jstring(&env, &s),
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_market_femi_api_ProjectServiceJvm_psxmpGetPrompt<'l>(
    mut env: JNIEnv<'l>, _class: JClass<'l>, file: JString<'l>,
) -> jstring {
    let Some(f) = jstr(&mut env, file) else { return std::ptr::null_mut() };
    let Ok(data) = fs::read(full_path(&f)) else { return std::ptr::null_mut() };
    match xmpkit_body::read_prompt(&data) {
        Some(s) if !s.is_empty() => new_jstring(&env, &s),
        _ => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_market_femi_api_ProjectServiceJvm_psxmpGetModel<'l>(
    mut env: JNIEnv<'l>, _class: JClass<'l>, file: JString<'l>,
) -> jstring {
    let Some(f) = jstr(&mut env, file) else { return std::ptr::null_mut() };
    let Ok(data) = fs::read(full_path(&f)) else { return std::ptr::null_mut() };
    match xmpkit_body::read_model(&data) {
        Some(s) if !s.is_empty() => new_jstring(&env, &s),
        _ => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_market_femi_api_ProjectServiceJvm_psxmpGetSubject<'l>(
    mut env: JNIEnv<'l>, _class: JClass<'l>, file: JString<'l>,
) -> jstring {
    let Some(f) = jstr(&mut env, file) else { return std::ptr::null_mut() };
    let Ok(data) = fs::read(full_path(&f)) else { return std::ptr::null_mut() };
    let count = xmpkit_body::read_subject_count(&data);
    if count <= 0 { return std::ptr::null_mut(); }
    let items: Vec<String> = (0..count)
        .filter_map(|i| xmpkit_body::read_subject_at(&data, i))
        .collect();
    if items.is_empty() {
        std::ptr::null_mut()
    } else {
        let json = serde_json::to_string(&items).unwrap();
        new_jstring(&env, &json)
    }
}

#[no_mangle]
pub extern "system" fn Java_market_femi_api_ProjectServiceJvm_psxmpGetLike<'l>(
    mut env: JNIEnv<'l>, _class: JClass<'l>, file: JString<'l>,
) -> jint {
    let Some(f) = jstr(&mut env, file) else { return 0 };
    let Ok(data) = fs::read(full_path(&f)) else { return 0 };
    let r = xmpkit_body::read_rating(&data);
    if (1..=5).contains(&r) { 1 } else { 0 }
}

#[no_mangle]
pub extern "system" fn Java_market_femi_api_ProjectServiceJvm_psxmpGetUrl<'l>(
    mut env: JNIEnv<'l>, _class: JClass<'l>, file: JString<'l>,
) -> jstring {
    let Some(f) = jstr(&mut env, file) else { return std::ptr::null_mut() };
    new_jstring(&env, &full_path(&f).to_string_lossy())
}

// ---------- Character cast / image edit (in-memory) ----------

#[no_mangle]
pub extern "system" fn Java_market_femi_api_ProjectServiceJvm_psxmpSetCharacterCast<'l>(
    mut env: JNIEnv<'l>, _class: JClass<'l>, a: JString<'l>, b: JString<'l>,
) {
    let (Some(a), Some(b)) = (jstr(&mut env, a), jstr(&mut env, b)) else { return };
    *CHARACTER_CAST.lock().unwrap() = Some((a, b));
}

#[no_mangle]
pub extern "system" fn Java_market_femi_api_ProjectServiceJvm_psxmpGetCharacterCast<'l>(
    env: JNIEnv<'l>, _class: JClass<'l>,
) -> jstring {
    match CHARACTER_CAST.lock().unwrap().clone() {
        Some((a, b)) => {
            let json = serde_json::to_string(&[a, b]).unwrap();
            new_jstring(&env, &json)
        }
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_market_femi_api_ProjectServiceJvm_psxmpClearCharacterCast<'l>(
    _env: JNIEnv<'l>, _class: JClass<'l>,
) {
    *CHARACTER_CAST.lock().unwrap() = None;
}

#[no_mangle]
pub extern "system" fn Java_market_femi_api_ProjectServiceJvm_psxmpSetImageEdit<'l>(
    mut env: JNIEnv<'l>, _class: JClass<'l>, file: JString<'l>,
) {
    let Some(f) = jstr(&mut env, file) else { return };
    *IMAGE_EDIT.lock().unwrap() = Some(f);
}

#[no_mangle]
pub extern "system" fn Java_market_femi_api_ProjectServiceJvm_psxmpGetImageEdit<'l>(
    env: JNIEnv<'l>, _class: JClass<'l>,
) -> jstring {
    match IMAGE_EDIT.lock().unwrap().clone() {
        Some(s) => new_jstring(&env, &s),
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_market_femi_api_ProjectServiceJvm_psxmpClearImageEdit<'l>(
    _env: JNIEnv<'l>, _class: JClass<'l>,
) {
    *IMAGE_EDIT.lock().unwrap() = None;
}
