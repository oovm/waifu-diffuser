use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct CancelTask {
    pub id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryTask {
    pub id: Uuid,
}
