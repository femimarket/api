//! Android backend — JNI wrappers consumed by `ProjectServiceJvm.kt`. Each
//! call reads bytes from the given filesystem path, delegates to
//! `shared::xmpkit_body` (the same bytes-based logic wasm uses), and writes
//! bytes back where applicable.

use super::shared::xmpkit_body;
use jni::objects::{JClass, JObjectArray, JString};
use jni::sys::{jint, jstring};
use jni::JNIEnv;
use std::fs;

fn jstr(env: &mut JNIEnv, s: JString) -> Option<String> {
    if s.is_null() { return None; }
    env.get_string(&s).ok().map(Into::into)
}

#[no_mangle]
pub extern "system" fn Java_market_femi_api_ProjectServiceJvm_psxmpEmbed<'l>(
    mut env: JNIEnv<'l>, _class: JClass<'l>,
    path: JString<'l>, prompt: JString<'l>, model: JString<'l>, subject: JObjectArray<'l>,
) -> jint {
    let Some(p) = jstr(&mut env, path) else { return -1 };
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
    let refs: Vec<&str> = items.iter().map(|s| s.as_str()).collect();
    let Ok(bytes) = fs::read(&p) else { return -1 };
    let Some(out) = xmpkit_body::embed(&bytes, pr.as_deref(), md.as_deref(), &refs) else { return -1 };
    if fs::write(&p, out).is_err() { return -1 }
    0
}

#[no_mangle]
pub extern "system" fn Java_market_femi_api_ProjectServiceJvm_psxmpReadPrompt<'l>(
    mut env: JNIEnv<'l>, _class: JClass<'l>, path: JString<'l>,
) -> jstring {
    let Some(p) = jstr(&mut env, path) else { return std::ptr::null_mut() };
    let Ok(bytes) = fs::read(&p) else { return std::ptr::null_mut() };
    match xmpkit_body::read_prompt(&bytes) {
        Some(s) if !s.is_empty() => env.new_string(s).map(|j| j.into_raw()).unwrap_or(std::ptr::null_mut()),
        _ => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_market_femi_api_ProjectServiceJvm_psxmpReadModel<'l>(
    mut env: JNIEnv<'l>, _class: JClass<'l>, path: JString<'l>,
) -> jstring {
    let Some(p) = jstr(&mut env, path) else { return std::ptr::null_mut() };
    let Ok(bytes) = fs::read(&p) else { return std::ptr::null_mut() };
    match xmpkit_body::read_model(&bytes) {
        Some(s) if !s.is_empty() => env.new_string(s).map(|j| j.into_raw()).unwrap_or(std::ptr::null_mut()),
        _ => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_market_femi_api_ProjectServiceJvm_psxmpReadSubjectCount<'l>(
    mut env: JNIEnv<'l>, _class: JClass<'l>, path: JString<'l>,
) -> jint {
    let Some(p) = jstr(&mut env, path) else { return -1 };
    let Ok(bytes) = fs::read(&p) else { return -1 };
    xmpkit_body::read_subject_count(&bytes)
}

#[no_mangle]
pub extern "system" fn Java_market_femi_api_ProjectServiceJvm_psxmpReadSubjectAt<'l>(
    mut env: JNIEnv<'l>, _class: JClass<'l>, path: JString<'l>, index: jint,
) -> jstring {
    let Some(p) = jstr(&mut env, path) else { return std::ptr::null_mut() };
    let Ok(bytes) = fs::read(&p) else { return std::ptr::null_mut() };
    match xmpkit_body::read_subject_at(&bytes, index) {
        Some(s) => env.new_string(s).map(|j| j.into_raw()).unwrap_or(std::ptr::null_mut()),
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_market_femi_api_ProjectServiceJvm_psxmpSetRating<'l>(
    mut env: JNIEnv<'l>, _class: JClass<'l>, path: JString<'l>, rating: jint,
) -> jint {
    let Some(p) = jstr(&mut env, path) else { return -1 };
    let Ok(bytes) = fs::read(&p) else { return -1 };
    let Some(out) = xmpkit_body::set_rating(&bytes, rating) else { return -1 };
    if fs::write(&p, out).is_err() { return -1 }
    0
}

#[no_mangle]
pub extern "system" fn Java_market_femi_api_ProjectServiceJvm_psxmpReadRating<'l>(
    mut env: JNIEnv<'l>, _class: JClass<'l>, path: JString<'l>,
) -> jint {
    let Some(p) = jstr(&mut env, path) else { return -1 };
    let Ok(bytes) = fs::read(&p) else { return -1 };
    xmpkit_body::read_rating(&bytes)
}
