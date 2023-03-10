use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::DiffuserScheduler;

pub use self::secrets::SecretKeeper;
pub use self::unique::UniqueKey;

mod secrets;
mod unique;
mod short_action;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DiffuserTask {
    Text2Image(Box<Text2ImageTask>),
    CollectLog(Box<CollectLogTask>),
}

pub enum DiffuserAnswer {
    Text2Image(Box<Text2ImageAnswer>),
    CollectLog(Box<CollectLogAnswer>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Text2ImageTask {
    /// UUID of the task, used for accept or cancel task
    pub id: Uuid,
    pub positive: String,
    pub negative: String,
    pub width: u32,
    pub height: u32,
    pub step: u32,
    pub scheduler: DiffuserScheduler,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Text2ImageAnswer {
    /// UUID of the task
    pub id: Uuid,
    /// index of the image in the batch
    pub index: u8,
    /// Step of this diffusion
    pub step: u32,
    /// width of the image
    pub width: u32,
    /// height of the image
    pub height: u32,
    /// png image
    pub png: Vec<u8>,
}


pub struct CollectLogTask {
    pub id: u128,
    pub output: PathBuf,
}

pub struct CollectLogAnswer {}
