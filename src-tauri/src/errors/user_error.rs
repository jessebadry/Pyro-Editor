use crate::errors::PyroError;
use serde::Serialize;
#[derive(Serialize)]
/// Used to propagate PyroError's to the user interface, all errors should be handled using this struct.
pub struct UserError {
  pub error_name: String,
  pub details: Option<String>,
}
type UserErrorArgs = (&'static str, Option<String>);

impl From<PyroError> for UserError {
  fn from(pyro_error: PyroError) -> Self {
    let (name, details) = match pyro_error {
      PyroError::ParsingError(err) => ("Parsing Error", Some(err.to_string())),
      PyroError::IOError(err) => ("IO Error", Some(err.to_string())),
      PyroError::NotEncryptedError => (
        "NotEncryptedError",
        Some("The Documents file is not encrypted!".into()),
      ),
      _ => ("Unimplemented Error", Some(pyro_error.to_string())),
    };

    UserError {
      error_name: name.to_string(),
      details,
    }
  }
}
