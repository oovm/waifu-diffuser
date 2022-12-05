use super::*;

mod der;

#[derive(Clone, Debug, Serialize)]
pub struct Text2ImageTask {
    /// positive prompts of the image
    pub positive: String,
    /// negative prompts of the image
    pub negative: String,
    /// preferred width of the image, recommended to be times of 32
    pub width: u32,
    /// preferred height of the image, recommended to be times of 32
    pub height: u32,
    /// batch size of the request
    pub batch: u8,
    /// start index of the batch, used for reducer
    pub start_id: usize,
    /// number of steps to run
    pub steps: usize,
    /// which scheduler to use
    pub scheduler: DiffuserScheduler,
}

impl Default for Text2ImageTask {
    fn default() -> Self {
        Self {
            positive: "masterpiece, best quality".to_string(),
            negative: "".to_string(),
            width: 256,
            height: 256,
            batch: 1,
            start_id: 0,
            steps: 20,
            scheduler: Default::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Text2ImageReply {
    /// UUID of the task, used to identify the task
    pub task_id: Uuid,
    /// Step of this diffusion
    pub step: usize,
    /// index of the image in the batch
    pub index: usize,
    /// width of the image
    pub width: u32,
    /// height of the image
    pub height: u32,
    /// png image bytes
    pub png: Vec<u8>,
}

impl Text2ImageTask {
    pub fn as_reply(&self, task_id: Uuid, step: usize, index: usize, png: Vec<u8>) -> DiffuserResponse {
        Text2ImageReply { task_id, step, index: self.start_id + index, width: self.width, height: self.height, png }
            .as_response()
    }
    pub fn with_prompts<P, N>(mut self, positive: P, negative: N) -> Self
    where
        P: Into<String>,
        N: Into<String>,
    {
        self.positive = positive.into();
        self.negative = negative.into();
        self
    }
}

impl Text2ImageReply {
    #[allow(clippy::wrong_self_convention)]
    pub fn as_response(self) -> DiffuserResponse {
        DiffuserResponse { code: 200, kind: DiffuserReply::Text2Image(Box::new(self)) }
    }
}
