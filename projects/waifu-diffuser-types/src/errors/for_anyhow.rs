use crate::DiffuserError;

impl From<anyhow::Error> for DiffuserError {
    fn from(value: anyhow::Error) -> Self {
        todo!()
    }
}
