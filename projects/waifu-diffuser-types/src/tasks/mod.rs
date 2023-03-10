use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::DiffuserScheduler;

pub use self::secrets::SecretKeeper;
pub use self::text2image::{Text2ImageReply, Text2ImageTask};
pub use self::unique::UniqueKey;

mod secrets;
mod unique;
mod short_action;
mod text2image;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DiffuserTask {
    Text2Image(Box<Text2ImageTask>),
    CollectLog(Box<CollectLogTask>),
}

pub enum DiffuserAnswer {
    Text2Image(Box<Text2ImageReply>),
    CollectLog(Box<CollectLogAnswer>),
}

pub struct CollectLogTask {
    pub id: u128,
    pub output: PathBuf,
}

pub struct CollectLogAnswer {}
