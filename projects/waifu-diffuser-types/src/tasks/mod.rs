use std::path::PathBuf;

pub struct DiffuserTask {
    pub id: u128,
    pub prompt: String,
    pub output: PathBuf,
}

pub mod secrets;
pub mod unique;
