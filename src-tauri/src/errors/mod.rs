mod user_error;


pub use user_error::*;
use std::{error, io};
use std::fmt;


#[derive(Debug)]
pub enum PyroError {
    CryptError(jencrypt::JEncryptError),
    IOError(io::Error),
    ParsingError(serde_json::Error),
    NotEncryptedError,
}

impl From<io::Error> for PyroError {
    fn from(err: io::Error) -> PyroError {
        Self::IOError(err)
    }
}

impl From<serde_json::Error> for PyroError {
    fn from(err: serde_json::Error) -> PyroError {
        Self::ParsingError(err)
    }
}

impl std::fmt::Display for PyroError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message: String = match self {

            Self::NotEncryptedError => "The documents are not encrypted!".into(),
            Self::IOError(io_error) => "An io error has occurred!".into(),

            _ => unimplemented!(),
        };

        write!(f, "{}", &message)
    }
}

impl error::Error for PyroError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Self::NotEncryptedError => None,
            Self::ParsingError(ref e) => Some(e),
            Self::IOError(ref e) => Some(e),
            Self::CryptError(ref e) => Some(e)
        }
    }
}
