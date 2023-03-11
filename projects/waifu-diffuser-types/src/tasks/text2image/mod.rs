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
    pub batch: u8,
    /// index of the image in the batch
    pub start_id: usize,
    /// index of the image in the batch
    pub step: usize,
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
            batch: 1,
            start_id: 0,
            step: 20,
            scheduler: Default::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Text2ImageReply {
    /// UUID of the task
    pub id: String,
    /// Step of this diffusion
    pub step: usize,
    /// index of the image in the batch
    pub index: usize,
    /// width of the image
    pub width: u32,
    /// height of the image
    pub height: u32,
    /// png image
    pub png: Vec<u8>,
}

impl Text2ImageReply {
    #[allow(clippy::wrong_self_convention)]
    pub fn as_response(self) -> DiffuserResponse {
        DiffuserResponse { code: 200, kind: DiffuserReply::Text2Image(Box::new(self)) }
    }
}
