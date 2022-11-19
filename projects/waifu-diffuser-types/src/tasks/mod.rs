use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use self::{
    collect_log::{CollectLogReply, CollectLogTask},
    secrets::SecretKeeper,
    text2image::{Text2ImageReply, Text2ImageTask},
    unique::UniqueKey,
};

mod collect_log;
mod secrets;
mod short_action;
mod text2image;
mod unique;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DiffuserTask {
    Text2Image(Box<Text2ImageTask>),
    CollectLog(Box<CollectLogTask>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiffuserResponse {
    pub code: i32,
    pub kind: DiffuserReply,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DiffuserReply {
    Text2Image(Box<Text2ImageReply>),
    CollectLog(Box<CollectLogReply>),
}
