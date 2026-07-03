//! Host-runnable integration tests calling the REAL production
//! `rust_ffi::project_service::shared::xmpkit_body::*` — the exact shared
//! bytes-in/bytes-out XMP logic that Android JNI and Wasm-bindgen wrap.
//!
//! Ports 22 of 39 `Tests/ApiTests/ProjectServiceTests.swift` cases: everything
//! that reduces to bytes-in/bytes-out. The remaining 17 (fs I/O, in-memory
//! statics, path handling) live in `android.rs` / `wasm.rs` and need Android
//! emulator / browser tests.

use std::io::BufWriter;

use rust_ffi::project_service::shared::xmpkit_body;

const NS_IPTC_EXT: &str = "http://iptc.org/std/Iptc4xmpExt/2008-02-29/";

const CAR_MP4: &[u8] = include_bytes!("../../Assets/car.mp4");

fn make_png() -> Vec<u8> {
    let mut out = Vec::new();
    {
        let w = BufWriter::new(&mut out);
        let mut enc = png::Encoder::new(w, 1, 1);
        enc.set_color(png::ColorType::Rgba);
        enc.set_depth(png::BitDepth::Eight);
        let mut writer = enc.write_header().unwrap();
        writer.write_image_data(&[255, 255, 255, 255]).unwrap();
    }
    out
}

fn get_like(bytes: &[u8]) -> bool {
    (1..=5).contains(&xmpkit_body::read_rating(bytes))
}

fn subject(bytes: &[u8]) -> Vec<String> {
    let n = xmpkit_body::read_subject_count(bytes);
    if n <= 0 { return Vec::new(); }
    (0..n).filter_map(|i| xmpkit_body::read_subject_at(bytes, i)).collect()
}

// ===================== PNG: prompt / model =====================

#[test] fn save_file_embeds_prompt() {
    let out = xmpkit_body::embed(&make_png(), Some("hello world"), None, &[]);
    assert_eq!(xmpkit_body::read_prompt(&out).as_deref(), Some("hello world"));
}

#[test] fn save_file_embeds_model() {
    let out = xmpkit_body::embed(&make_png(), None, Some("dalle-3"), &[]);
    assert_eq!(xmpkit_body::read_model(&out).as_deref(), Some("dalle-3"));
}

#[test] fn save_file_embeds_both() {
    let out = xmpkit_body::embed(&make_png(), Some("p"), Some("m"), &[]);
    assert_eq!(xmpkit_body::read_prompt(&out).as_deref(), Some("p"));
    assert_eq!(xmpkit_body::read_model(&out).as_deref(), Some("m"));
}

#[test] fn save_file_writes_iptc_ext_ai_prompt_information() {
    let out = xmpkit_body::embed(&make_png(), Some("what is AI"), None, &[]);
    assert_eq!(xmpkit_body::read_property(&out, NS_IPTC_EXT, "AIPromptInformation").as_deref(), Some("what is AI"));
}

#[test] fn save_file_writes_iptc_ext_ai_system_used() {
    let out = xmpkit_body::embed(&make_png(), None, Some("dalle-3"), &[]);
    assert_eq!(xmpkit_body::read_property(&out, NS_IPTC_EXT, "AISystemUsed").as_deref(), Some("dalle-3"));
}

// ===================== PNG: subject =====================

#[test] fn save_file_embeds_subject() {
    let out = xmpkit_body::embed(&make_png(), None, None, &["cat", "fluffy", "studio"]);
    assert_eq!(subject(&out), vec!["cat", "fluffy", "studio"]);
}

#[test] fn save_file_embeds_all_three() {
    let out = xmpkit_body::embed(&make_png(), Some("p"), Some("m"), &["a", "b"]);
    assert_eq!(xmpkit_body::read_prompt(&out).as_deref(), Some("p"));
    assert_eq!(xmpkit_body::read_model(&out).as_deref(), Some("m"));
    assert_eq!(subject(&out), vec!["a", "b"]);
}

#[test] fn get_subject_nil_when_absent() {
    let out = xmpkit_body::embed(&make_png(), None, None, &[]);
    assert!(subject(&out).is_empty());
}

#[test] fn get_subject_nil_when_empty_array_passed() {
    let out = xmpkit_body::embed(&make_png(), None, None, &[]);
    assert!(subject(&out).is_empty());
}

// ===================== MP4 =====================

#[test] fn save_file_embeds_prompt_in_video() {
    let out = xmpkit_body::embed(CAR_MP4, Some("a video of a fox"), None, &[]);
    assert_eq!(xmpkit_body::read_prompt(&out).as_deref(), Some("a video of a fox"));
}

#[test] fn save_file_embeds_model_in_video() {
    let out = xmpkit_body::embed(CAR_MP4, None, Some("sora-1"), &[]);
    assert_eq!(xmpkit_body::read_model(&out).as_deref(), Some("sora-1"));
}

#[test] fn save_file_embeds_subject_in_video() {
    let out = xmpkit_body::embed(CAR_MP4, None, None, &["fox", "wildlife"]);
    assert_eq!(subject(&out), vec!["fox", "wildlife"]);
}

#[test] fn save_file_embeds_all_three_in_video() {
    let out = xmpkit_body::embed(CAR_MP4, Some("p"), Some("m"), &["a", "b"]);
    assert_eq!(xmpkit_body::read_prompt(&out).as_deref(), Some("p"));
    assert_eq!(xmpkit_body::read_model(&out).as_deref(), Some("m"));
    assert_eq!(subject(&out), vec!["a", "b"]);
}

#[test] fn save_file_writes_iptc_ext_ai_prompt_information_in_video() {
    let out = xmpkit_body::embed(CAR_MP4, Some("what is AI video"), None, &[]);
    assert_eq!(xmpkit_body::read_property(&out, NS_IPTC_EXT, "AIPromptInformation").as_deref(), Some("what is AI video"));
}

#[test] fn save_file_writes_iptc_ext_ai_system_used_in_video() {
    let out = xmpkit_body::embed(CAR_MP4, None, Some("sora-1"), &[]);
    assert_eq!(xmpkit_body::read_property(&out, NS_IPTC_EXT, "AISystemUsed").as_deref(), Some("sora-1"));
}

// ===================== overwrite / absent =====================

#[test] fn save_file_overwrites_existing() {
    let first = xmpkit_body::embed(&make_png(), Some("first"), None, &[]);
    let second = xmpkit_body::embed(&first, Some("second"), None, &[]);
    assert_eq!(xmpkit_body::read_prompt(&second).as_deref(), Some("second"));
}

#[test] fn get_prompt_nil_when_absent() {
    let out = xmpkit_body::embed(&make_png(), None, None, &[]);
    assert!(xmpkit_body::read_prompt(&out).is_none());
}

#[test] fn get_model_nil_when_absent() {
    let out = xmpkit_body::embed(&make_png(), None, None, &[]);
    assert!(xmpkit_body::read_model(&out).is_none());
}

// ===================== rating (Swift "like") =====================

#[test] fn like_true_then_read() {
    let out = xmpkit_body::set_rating(&make_png(), 5);
    assert!(get_like(&out));
}

#[test] fn like_false_after_true() {
    let out = xmpkit_body::set_rating(&make_png(), 5);
    let out = xmpkit_body::set_rating(&out, 0);
    assert!(!get_like(&out));
}

#[test] fn get_like_false_when_absent() {
    let out = xmpkit_body::embed(&make_png(), None, None, &[]);
    assert!(!get_like(&out));
}
