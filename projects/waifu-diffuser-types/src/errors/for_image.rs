use image::ImageError;
use url::ParseError;

use crate::DiffuserError;

impl From<ImageError> for DiffuserError {
    fn from(value: ImageError) -> Self {
        match value {
            ImageError::Decoding(e) => DiffuserError::decode_error(e.to_string()),
            ImageError::Encoding(_) => {
                unimplemented!()
            }
            ImageError::Parameter(_) => {
                unimplemented!()
            }
            ImageError::Limits(_) => {
                unimplemented!()
            }
            ImageError::Unsupported(_) => {
                unimplemented!()
            }
            ImageError::IoError(_) => {
                unimplemented!()
            }
        }
    }
}

impl From<ParseError> for DiffuserError {
    fn from(value: ParseError) -> Self {
        DiffuserError::decode_error(value.to_string())
    }
}
