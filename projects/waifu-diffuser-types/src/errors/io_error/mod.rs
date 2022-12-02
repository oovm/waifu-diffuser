use std::io::Error;

use crate::{DiffuserError, DiffuserErrorKind};

impl From<std::io::Error> for DiffuserError {
    fn from(value: Error) -> Self {
        DiffuserError {
            code: -10001,
            kind: Box::new(DiffuserErrorKind::IOError { message: value.to_string(), path: String::new() }),
        }
    }
}
