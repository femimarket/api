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

fn jstring_to_cstring(env: &mut JNIEnv, jstr: JString) -> Option<CString> {
    if jstr.is_null() { return None; }
    let s: String = env.get_string(&jstr).ok()?.into();
    CString::new(s).ok()
}

macro_rules! jni_fn_3 {
    ($name:ident, $ffi_fn:ident) => {
        #[no_mangle]
        pub extern "system" fn $name<'l>(
            mut env: JNIEnv<'l>, _class: JClass<'l>,
            user: JString<'l>, password: JString<'l>, param3: JString<'l>,
            cancel_flag: jlong,
        ) -> jbyteArray {
            let u = jstring_to_cstring(&mut env, user);
            let pw = jstring_to_cstring(&mut env, password);
            let p3 = jstring_to_cstring(&mut env, param3);
            let mut len = 0usize;
            let ptr = $ffi_fn(
                u.as_ref().map(|x| x.as_ptr()).unwrap_or(std::ptr::null()),
                pw.as_ref().map(|x| x.as_ptr()).unwrap_or(std::ptr::null()),
                p3.as_ref().map(|x| x.as_ptr()).unwrap_or(std::ptr::null()),
                cancel_flag as *const u8, &mut len,
            );
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
    }
}

macro_rules! jni_fn_4 {
    ($name:ident, $ffi_fn:ident) => {
        #[no_mangle]
        pub extern "system" fn $name<'l>(
            mut env: JNIEnv<'l>, _class: JClass<'l>,
            user: JString<'l>, password: JString<'l>, param3: JString<'l>, param4: JString<'l>,
            cancel_flag: jlong,
        ) -> jbyteArray {
            let u = jstring_to_cstring(&mut env, user);
            let pw = jstring_to_cstring(&mut env, password);
            let p3 = jstring_to_cstring(&mut env, param3);
            let p4 = jstring_to_cstring(&mut env, param4);
            let mut len = 0usize;
            let ptr = $ffi_fn(
                u.as_ref().map(|x| x.as_ptr()).unwrap_or(std::ptr::null()),
                pw.as_ref().map(|x| x.as_ptr()).unwrap_or(std::ptr::null()),
                p3.as_ref().map(|x| x.as_ptr()).unwrap_or(std::ptr::null()),
                p4.as_ref().map(|x| x.as_ptr()).unwrap_or(std::ptr::null()),
                cancel_flag as *const u8, &mut len,
            );
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
    }
}

macro_rules! jni_fn_5 {
    ($name:ident, $ffi_fn:ident) => {
        #[no_mangle]
        pub extern "system" fn $name<'l>(
            mut env: JNIEnv<'l>, _class: JClass<'l>,
            user: JString<'l>, password: JString<'l>, param3: JString<'l>, param4: JString<'l>, param5: JString<'l>,
            cancel_flag: jlong,
        ) -> jbyteArray {
            let u = jstring_to_cstring(&mut env, user);
            let pw = jstring_to_cstring(&mut env, password);
            let p3 = jstring_to_cstring(&mut env, param3);
            let p4 = jstring_to_cstring(&mut env, param4);
            let p5 = jstring_to_cstring(&mut env, param5);
            let mut len = 0usize;
            let ptr = $ffi_fn(
                u.as_ref().map(|x| x.as_ptr()).unwrap_or(std::ptr::null()),
                pw.as_ref().map(|x| x.as_ptr()).unwrap_or(std::ptr::null()),
                p3.as_ref().map(|x| x.as_ptr()).unwrap_or(std::ptr::null()),
                p4.as_ref().map(|x| x.as_ptr()).unwrap_or(std::ptr::null()),
                p5.as_ref().map(|x| x.as_ptr()).unwrap_or(std::ptr::null()),
                cancel_flag as *const u8, &mut len,
            );
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
    }
}

jni_fn_3!(Java_market_femi_api_FemiApiJvm_rustFfiZImageTurbo, rust_ffi_z_image_turbo);
jni_fn_3!(Java_market_femi_api_FemiApiJvm_rustFfiQwen3AsrFlash, rust_ffi_qwen3_asr_flash);
jni_fn_3!(Java_market_femi_api_FemiApiJvm_rustFfiQwen3635bA3b, rust_ffi_qwen3_6_35b_a3b);
jni_fn_3!(Java_market_femi_api_FemiApiJvm_rustFfiNanoBanana2, rust_ffi_nano_banana2);
jni_fn_3!(Java_market_femi_api_FemiApiJvm_rustFfiFlux2Pro, rust_ffi_flux2_pro);
jni_fn_4!(Java_market_femi_api_FemiApiJvm_rustFfiFlux2DevI2i, rust_ffi_flux2_dev_i2i);
jni_fn_5!(Java_market_femi_api_FemiApiJvm_rustFfiFlux2KleinI2i, rust_ffi_flux2_klein_i2i);
jni_fn_5!(Java_market_femi_api_FemiApiJvm_rustFfiLtx23a2v, rust_ffi_ltx2_3a2v);
jni_fn_4!(Java_market_femi_api_FemiApiJvm_rustFfiLtx23a2vComfyui, rust_ffi_ltx2_3a2v_comfyui);
