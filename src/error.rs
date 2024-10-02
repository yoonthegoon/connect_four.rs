#[derive(Debug)]
pub enum Error {
    Generic(Box<str>),
    IoError(std::io::Error),
    ParseError(std::num::ParseIntError),
}
