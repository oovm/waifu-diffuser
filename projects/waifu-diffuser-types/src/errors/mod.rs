mod for_7z;

#[derive(Debug, Copy, Clone)]
pub struct DiffuserError {
    kind: DiffuserErrorKind,
}

pub enum DiffuserErrorKind {
    IOError(std::io::Error),
}

pub struct IOError {}

pub type DiffuserResult<T> = Result<T, DiffuserError>;
