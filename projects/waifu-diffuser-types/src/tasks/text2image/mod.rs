use crate::DiffuserScheduler;

use super::*;

mod der;

#[derive(Debug, Serialize)]
pub struct Text2ImageTask {
    /// UUID of the task, used for accept or cancel task
    pub id: String,
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

impl Default for Text2ImageTask {
    fn default() -> Self {
        Self {
            id: Default::default(),
            positive: "masterpiece, best quality".to_string(),
            negative: "".to_string(),
            width: 256,
            height: 256,
            step: 20,
            scheduler: Default::default(),
        }
    }
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
