use std::io;

pub enum Error {
    IllegalInstruction,
    IoError(io::Error),
}

pub type Result<T> = core::result::Result<T, Error>;

impl From<io::Error> for Error {
    fn from(src: io::Error) -> Error {
        Error::IoError(src)
    }
}
