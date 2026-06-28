use std::sync::OnceLock;
use std::time::Duration;

mod flux2_dev_i2i;
mod flux2_klein_i2i;
mod flux2_pro;
mod ltx2_3a2v;
mod nano_banana2;
mod qwen3_6_35b_a3b;
mod qwen3_asr_flash;
mod z_image_turbo;

pub(crate) const URL: &str = "https://femi.market/api";

static RT: OnceLock<tokio::runtime::Runtime> = OnceLock::new();
pub(crate) fn rt() -> &'static tokio::runtime::Runtime {
    RT.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .expect("tokio runtime")
    })
}

static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();
pub(crate) fn client() -> &'static reqwest::Client {
    CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(600))
            .build()
            .expect("reqwest client")
    })
}
