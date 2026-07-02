
//! Shared across all backends.
//!
//! - XMP namespace constants (used by Apple, Android, WASM).
//! - `xmpkit_body`: pure-Rust bytes-in/bytes-out XMP logic used by both
//!   Android and WASM. Compiled only when one of those targets is active.

pub const NS_DC: &str = "http://purl.org/dc/elements/1.1/";
pub const NS_XMP: &str = "http://ns.adobe.com/xap/1.0/";
pub const NS_IPTC_EXT: &str = "http://iptc.org/std/Iptc4xmpExt/2008-02-29/";

#[cfg(any(target_os = "android", target_arch = "wasm32"))]
pub mod xmpkit_body {
    use super::{NS_DC, NS_IPTC_EXT, NS_XMP};
    use xmpkit::core::metadata::XmpMeta;
    use xmpkit::files::file::XmpFile;
    use xmpkit::files::handler::XmpOptions;
    use xmpkit::types::value::XmpValue;

    fn load(bytes: &[u8]) -> Option<(XmpFile, XmpMeta)> {
        let mut xf = XmpFile::new();
        xf.from_bytes_with(bytes, XmpOptions::default().for_update()).ok()?;
        let meta = xf.get_xmp().cloned().unwrap_or_else(XmpMeta::new);
        Some((xf, meta))
    }

    pub fn embed(bytes: &[u8], prompt: Option<&str>, model: Option<&str>, subject: &[&str]) -> Option<Vec<u8>> {
        let (mut xf, mut meta) = load(bytes)?;
        if let Some(s) = prompt {
            meta.set_localized_text(NS_DC, "description", "", "x-default", s).ok()?;
            meta.set_property(NS_IPTC_EXT, "AIPromptInformation", XmpValue::from(s)).ok()?;
        }
        if let Some(s) = model {
            meta.set_property(NS_XMP, "CreatorTool", XmpValue::from(s)).ok()?;
            meta.set_property(NS_IPTC_EXT, "AISystemUsed", XmpValue::from(s)).ok()?;
        }
        if !subject.is_empty() {
            let _ = meta.delete_property(NS_DC, "subject");
            for it in subject {
                meta.append_array_item(NS_DC, "subject", XmpValue::from(*it)).ok()?;
            }
        }
        xf.put_xmp(meta);
        xf.write_to_bytes().ok()
    }

    pub fn set_rating(bytes: &[u8], rating: i32) -> Option<Vec<u8>> {
        let (mut xf, mut meta) = load(bytes)?;
        meta.set_property(NS_XMP, "Rating", XmpValue::Integer(rating as i64)).ok()?;
        xf.put_xmp(meta);
        xf.write_to_bytes().ok()
    }

    fn read_only(bytes: &[u8]) -> Option<XmpMeta> {
        let mut xf = XmpFile::new();
        xf.from_bytes(bytes).ok()?;
        Some(xf.get_xmp().cloned().unwrap_or_else(XmpMeta::new))
    }

    pub fn read_prompt(bytes: &[u8]) -> Option<String> {
        let m = read_only(bytes)?;
        if let Some(v) = m.get_property(NS_IPTC_EXT, "AIPromptInformation") {
            if let Some(s) = v.as_str() { return Some(s.to_string()); }
        }
        m.get_localized_text(NS_DC, "description", "", "x-default").map(|(v, _)| v)
    }
    pub fn read_model(bytes: &[u8]) -> Option<String> {
        let m = read_only(bytes)?;
        if let Some(v) = m.get_property(NS_IPTC_EXT, "AISystemUsed") {
            if let Some(s) = v.as_str() { return Some(s.to_string()); }
        }
        m.get_property(NS_XMP, "CreatorTool").and_then(|v| v.as_str().map(|s| s.to_string()))
    }
    pub fn read_subject_count(bytes: &[u8]) -> i32 {
        let Some(m) = read_only(bytes) else { return -1 };
        m.get_array_size(NS_DC, "subject").unwrap_or(0) as i32
    }
    pub fn read_subject_at(bytes: &[u8], index: i32) -> Option<String> {
        let m = read_only(bytes)?;
        m.get_array_item(NS_DC, "subject", index as usize)
            .and_then(|v| v.as_str().map(|s| s.to_string()))
    }
    pub fn read_rating(bytes: &[u8]) -> i32 {
        let Some(m) = read_only(bytes) else { return -1 };
        match m.get_property(NS_XMP, "Rating") {
            Some(XmpValue::Integer(i)) => i as i32,
            _ => -100,
        }
    }
    pub fn read_property(bytes: &[u8], ns: &str, name: &str) -> Option<String> {
        let m = read_only(bytes)?;
        m.get_property(ns, name).and_then(|v| v.as_str().map(|s| s.to_string()))
    }
}
