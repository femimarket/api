use std::io::Cursor;

use ::id3::Tag;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Line {
    index: u32,
    text: String,
    start_ms: i64,
    duration_ms: i64,
}

/// Read the first SYLT (synchronised lyrics) frame from MP3 bytes and return the
/// timed lines as a JSON array string ("[]" when there are none).
///
/// Port of Generate2/LyricExtractor.swift: skip blank lines, duration =
/// next.start − this.start, and number the surviving lines sequentially.
pub(crate) fn core_extract_sylt(bytes: &[u8]) -> String {
    let lines = read_lines(bytes).unwrap_or_default();
    serde_json::to_string(&lines).unwrap_or_else(|_| "[]".to_owned())
}

fn read_lines(bytes: &[u8]) -> Option<Vec<Line>> {
    let tag = Tag::read_from2(Cursor::new(bytes)).ok()?;
    let sync = tag.synchronised_lyrics().next()?;
    let raw = &sync.content; // Vec<(timestamp_ms, text)>
    if raw.is_empty() {
        return None;
    }

    let mut out = Vec::new();
    let mut visible_index = 0;
    for i in 0..raw.len() {
        let (time, text) = &raw[i];
        if text.is_empty() {
            continue;
        }
        let start_ms = i64::from(*time);
        let end_ms = if i + 1 < raw.len() {
            i64::from(raw[i + 1].0)
        } else {
            start_ms
        };
        out.push(Line {
            index: visible_index,
            text: text.clone(),
            start_ms,
            duration_ms: (end_ms - start_ms).max(0),
        });
        visible_index += 1;
    }
    Some(out)
}

#[cfg(not(target_arch = "wasm32"))]
pub mod native {
    use super::*;

    /// Read the first SYLT frame from `bytes` (`bytes_len` long) and return a
    /// heap-allocated UTF-8 JSON string. `out_len` receives the byte length; the
    /// caller must `free()` the returned pointer. Always non-null (worst case
    /// the two-byte string "[]").
    #[no_mangle]
    pub extern "C" fn id3_ffi_extract_sylt(
        bytes: *const u8,
        bytes_len: usize,
        out_len: *mut usize,
    ) -> *mut u8 {
        let input: &[u8] = if bytes.is_null() || bytes_len == 0 {
            &[]
        } else {
            unsafe { std::slice::from_raw_parts(bytes, bytes_len) }
        };
        let out = core_extract_sylt(input).into_bytes();
        let len = out.len();
        unsafe { *out_len = len; }
        if len == 0 { std::ptr::null_mut() } else { Box::into_raw(out.into_boxed_slice()) as *mut u8 }
    }
}

#[cfg(target_arch = "wasm32")]
pub mod wasm {
    use super::*;
    use wasm_bindgen::prelude::*;

    /// Read the first SYLT frame from MP3 `bytes` and return the timed lines as a
    /// JSON array string ("[]" when the file has none). Name kept as
    /// `extract_sylt` so the existing id3-wasm JS wrapper binds unchanged.
    #[wasm_bindgen]
    pub fn extract_sylt(bytes: &[u8]) -> String {
        core_extract_sylt(bytes)
    }
}
