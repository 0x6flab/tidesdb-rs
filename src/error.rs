use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Memory allocation error")]
    Memory,

    #[error("Invalid arguments")]
    InvalidArgs,

    #[error("Key not found")]
    NotFound,

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Data corruption detected")]
    Corruption,

    #[error("Resource already exists")]
    Exists,

    #[error("Conflict detected")]
    Conflict,

    #[error("Value too large")]
    TooLarge,

    #[error("Memory limit exceeded")]
    MemoryLimit,

    #[error("Invalid database")]
    InvalidDb,

    #[error("Unknown error: {0}")]
    Unknown(i32),

    #[error("Invalid UTF-8")]
    InvalidUtf8(#[from] std::str::Utf8Error),

    #[error("Nul byte in string")]
    Nul(#[from] std::ffi::NulError),
}

impl Error {
    pub fn from_code(code: i32) -> Self {
        match code {
            0 => panic!("TDB_SUCCESS should not be converted to Error"),
            -1 => Error::Memory,
            -2 => Error::InvalidArgs,
            -3 => Error::NotFound,
            -4 => Error::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                "TidesDB I/O error",
            )),
            -5 => Error::Corruption,
            -6 => Error::Exists,
            -7 => Error::Conflict,
            -8 => Error::TooLarge,
            -9 => Error::MemoryLimit,
            -10 => Error::InvalidDb,
            _ => Error::Unknown(code),
        }
    }
}
