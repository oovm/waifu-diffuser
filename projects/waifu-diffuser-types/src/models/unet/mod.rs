use super::*;
use crate::ResourcePath;
use semver::Version;
use url::Url;

// mod der;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UNetModel {
    version: Version,
    net: ResourcePath,
    vae_encoder: Option<ResourcePath>,
    vae_decoder: Option<ResourcePath>,
}
