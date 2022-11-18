use serde_json::Error;

use crate::DiffuserError;

impl From<Error> for DiffuserError {
    fn from(e: Error) -> Self {
        DiffuserError::decode_error(e.to_string())
    }
}
