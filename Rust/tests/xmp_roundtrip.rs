//! Round-trip tests for the XMP FFI on the host backend.
//!
//! Ports every Swift ProjectServiceTests case whose behavior is exercised
//! through the Rust FFI (XMP embed/read/rating). Swift-only cases (Documents
//! dir, character cast RAM, image edit RAM, audio housekeeping, URL path
//! traversal) don't apply here — the Rust FFI is stateless and only mutates
//! whatever file path the caller passes.

#![cfg(not(target_arch = "wasm32"))]

use std::ffi::{c_char, CString};
use std::fs::File;
use std::io::BufWriter;

use project_service_ffi::{
    psxmp_embed, psxmp_read_model, psxmp_read_prompt, psxmp_read_rating,
    psxmp_read_subject_at, psxmp_read_subject_count, psxmp_set_rating,
};
use tempfile::TempDir;
use xmp_toolkit::{OpenFileOptions, XmpFile};

const NS_IPTC_EXT: &str = "http://iptc.org/std/Iptc4xmpExt/2008-02-29/";

/// Test-only probe: read an arbitrary XMP property directly off disk, so tests
/// can assert the on-disk namespace/name rather than round-tripping through
/// the FFI's own reader (which would tautologically pass).
fn read_property_on_disk(path: &std::path::Path, ns: &str, name: &str) -> Option<String> {
    let mut f = XmpFile::new().ok()?;
    f.open_file(path, OpenFileOptions::default().for_read().use_smart_handler()).ok()?;
    let m = f.xmp()?;
    m.property(ns, name).map(|v| v.value)
}

fn write_1x1_png(path: &std::path::Path) {
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, 1, 1);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&[255, 255, 255, 255]).unwrap();
}

fn cs(s: &str) -> CString {
    CString::new(s).unwrap()
}

fn read_str_via<F>(path: &CString, f: F) -> Option<String>
where
    F: FnOnce(*const c_char, *mut c_char, i32) -> i32,
{
    let mut buf = vec![0i8; 8192];
    let n = f(path.as_ptr(), buf.as_mut_ptr(), buf.len() as i32);
    if n <= 0 {
        return None;
    }
    let bytes: Vec<u8> = buf[..n as usize].iter().map(|b| *b as u8).collect();
    Some(String::from_utf8(bytes).unwrap())
}

/// Make a fresh 1x1 PNG in a new temp dir and return (tempdir, cstr-path).
fn fresh_png() -> (TempDir, CString) {
    let dir = TempDir::new().unwrap();
    let png = dir.path().join("t.png");
    write_1x1_png(&png);
    let cpath = cs(png.to_str().unwrap());
    (dir, cpath)
}

fn embed_all(path: &CString, prompt: Option<&str>, model: Option<&str>, subject: &[&str]) -> i32 {
    let pr = prompt.map(cs);
    let md = model.map(cs);
    let subs: Vec<CString> = subject.iter().map(|s| cs(s)).collect();
    let ptrs: Vec<*const c_char> = subs.iter().map(|c| c.as_ptr()).collect();
    unsafe {
        psxmp_embed(
            path.as_ptr(),
            pr.as_ref().map(|c| c.as_ptr()).unwrap_or(std::ptr::null()),
            md.as_ref().map(|c| c.as_ptr()).unwrap_or(std::ptr::null()),
            if ptrs.is_empty() { std::ptr::null() } else { ptrs.as_ptr() },
            ptrs.len() as i32,
        )
    }
}

// ---- prompt / model / iptcext ----

#[test]
fn embed_embeds_prompt() {
    let (_d, p) = fresh_png();
    assert_eq!(embed_all(&p, Some("hello"), None, &[]), 0);
    assert_eq!(
        read_str_via(&p, |p, b, n| unsafe { psxmp_read_prompt(p, b, n) }).as_deref(),
        Some("hello")
    );
}

#[test]
fn embed_embeds_model() {
    let (_d, p) = fresh_png();
    assert_eq!(embed_all(&p, None, Some("dalle-3"), &[]), 0);
    assert_eq!(
        read_str_via(&p, |p, b, n| unsafe { psxmp_read_model(p, b, n) }).as_deref(),
        Some("dalle-3")
    );
}

#[test]
fn embed_embeds_both() {
    let (_d, p) = fresh_png();
    assert_eq!(embed_all(&p, Some("hi"), Some("m"), &[]), 0);
    assert_eq!(
        read_str_via(&p, |p, b, n| unsafe { psxmp_read_prompt(p, b, n) }).as_deref(),
        Some("hi")
    );
    assert_eq!(
        read_str_via(&p, |p, b, n| unsafe { psxmp_read_model(p, b, n) }).as_deref(),
        Some("m")
    );
}

#[test]
fn iptcext_ai_prompt_information_is_on_disk() {
    let (d, p) = fresh_png();
    assert_eq!(embed_all(&p, Some("hello"), None, &[]), 0);
    let png = d.path().join("t.png");
    assert_eq!(
        read_property_on_disk(&png, NS_IPTC_EXT, "AIPromptInformation").as_deref(),
        Some("hello")
    );
}

#[test]
fn iptcext_ai_system_used_is_on_disk() {
    let (d, p) = fresh_png();
    assert_eq!(embed_all(&p, None, Some("dalle-3"), &[]), 0);
    let png = d.path().join("t.png");
    assert_eq!(
        read_property_on_disk(&png, NS_IPTC_EXT, "AISystemUsed").as_deref(),
        Some("dalle-3")
    );
}

// ---- subject ----

#[test]
fn embed_embeds_subject() {
    let (_d, p) = fresh_png();
    assert_eq!(embed_all(&p, None, None, &["fox", "wildlife"]), 0);
    let count = unsafe { psxmp_read_subject_count(p.as_ptr()) };
    assert_eq!(count, 2);
    assert_eq!(
        read_str_via(&p, |p, b, n| unsafe { psxmp_read_subject_at(p, 0, b, n) }).as_deref(),
        Some("fox")
    );
    assert_eq!(
        read_str_via(&p, |p, b, n| unsafe { psxmp_read_subject_at(p, 1, b, n) }).as_deref(),
        Some("wildlife")
    );
}

#[test]
fn embed_embeds_all_three() {
    let (_d, p) = fresh_png();
    assert_eq!(embed_all(&p, Some("p"), Some("m"), &["a", "b"]), 0);
    assert_eq!(
        read_str_via(&p, |p, b, n| unsafe { psxmp_read_prompt(p, b, n) }).as_deref(),
        Some("p")
    );
    assert_eq!(
        read_str_via(&p, |p, b, n| unsafe { psxmp_read_model(p, b, n) }).as_deref(),
        Some("m")
    );
    assert_eq!(unsafe { psxmp_read_subject_count(p.as_ptr()) }, 2);
}

#[test]
fn subject_absent_returns_zero() {
    let (_d, p) = fresh_png();
    assert_eq!(unsafe { psxmp_read_subject_count(p.as_ptr()) }, 0);
}

#[test]
fn embed_with_empty_subject_array_writes_nothing() {
    let (_d, p) = fresh_png();
    assert_eq!(embed_all(&p, None, None, &[]), 0);
    assert_eq!(unsafe { psxmp_read_subject_count(p.as_ptr()) }, 0);
}

// ---- absent ----

#[test]
fn absent_prompt_returns_zero() {
    let (_d, p) = fresh_png();
    let mut buf = vec![0i8; 128];
    let n = unsafe { psxmp_read_prompt(p.as_ptr(), buf.as_mut_ptr(), buf.len() as i32) };
    assert_eq!(n, 0);
}

#[test]
fn absent_model_returns_zero() {
    let (_d, p) = fresh_png();
    let mut buf = vec![0i8; 128];
    let n = unsafe { psxmp_read_model(p.as_ptr(), buf.as_mut_ptr(), buf.len() as i32) };
    assert_eq!(n, 0);
}

// ---- rating (Swift "like") ----

#[test]
fn rating_5_roundtrips() {
    let (_d, p) = fresh_png();
    assert_eq!(unsafe { psxmp_set_rating(p.as_ptr(), 5) }, 0);
    assert_eq!(unsafe { psxmp_read_rating(p.as_ptr()) }, 5);
}

#[test]
fn rating_0_after_5() {
    let (_d, p) = fresh_png();
    assert_eq!(unsafe { psxmp_set_rating(p.as_ptr(), 5) }, 0);
    assert_eq!(unsafe { psxmp_set_rating(p.as_ptr(), 0) }, 0);
    assert_eq!(unsafe { psxmp_read_rating(p.as_ptr()) }, 0);
}

#[test]
fn absent_rating_returns_sentinel() {
    let (_d, p) = fresh_png();
    // -100 = "absent" sentinel from the FFI (see psxmp_read_rating).
    assert_eq!(unsafe { psxmp_read_rating(p.as_ptr()) }, -100);
}
