use crate::DiffuserError;

impl From<tungstenite::Error> for DiffuserError {
    fn from(value: tungstenite::Error) -> Self {
        DiffuserError::custom_error(value.to_string(), -10002)
    }
}
