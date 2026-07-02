//! Apple backend — direct `xmp_toolkit` path API (full smart-handler set:
//! MP4/MOV, PNG, JPEG, TIFF, etc.).

use super::shared::{NS_DC, NS_IPTC_EXT, NS_XMP};
use xmp_toolkit::{OpenFileOptions, XmpFile, XmpMeta, XmpValue};

fn open_for_update(path: &str) -> Result<XmpFile, i32> {
    let mut f = XmpFile::new().map_err(|_| -1)?;
    f.open_file(path, OpenFileOptions::default().for_update().use_smart_handler())
        .map_err(|_| -1)?;
    Ok(f)
}
fn open_for_read(path: &str) -> Result<XmpFile, i32> {
    let mut f = XmpFile::new().map_err(|_| -1)?;
    f.open_file(path, OpenFileOptions::default().for_read().use_smart_handler())
        .map_err(|_| -1)?;
    Ok(f)
}
fn load_meta(f: &mut XmpFile) -> XmpMeta {
    f.xmp().unwrap_or_else(|| XmpMeta::new().unwrap())
}

pub fn embed(path: &str, prompt: Option<&str>, model: Option<&str>, subject: &[&str]) -> i32 {
    let mut file = match open_for_update(path) { Ok(f) => f, Err(e) => return e };
    let mut meta = load_meta(&mut file);
    if let Some(s) = prompt {
        if meta.set_localized_text(NS_DC, "description", None, "x-default", s).is_err() { return -1; }
        if meta.set_property(NS_IPTC_EXT, "AIPromptInformation", &XmpValue::new(s.to_owned())).is_err() { return -1; }
    }
    if let Some(s) = model {
        if meta.set_property(NS_XMP, "CreatorTool", &XmpValue::new(s.to_owned())).is_err() { return -1; }
        if meta.set_property(NS_IPTC_EXT, "AISystemUsed", &XmpValue::new(s.to_owned())).is_err() { return -1; }
    }
    if !subject.is_empty() {
        let _ = meta.delete_property(NS_DC, "subject");
        let arr = XmpValue::new("subject".to_owned()).set_is_array(true);
        for it in subject {
            if meta.append_array_item(NS_DC, &arr, &XmpValue::new((*it).to_owned())).is_err() { return -1; }
        }
    }
    if !file.can_put_xmp(&meta) { return -1; }
    if file.put_xmp(&meta).is_err() { return -1; }
    file.close();
    0
}

pub fn read_prompt(path: &str) -> Option<String> {
    let mut f = open_for_read(path).ok()?;
    let m = load_meta(&mut f);
    if let Some(v) = m.property(NS_IPTC_EXT, "AIPromptInformation") { return Some(v.value); }
    m.localized_text(NS_DC, "description", None, "x-default").map(|(v, _)| v.value)
}
pub fn read_model(path: &str) -> Option<String> {
    let mut f = open_for_read(path).ok()?;
    let m = load_meta(&mut f);
    if let Some(v) = m.property(NS_IPTC_EXT, "AISystemUsed") { return Some(v.value); }
    m.property(NS_XMP, "CreatorTool").map(|v| v.value)
}
pub fn read_subject_count(path: &str) -> i32 {
    let Ok(mut f) = open_for_read(path) else { return -1 };
    let m = load_meta(&mut f);
    m.array_len(NS_DC, "subject") as i32
}
pub fn read_subject_at(path: &str, index: i32) -> Option<String> {
    let mut f = open_for_read(path).ok()?;
    let m = load_meta(&mut f);
    m.array_item(NS_DC, "subject", index + 1).map(|v| v.value)
}
pub fn set_rating(path: &str, rating: i32) -> i32 {
    let mut file = match open_for_update(path) { Ok(f) => f, Err(e) => return e };
    let mut meta = load_meta(&mut file);
    if meta.set_property_i32(NS_XMP, "Rating", &XmpValue::new(rating)).is_err() { return -1; }
    if !file.can_put_xmp(&meta) { return -1; }
    if file.put_xmp(&meta).is_err() { return -1; }
    file.close();
    0
}
pub fn read_rating(path: &str) -> i32 {
    let Ok(mut f) = open_for_read(path) else { return -1 };
    let m = load_meta(&mut f);
    match m.property_i32(NS_XMP, "Rating") { Some(v) => v.value, None => -100 }
}
pub fn read_property(path: &str, ns: &str, name: &str) -> Option<String> {
    let mut f = open_for_read(path).ok()?;
    let m = load_meta(&mut f);
    m.property(ns, name).map(|v| v.value)
}

