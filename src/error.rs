#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    Generic(String),
    IoError(std::io::Error),
    ParseError(std::num::ParseIntError),
    ValueError(String),
}

impl From<&str> for Error {
    fn from(e: &str) -> Self {
        Error::Generic(e.to_string())
    }
}

impl From<String> for Error {
    fn from(e: String) -> Self {
        Error::Generic(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IoError(e)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Error::ParseError(e)
    }
}
