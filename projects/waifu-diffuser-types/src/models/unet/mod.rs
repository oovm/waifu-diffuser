use super::*;
use semver::Version;
use url::Url;

// mod der;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UNetModel {
    version: Version,
    net_path: String,
    net_url: Url,
    vae_encoder_path: String,
    vae_encoder_url: Url,
    vae_decoder_path: String,
    vae_decoder_url: Url,
}
