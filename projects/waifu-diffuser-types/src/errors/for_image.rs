use crate::DiffuserError;
use image::ImageError;
impl From<ImageError> for DiffuserError {
    fn from(value: ImageError) -> Self {
        todo!()
    }
}
