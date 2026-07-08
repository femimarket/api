
//! Shared across all backends.
//!
//! - XMP namespace constants (used by Apple, Android, WASM).
//! - `xmpkit_body`: pure-Rust bytes-in/bytes-out XMP logic used by both
//!   Android and WASM. Compiled only when one of those targets is active.

pub const NS_DC: &str = "http://purl.org/dc/elements/1.1/";
pub const NS_XMP: &str = "http://ns.adobe.com/xap/1.0/";
pub const NS_IPTC_EXT: &str = "http://iptc.org/std/Iptc4xmpExt/2008-02-29/";
pub const NS_XMP_DM: &str = "http://ns.adobe.com/xmp/1.0/DynamicMedia/";

#[cfg(not(target_os = "ios"))]
pub mod xmpkit_body {
    use super::{NS_DC, NS_IPTC_EXT, NS_XMP, NS_XMP_DM};
    use xmpkit::core::metadata::XmpMeta;
    use xmpkit::files::file::XmpFile;
    use xmpkit::files::handler::XmpOptions;
    use xmpkit::types::value::XmpValue;

    fn load(bytes: &[u8]) -> (XmpFile, XmpMeta) {
        let mut xf = XmpFile::new();
        xf.from_bytes_with(bytes, XmpOptions::default().for_update())
            .expect("xmpkit: from_bytes_with failed");
        let meta = xf.get_xmp().cloned().unwrap_or_else(XmpMeta::new);
        (xf, meta)
    }

    pub fn embed(bytes: &[u8], prompt: Option<&str>, model: Option<&str>, subject: &[&str], project_name: Option<&str>, lyrics: Option<&str>, shot_number: Option<&str>) -> Vec<u8> {
        let (mut xf, mut meta) = load(bytes);
        if let Some(s) = project_name {
            meta.set_property(NS_XMP_DM, "projectName", XmpValue::from(s))
                .expect("xmpkit: set_property(xmpDM:projectName) failed");
        }
        if let Some(s) = lyrics {
            meta.set_property(NS_XMP_DM, "lyrics", XmpValue::from(s))
                .expect("xmpkit: set_property(xmpDM:lyrics) failed");
        }
        if let Some(s) = shot_number {
            meta.set_property(NS_XMP_DM, "shotNumber", XmpValue::from(s))
                .expect("xmpkit: set_property(xmpDM:shotNumber) failed");
        }
        if let Some(s) = prompt {
            meta.set_localized_text(NS_DC, "description", "", "x-default", s)
                .expect("xmpkit: set_localized_text(dc:description) failed");
            meta.set_property(NS_IPTC_EXT, "AIPromptInformation", XmpValue::from(s))
                .expect("xmpkit: set_property(Iptc4xmpExt:AIPromptInformation) failed");
        }
        if let Some(s) = model {
            meta.set_property(NS_XMP, "CreatorTool", XmpValue::from(s))
                .expect("xmpkit: set_property(xmp:CreatorTool) failed");
            meta.set_property(NS_IPTC_EXT, "AISystemUsed", XmpValue::from(s))
                .expect("xmpkit: set_property(Iptc4xmpExt:AISystemUsed) failed");
        }
        if !subject.is_empty() {
            let _ = meta.delete_property(NS_DC, "subject");
            for it in subject {
                meta.append_array_item(NS_DC, "subject", XmpValue::from(*it))
                    .expect("xmpkit: append_array_item(dc:subject) failed");
            }
        }
        xf.put_xmp(meta);
        xf.write_to_bytes().expect("xmpkit: write_to_bytes failed")
    }

    pub fn set_rating(bytes: &[u8], rating: i32) -> Vec<u8> {
        let (mut xf, mut meta) = load(bytes);
        meta.set_property(NS_XMP, "Rating", XmpValue::Integer(rating as i64))
            .expect("xmpkit: set_property(xmp:Rating) failed");
        xf.put_xmp(meta);
        xf.write_to_bytes().expect("xmpkit: write_to_bytes failed")
    }

    fn read_only(bytes: &[u8]) -> Option<XmpMeta> {
        let mut xf = XmpFile::new();
        xf.from_bytes(bytes).ok()?;
        Some(xf.get_xmp().cloned().unwrap_or_else(XmpMeta::new))
    }

    pub fn read_project_name(bytes: &[u8]) -> Option<String> {
        let m = read_only(bytes)?;
        m.get_property(NS_XMP_DM, "projectName").and_then(|v| v.as_str().map(|s| s.to_string()))
    }
    pub fn read_lyrics(bytes: &[u8]) -> Option<String> {
        let m = read_only(bytes)?;
        m.get_property(NS_XMP_DM, "lyrics").and_then(|v| v.as_str().map(|s| s.to_string()))
    }
    pub fn read_shot_number(bytes: &[u8]) -> Option<String> {
        let m = read_only(bytes)?;
        m.get_property(NS_XMP_DM, "shotNumber").and_then(|v| v.as_str().map(|s| s.to_string()))
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
            Some(v) => v.as_str().and_then(|s| s.parse().ok()).unwrap_or(-100),
            None => -100,
        }
    }
    pub fn read_property(bytes: &[u8], ns: &str, name: &str) -> Option<String> {
        let m = read_only(bytes)?;
        m.get_property(ns, name).and_then(|v| v.as_str().map(|s| s.to_string()))
    }
}
