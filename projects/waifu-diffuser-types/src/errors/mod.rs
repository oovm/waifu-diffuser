use serde::{Deserialize, Serialize};

// mod for_7z;
mod for_serde_json;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct DiffuserError {
    kind: Box<DiffuserErrorKind>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DiffuserErrorKind {
    IOError(std::io::Error),
    DecodeError { message: String },
}

pub type DiffuserResult<T> = Result<T, DiffuserError>;

impl DiffuserError {
    pub fn decode_error<S>(message: S) -> Self
    where
        S: Into<String>,
    {
        let kind = DiffuserErrorKind::DecodeError { message: message.into() };
        Self { kind: Box::new(kind) }
    }
}
