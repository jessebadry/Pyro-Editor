use crate::errors::PyroError;
use serde::Serialize;
use zip::result::ZipError;
#[derive(Serialize)]
/// Used to propagate PyroError's to the user interface, all errors should be handled using this struct.
pub struct UserError {
  pub error_name: String,
  pub details: Option<String>,
}
type UserErrorArgs = (&'static str, Option<String>);
fn handle_zip_errors(zip_error: ZipError) -> UserErrorArgs {
  let name = match zip_error {
    ZipError::InvalidArchive(_) => "Invalid Zip File",
    ZipError::FileNotFound => "Zip Not Found",
    _ => "CorruptZipError",
  };

  (name, Some(zip_error.to_string()))
}

impl From<PyroError> for UserError {
  fn from(pyro_error: PyroError) -> Self {
    let (name, details) = match pyro_error {
      PyroError::ParsingError(err) => ("Parsing Error", Some(err.to_string())),
      PyroError::IOError(err) => ("IO Error", Some(err.to_string())),
      PyroError::NotEncryptedError => (
        "NotEncryptedError",
        Some("The Documents file is not encrypted!".into()),
      ),
      PyroError::ZipError(err) => handle_zip_errors(err),
      PyroError::CorruptZipError => ("CorruptZipError", None),
      _ => ("Unimplemented Error", Some(pyro_error.to_string())),
    };

    UserError {
      error_name: name.to_string(),
      details,
    }
  }
}
