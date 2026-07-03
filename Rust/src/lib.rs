use std::sync::OnceLock;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Duration;

mod api;
mod id3;
pub mod project_service;

pub(crate) const URL: &str = "https://femi.market/api";

pub(crate) const FALLBACK_IMAGE: &[u8] = include_bytes!("../../Assets/fallback.png");
pub(crate) const TOPUP_IMAGE: &[u8] = include_bytes!("../../Assets/topup.jpg");
pub(crate) const FALLBACK_VIDEO: &[u8] = include_bytes!("../../Assets/could-not-generate.mp4");
pub(crate) const TOPUP_VIDEO: &[u8] = include_bytes!("../../Assets/topup-video.mp4");

/// On 200 decode `action.file` base64 → real image bytes; on 402 → embedded
/// topup image; anything else → embedded fallback image.
pub(crate) fn resolve_image(status: u16, body: &[u8]) -> Vec<u8> {
    if status == 200 {
        if let Ok(json) = serde_json::from_slice::<serde_json::Value>(body) {
            if let Some(file) = json.pointer("/action/file").and_then(|v| v.as_str()) {
                if !file.is_empty() {
                    use base64::Engine;
                    if let Ok(data) = base64::engine::general_purpose::STANDARD.decode(file) {
                        if !data.is_empty() { return data; }
                    }
                }
            }
        }
    }
    if status == 402 { TOPUP_IMAGE.to_vec() } else { FALLBACK_IMAGE.to_vec() }
}

/// Same as resolve_image but for the video assets.
pub(crate) fn resolve_video(status: u16, body: &[u8]) -> Vec<u8> {
    if status == 200 {
        if let Ok(json) = serde_json::from_slice::<serde_json::Value>(body) {
            if let Some(file) = json.pointer("/action/file").and_then(|v| v.as_str()) {
                if !file.is_empty() {
                    use base64::Engine;
                    if let Ok(data) = base64::engine::general_purpose::STANDARD.decode(file) {
                        if !data.is_empty() { return data; }
                    }
                }
            }
        }
    }
    if status == 402 { TOPUP_VIDEO.to_vec() } else { FALLBACK_VIDEO.to_vec() }
}

#[cfg(not(target_arch = "wasm32"))]
static RT: OnceLock<tokio::runtime::Runtime> = OnceLock::new();
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn rt() -> &'static tokio::runtime::Runtime {
    RT.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("tokio runtime")
    })
}

static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();
pub(crate) fn client() -> &'static reqwest::Client {
    CLIENT.get_or_init(|| {
        let b = reqwest::Client::builder();
        #[cfg(not(target_arch = "wasm32"))]
        let b = b
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(600));
        b.build().expect("reqwest client")
    })
}
