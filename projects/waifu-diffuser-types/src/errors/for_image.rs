use crate::DiffuserError;
use image::ImageError;
use url::{ParseError, Url};

impl From<ImageError> for DiffuserError {
    fn from(value: ImageError) -> Self {
        todo!()
    }
}

impl From<ParseError> for DiffuserError {
    fn from(value: ParseError) -> Self {
        DiffuserError::decode_error(value.to_string())
    }
}
