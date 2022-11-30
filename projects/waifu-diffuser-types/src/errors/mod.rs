use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

use serde::{Deserialize, Serialize};

// mod for_7z;
#[cfg(feature = "anyhow")]
mod for_anyhow;
mod for_image;
mod for_serde_json;

pub type DiffuserResult<T> = Result<T, DiffuserError>;

#[derive(Serialize, Deserialize)]
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

impl Error for DiffuserError {}

impl Debug for DiffuserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.kind, f)
    }
}

impl Display for DiffuserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.kind, f)
    }
}

impl Display for DiffuserErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DiffuserErrorKind::IOError { message, path } => {
                write!(f, "IOError: {} at {}", message, path)
            }
            DiffuserErrorKind::DecodeError { message } => {
                write!(f, "DecodeError: {}", message)
            }
            DiffuserErrorKind::CustomError { message } => {
                write!(f, "CustomError: {}", message)
            }
        }
    }
}

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
