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
/// Port of Generate2/LyricExtractor.swift: SYLT timestamps are per-WORD; split
/// each word's text on `\n` to close lines, join words in a line with a space,
/// then duration = next.start − this.start.
pub(crate) fn core_extract_sylt(bytes: &[u8]) -> String {
    let lines = read_lines(bytes).unwrap_or_default();
    serde_json::to_string(&lines).unwrap_or_else(|_| "[]".to_owned())
}

fn read_lines(bytes: &[u8]) -> Option<Vec<Line>> {
    let tag = Tag::read_from2(Cursor::new(bytes)).ok()?;
    let sync = tag.synchronised_lyrics().next()?;
    let raw = &sync.content; // Vec<(word_start_ms, word_text)>
    if raw.is_empty() {
        return None;
    }

    struct Pending {
        start_ms: i64,
        words: Vec<String>,
    }
    let mut lines: Vec<(i64, String)> = Vec::new();
    let mut pending: Option<Pending> = None;

    for (time, text) in raw {
        let word_start_ms = i64::from(*time);
        let parts: Vec<&str> = text.split('\n').collect();
        for (i, part) in parts.iter().enumerate() {
            let token = part.trim();
            if !token.is_empty() {
                let p = pending.get_or_insert(Pending { start_ms: word_start_ms, words: Vec::new() });
                p.words.push(token.to_owned());
            }
            if i < parts.len() - 1 {
                if let Some(p) = pending.take() {
                    if !p.words.is_empty() {
                        lines.push((p.start_ms, p.words.join(" ")));
                    }
                }
            }
        }
    }
    if let Some(p) = pending.take() {
        if !p.words.is_empty() {
            lines.push((p.start_ms, p.words.join(" ")));
        }
    }

    if lines.is_empty() {
        return None;
    }

    let out = (0..lines.len())
        .map(|i| {
            let (start_ms, text) = &lines[i];
            let end_ms = if i + 1 < lines.len() { lines[i + 1].0 } else { *start_ms };
            Line {
                index: i as u32,
                text: text.clone(),
                start_ms: *start_ms,
                duration_ms: (end_ms - start_ms).max(0),
            }
        })
        .collect();
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
