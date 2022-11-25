use serde::{Deserialize, Serialize};

// mod for_7z;
#[cfg(feature = "anyhow")]
mod for_anyhow;
mod for_image;
mod for_serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct DiffuserError {
    code: i32,
    kind: Box<DiffuserErrorKind>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DiffuserErrorKind {
    IOError { message: String, path: String },
    DecodeError { message: String },

    CustomError { message: String },
}

pub type DiffuserResult<T> = Result<T, DiffuserError>;

impl DiffuserError {
    pub fn decode_error<S>(message: S) -> Self
    where
        S: Into<String>,
    {
        let kind = DiffuserErrorKind::DecodeError { message: message.into() };
        Self { code: -10101, kind: Box::new(kind) }
    }

    pub fn custom_error<S>(message: S, code: i32) -> Self
    where
        S: Into<String>,
    {
        let kind = DiffuserErrorKind::CustomError { message: message.into() };
        Self { code, kind: Box::new(kind) }
    }
}
