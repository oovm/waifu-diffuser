use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use self::{
    secrets::SecretKeeper,
    text2image::{Text2ImageReply, Text2ImageTask},
    unique::UniqueKey,
};

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

pub enum DiffuserAnswer {
    Text2Image(Box<Text2ImageReply>),
    CollectLog(Box<CollectLogAnswer>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectLogTask {
    pub id: u128,
    pub output: PathBuf,
}

pub struct CollectLogAnswer {}
