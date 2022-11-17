use super::*;
use crate::DiffuserScheduler;

#[derive(Debug, Serialize, Deserialize)]
pub struct Text2ImageTask {
    /// UUID of the task, used for accept or cancel task
    pub id: Uuid,
    /// index of the image in the batch
    pub positive: String,
    /// index of the image in the batch
    pub negative: String,
    /// index of the image in the batch
    pub width: u32,
    /// index of the image in the batch
    pub height: u32,
    /// index of the image in the batch
    pub step: u32,
    /// index of the image in the batch
    pub scheduler: DiffuserScheduler,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Text2ImageReply {
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
