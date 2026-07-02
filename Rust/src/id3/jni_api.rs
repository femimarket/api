use jni::JNIEnv;
use jni::objects::{JByteArray, JClass};
use jni::sys::jstring;

use crate::id3::sylt::core_extract_sylt;

/// JNI entry point for Android/Kotlin. Reads the first SYLT frame from the
/// `bytes` ByteArray and returns the timed lines as a JSON string ("[]" when
/// the file has none).
///
/// Kotlin side:
/// ```kotlin
/// class Id3Jvm {
///     companion object { init { System.loadLibrary("id3_ffi") } }
///     external fun extractSylt(bytes: ByteArray): String
/// }
/// ```
#[unsafe(no_mangle)]
pub extern "system" fn Java_market_femi_api_Id3Jvm_extractSylt<'l>(
    env: JNIEnv<'l>,
    _class: JClass<'l>,
    bytes: JByteArray<'l>,
) -> jstring {
    let input = env.convert_byte_array(&bytes).unwrap_or_default();
    let json = core_extract_sylt(&input);
    env.new_string(json)
        .map(|s| s.into_raw())
        .unwrap_or(std::ptr::null_mut())
}
