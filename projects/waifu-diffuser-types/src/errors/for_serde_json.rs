use std::fmt::Display;

use serde_json::Error;

use crate::DiffuserError;

impl From<Error> for DiffuserError {
    fn from(e: Error) -> Self {
        DiffuserError::decode_error(e.to_string())
    }
}

impl serde::ser::Error for DiffuserError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        DiffuserError::custom_error(msg.to_string(), -10102)
    }
}

impl serde::de::Error for DiffuserError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        DiffuserError::decode_error(msg.to_string())
    }
}
