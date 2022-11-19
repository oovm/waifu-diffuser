use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectLogTask {
    pub id: u128,
    pub output: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectLogReply {}
