use std::path::PathBuf;

use package_key::InsensitiveKey;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use self::{
    collect_log::{CollectLogReply, CollectLogTask},
    secrets::SecretKeeper,
    text2image::{Text2ImageReply, Text2ImageTask},
};

mod collect_log;
mod queue;
mod secrets;
mod short_action;
mod text2image;

#[derive(Debug, Serialize, Deserialize)]
pub struct DiffuserTask {
    pub user_id: Uuid,
    pub task_id: Uuid,
    pub body: DiffuserTaskKind,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DiffuserTaskKind {
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
