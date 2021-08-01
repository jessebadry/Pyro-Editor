use std::error;
use std::io;

use serde::{Serialize};

#[derive(Serialize)]
/// Used to propagate PyroError's to the user interface, all errors should be handled using this struct.
pub struct UserError {
    pub error_name: String,
    pub details: Option<String>,
}
impl From<PyroError> for UserError {
    /// Custom conversion method to include line number for debugging purposes.
    fn from(pyro_error: PyroError) -> Self {
        let (name, details) = match pyro_error {
   
            PyroError::ParsingError(err) => ("Parsing Error", Some(err.to_string())),
           
            PyroError::NotEncryptedError => ("NotEncryptedError", None),
            _ => unimplemented!(),
        };

        UserError {
            error_name: name.to_string(),
            details,
        }
    }
}

#[derive(Debug)]
pub enum PyroError {
    CryptError(std::io::Error),
    IOError(std::io::Error),
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
        let message = match self {
           
            Self::IOError(io_error) => io_error.to_string(),
            _ => unimplemented!(),
        };
        write!(f, "{}", &message)
    }
}
impl error::Error for PyroError {}
