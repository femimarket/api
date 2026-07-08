use jni::objects::{JClass, JString};
use jni::JNIEnv;
use jni::sys::{jbyteArray, jlong};
use std::ffi::CString;

use crate::api::flux2_dev_i2i::native::rust_ffi_flux2_dev_i2i;
use crate::api::flux2_klein_i2i::native::rust_ffi_flux2_klein_i2i;
use crate::api::flux2_pro::native::rust_ffi_flux2_pro;
use crate::api::ltx2_3a2v::native::rust_ffi_ltx2_3a2v;
use crate::api::ltx2_3a2v_comfyui::native::rust_ffi_ltx2_3a2v_comfyui;
use crate::api::nano_banana2::native::rust_ffi_nano_banana2;
use crate::api::qwen3_6_35b_a3b::native::rust_ffi_qwen3_6_35b_a3b;
use crate::api::qwen3_asr_flash::native::rust_ffi_qwen3_asr_flash;
use crate::api::z_image_turbo::native::rust_ffi_z_image_turbo;
use crate::api::qwen3_6_prompts::native::{
    rust_ffi_qwen3_6_35b_a3b_0gen_music_video_prompt,
    rust_ffi_qwen3_6_35b_a3b_1gen_new_angle_from_xmp_image_prompt,
    rust_ffi_qwen3_6_35b_a3b_1gen_random_idea_from_xmp_prompt,
    rust_ffi_qwen3_6_35b_a3b_1gen_3_variants_from_xmp_prompt_as_jsonarray,
    rust_ffi_qwen3_6_35b_a3b_2gen_augment_idea_from_xmp_prompt_and_text,
    rust_ffi_qwen3_6_35b_a3b_2gen_50_100_word_multishot_timestamped_prompt_from_2_xmp_image_prompts,
    rust_ffi_qwen3_6_35b_a3b_3gen_50_100_word_multishot_timestamped_prompt_from_3_xmp_image_prompts,
    rust_ffi_qwen3_6_35b_a3b_4gen_augment_idea_from_3_xmp_prompts_and_text,
};

fn jstring_to_cstring(env: &mut JNIEnv, jstr: JString) -> Option<CString> {
    if jstr.is_null() { return None; }
    let s: String = env.get_string(&jstr).ok()?.into();
    CString::new(s).ok()
}

// Copy a Rust-owned heap buffer (ptr,len) into a fresh jbyteArray and free it.
fn to_jbytes<'l>(env: &JNIEnv<'l>, ptr: *mut u8, len: usize) -> jbyteArray {
    if ptr.is_null() || len == 0 {
        return env.new_byte_array(0).unwrap().into_raw();
    }
    let slice = unsafe { std::slice::from_raw_parts(ptr, len) };
    let jbytes = env.new_byte_array(len as i32).unwrap();
    let jslice = unsafe { std::slice::from_raw_parts(slice.as_ptr() as *const i8, slice.len()) };
    let _ = env.set_byte_array_region(&jbytes, 0, jslice);
    unsafe { let _ = Box::from_raw(std::slice::from_raw_parts_mut(ptr, len)); }
    jbytes.into_raw()
}

// N-string JNI wrappers. Each takes N JString args + a cancel-flag jlong (raw
// address of a u8 the caller flips to 1 to cancel), and returns a jbyteArray.
macro_rules! jni_fn_0 {
    ($name:ident, $ffi_fn:ident) => {
        #[no_mangle]
        pub extern "system" fn $name<'l>(env: JNIEnv<'l>, _class: JClass<'l>, cancel_flag: jlong) -> jbyteArray {
            let mut len = 0usize;
            let ptr = $ffi_fn(cancel_flag as *const u8, &mut len);
            to_jbytes(&env, ptr, len)
        }
    }
}
macro_rules! jni_fn_1 {
    ($name:ident, $ffi_fn:ident) => {
        #[no_mangle]
        pub extern "system" fn $name<'l>(mut env: JNIEnv<'l>, _class: JClass<'l>, p1: JString<'l>, cancel_flag: jlong) -> jbyteArray {
            let a1 = jstring_to_cstring(&mut env, p1);
            let mut len = 0usize;
            let ptr = $ffi_fn(
                a1.as_ref().map(|x| x.as_ptr()).unwrap_or(std::ptr::null()),
                cancel_flag as *const u8, &mut len,
            );
            to_jbytes(&env, ptr, len)
        }
    }
}
macro_rules! jni_fn_2 {
    ($name:ident, $ffi_fn:ident) => {
        #[no_mangle]
        pub extern "system" fn $name<'l>(mut env: JNIEnv<'l>, _class: JClass<'l>, p1: JString<'l>, p2: JString<'l>, cancel_flag: jlong) -> jbyteArray {
            let a1 = jstring_to_cstring(&mut env, p1);
            let a2 = jstring_to_cstring(&mut env, p2);
            let mut len = 0usize;
            let ptr = $ffi_fn(
                a1.as_ref().map(|x| x.as_ptr()).unwrap_or(std::ptr::null()),
                a2.as_ref().map(|x| x.as_ptr()).unwrap_or(std::ptr::null()),
                cancel_flag as *const u8, &mut len,
            );
            to_jbytes(&env, ptr, len)
        }
    }
}
macro_rules! jni_fn_3 {
    ($name:ident, $ffi_fn:ident) => {
        #[no_mangle]
        pub extern "system" fn $name<'l>(mut env: JNIEnv<'l>, _class: JClass<'l>, p1: JString<'l>, p2: JString<'l>, p3: JString<'l>, cancel_flag: jlong) -> jbyteArray {
            let a1 = jstring_to_cstring(&mut env, p1);
            let a2 = jstring_to_cstring(&mut env, p2);
            let a3 = jstring_to_cstring(&mut env, p3);
            let mut len = 0usize;
            let ptr = $ffi_fn(
                a1.as_ref().map(|x| x.as_ptr()).unwrap_or(std::ptr::null()),
                a2.as_ref().map(|x| x.as_ptr()).unwrap_or(std::ptr::null()),
                a3.as_ref().map(|x| x.as_ptr()).unwrap_or(std::ptr::null()),
                cancel_flag as *const u8, &mut len,
            );
            to_jbytes(&env, ptr, len)
        }
    }
}
macro_rules! jni_fn_4 {
    ($name:ident, $ffi_fn:ident) => {
        #[no_mangle]
        pub extern "system" fn $name<'l>(mut env: JNIEnv<'l>, _class: JClass<'l>, p1: JString<'l>, p2: JString<'l>, p3: JString<'l>, p4: JString<'l>, cancel_flag: jlong) -> jbyteArray {
            let a1 = jstring_to_cstring(&mut env, p1);
            let a2 = jstring_to_cstring(&mut env, p2);
            let a3 = jstring_to_cstring(&mut env, p3);
            let a4 = jstring_to_cstring(&mut env, p4);
            let mut len = 0usize;
            let ptr = $ffi_fn(
                a1.as_ref().map(|x| x.as_ptr()).unwrap_or(std::ptr::null()),
                a2.as_ref().map(|x| x.as_ptr()).unwrap_or(std::ptr::null()),
                a3.as_ref().map(|x| x.as_ptr()).unwrap_or(std::ptr::null()),
                a4.as_ref().map(|x| x.as_ptr()).unwrap_or(std::ptr::null()),
                cancel_flag as *const u8, &mut len,
            );
            to_jbytes(&env, ptr, len)
        }
    }
}

// Existing endpoints (now authless).
jni_fn_1!(Java_market_femi_api_FemiApiJvm_rustFfiZImageTurbo, rust_ffi_z_image_turbo);
jni_fn_1!(Java_market_femi_api_FemiApiJvm_rustFfiQwen3AsrFlash, rust_ffi_qwen3_asr_flash);
jni_fn_1!(Java_market_femi_api_FemiApiJvm_rustFfiQwen3635bA3b, rust_ffi_qwen3_6_35b_a3b);
jni_fn_1!(Java_market_femi_api_FemiApiJvm_rustFfiNanoBanana2, rust_ffi_nano_banana2);
jni_fn_1!(Java_market_femi_api_FemiApiJvm_rustFfiFlux2Pro, rust_ffi_flux2_pro);
jni_fn_2!(Java_market_femi_api_FemiApiJvm_rustFfiFlux2DevI2i, rust_ffi_flux2_dev_i2i);
jni_fn_3!(Java_market_femi_api_FemiApiJvm_rustFfiFlux2KleinI2i, rust_ffi_flux2_klein_i2i);
jni_fn_3!(Java_market_femi_api_FemiApiJvm_rustFfiLtx23a2v, rust_ffi_ltx2_3a2v);
jni_fn_4!(Java_market_femi_api_FemiApiJvm_rustFfiLtx23a2vComfyui, rust_ffi_ltx2_3a2v_comfyui);

// New qwen3_6_35b_a3b prompt-gen family.
jni_fn_0!(Java_market_femi_api_FemiApiJvm_rustFfiQwen3635bA3b0GenMusicVideoPrompt, rust_ffi_qwen3_6_35b_a3b_0gen_music_video_prompt);
jni_fn_1!(Java_market_femi_api_FemiApiJvm_rustFfiQwen3635bA3b1GenNewAngleFromXmpImagePrompt, rust_ffi_qwen3_6_35b_a3b_1gen_new_angle_from_xmp_image_prompt);
jni_fn_1!(Java_market_femi_api_FemiApiJvm_rustFfiQwen3635bA3b1GenRandomIdeaFromXmpPrompt, rust_ffi_qwen3_6_35b_a3b_1gen_random_idea_from_xmp_prompt);
jni_fn_1!(Java_market_femi_api_FemiApiJvm_rustFfiQwen3635bA3b1Gen3VariantsFromXmpPromptAsJsonarray, rust_ffi_qwen3_6_35b_a3b_1gen_3_variants_from_xmp_prompt_as_jsonarray);
jni_fn_2!(Java_market_femi_api_FemiApiJvm_rustFfiQwen3635bA3b2GenAugmentIdeaFromXmpPromptAndText, rust_ffi_qwen3_6_35b_a3b_2gen_augment_idea_from_xmp_prompt_and_text);
jni_fn_2!(Java_market_femi_api_FemiApiJvm_rustFfiQwen3635bA3b2Gen50100WordMultishotTimestampedPromptFrom2XmpImagePrompts, rust_ffi_qwen3_6_35b_a3b_2gen_50_100_word_multishot_timestamped_prompt_from_2_xmp_image_prompts);
jni_fn_3!(Java_market_femi_api_FemiApiJvm_rustFfiQwen3635bA3b3Gen50100WordMultishotTimestampedPromptFrom3XmpImagePrompts, rust_ffi_qwen3_6_35b_a3b_3gen_50_100_word_multishot_timestamped_prompt_from_3_xmp_image_prompts);
jni_fn_4!(Java_market_femi_api_FemiApiJvm_rustFfiQwen3635bA3b4GenAugmentIdeaFrom3XmpPromptsAndText, rust_ffi_qwen3_6_35b_a3b_4gen_augment_idea_from_3_xmp_prompts_and_text);
