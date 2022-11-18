use serde::{Deserialize, Serialize};

// mod for_7z;
mod for_serde_json;

#[derive(Debug, Serialize)]
pub struct DiffuserError {
    code: i16,
    #[serde(flatten)]
    kind: Box<DiffuserErrorKind>,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum DiffuserErrorKind {
    IOError { message: String, path: String },
    DecodeError { message: String },
}

pub type DiffuserResult<T> = Result<T, DiffuserError>;

impl DiffuserError {
    pub fn decode_error<S>(message: S) -> Self
    where
        S: Into<String>,
    {
        let kind = DiffuserErrorKind::DecodeError { message: message.into() };
        Self { code: -12000, kind: Box::new(kind) }
    }
}
