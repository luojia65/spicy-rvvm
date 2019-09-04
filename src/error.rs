pub enum Error {
    IllegalInstruction,
}

pub type Result<T> = core::result::Result<T, Error>;
