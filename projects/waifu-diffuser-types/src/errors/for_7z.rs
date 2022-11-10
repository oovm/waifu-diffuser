use crate::DiffuserError;

impl From<sevenz_rust::Error> for DiffuserError {
    fn from(e: sevenz_rust::Error) -> Self {
        DiffuserError::SevenZ(e)
    }
}
