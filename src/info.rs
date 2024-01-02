use crate::util::variables;
use serde::Serialize;

pub static VOSO_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Serialize)]
pub struct Info {
    voso: &'static str,
    features: Features,
    ws: &'static str,
}

#[derive(Serialize)]
pub struct Features {
    rtp: bool,
}

pub fn get_info() -> Info {
    let features = Features {
        rtp: !*variables::DISABLE_RTP,
    };

    Info {
        voso: VOSO_VERSION,
        features,
        ws: &variables::WS_URL,
    }
}
