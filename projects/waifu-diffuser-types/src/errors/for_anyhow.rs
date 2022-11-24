use anyhow::Error;

use crate::DiffuserError;

impl From<Error> for DiffuserError {
    fn from(value: Error) -> Self {
        DiffuserError::custom_error(value.to_string(), -9999)
    }
}
