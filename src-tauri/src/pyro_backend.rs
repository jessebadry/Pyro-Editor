use crate::{
  commands::Cmd::*,
  errors::{PyroError, PyroError::*, UserError},
};
use jencryptlib::{decrypt_files, encrypt_files, j_file::JFile};

use serde::{Deserialize, Serialize};
use std::{
  collections::HashMap,
  fs::{read, write},
  io,
  path::Path,
  sync::{Arc, Mutex},
};
pub type Documents = Arc<Mutex<HashMap<String, Document>>>;
pub type DocumentsRaw = HashMap<String, Document>;
const DOCUMENTS_FILE: &str = "documents.json";
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Document {
  document_name: String,
  text: String,
}

fn save_documents(docs: &HashMap<String, Document>) -> Result<(), PyroError> {
  let doc_json = serde_json::to_string(docs).expect("Document cannot be converted to Json!");

  write(DOCUMENTS_FILE, doc_json)?;
  Ok(())
}

fn ensure_documents_exists() -> Result<(), PyroError> {
  if !Path::new(DOCUMENTS_FILE).exists() {
    save_documents(&HashMap::<String, Document>::new())?;
  }
  Ok(())
}

/// Retrieves all text document filenames from the documents folder
pub fn load_documents() -> io::Result<HashMap<String, Document>> {
  Ok(serde_json::from_slice(&read(DOCUMENTS_FILE)?)?)
}

fn crypt_operation(password: &str, encrypting: bool) -> Result<(), PyroError> {
  /*TODO: log errors */

  let has_header =
    JFile::file_contains_header(DOCUMENTS_FILE).expect("Couldn't read Document header!");
  if !has_header && !encrypting {
    return Err(NotEncryptedError);
  }

  let crypt_method = if encrypting {
    encrypt_files
  } else {
    decrypt_files
  };

  crypt_method(password, &[DOCUMENTS_FILE]).map_err(CryptError)
}
/// Creates or overwrites a document, then saves to disk as `DOCUMENTS_FILE`
fn save_document(
  documents: &mut DocumentsRaw,
  doc_name: String,
  text: String,
) -> Result<(), PyroError> {
  if let Some(document) = documents.get_mut(&doc_name) {
    document.text = text;
  } else {
    let document_name = doc_name.clone();
    let new_doc = Document {
      document_name,
      text,
    };
    documents.insert(doc_name, new_doc);
  }
  save_documents(&documents)?;
  Ok(())
}
pub fn run_command(arg: &str, documents: &mut HashMap<String, Document>) -> Result<(), UserError> {
  ensure_documents_exists()?;
  
  match serde_json::from_str(arg).unwrap() {
    SaveDocument { doc_name, text } => save_document(documents, doc_name, text)?,
    Crypt { password, locking } => crypt_operation(&password, locking)?,
  
    _ => unimplemented!(),
  }
  Ok(())
}
