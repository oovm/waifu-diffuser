use std::path::PathBuf;

pub struct DiffuserTask {
    pub prompt: String,
    pub output: PathBuf,
}

pub mod secrets;
pub mod unique;