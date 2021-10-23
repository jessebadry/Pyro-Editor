//! User Error is used for representing user-friendly messages from the Pyro backend.
use crate::{documents::DocError, errors::PyroError};
use serde::Serialize;

type UserErrorArgs = (&'static str, Option<String>);

#[derive(Serialize)]
/// Used to propagate PyroError's to the user interface, all errors should be handled using this struct.
pub struct UserError {
    pub error_display_msg: String,
    pub debug_details: Option<String>,
}

impl From<DocError> for UserError {
    fn from(doc_error: DocError) -> Self {
        let debug_details = match &doc_error {
            DocError::DocumentNotFound => None,
            DocError::SavedWhenLocked => None,
            _ => Some(doc_error.to_string()),
        };
        let error_display_msg = doc_error.to_string();

        UserError {
            error_display_msg,
            debug_details,
        }
    }
}

impl From<PyroError> for UserError {
    fn from(pyro_error: PyroError) -> Self {
        use PyroError::*;
        let (name, details) = match pyro_error {
            ParsingError(err) => ("Parsing Error", Some(err.to_string())),
            IOError(err) => ("IO Error", Some(err.to_string())),
            NotEncryptedError => (
                "NotEncryptedError",
                Some("The Documents file is not encrypted!".into()),
            ),

            _ => ("Unimplemented Error", Some(pyro_error.to_string())),
        };

        UserError {
            error_display_msg: name.to_string(),
            debug_details: details,
        }
    }
}
