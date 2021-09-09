use crate::errors::{PyroError, PyroError::*};

use jencrypt::{decrypt_files, encrypt_files, j_file::JFile};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, error::Error, fmt};
type Result<T, E = DocError> = std::result::Result<T, E>;

const DOCUMENTS_DB: &str = "documents.sqlite";
const DOC_NAME_FIELD: &str = "name";
const DOC_TABLE: &str = "Documents";

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Document {
  document_name: String,
  text: String,
}
impl Document {
  pub fn new(doc_name: String, text: String) -> Self {
    Self {
      document_name: doc_name,
      text,
    }
  }
}
#[derive(Debug)]
enum DocError {
  DbError(rusqlite::Error),
  DocumentNotFound,
  LockedError,
}
impl fmt::Display for DocError {
  fn fmt(&self, _fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    todo!("Implement user error messages.")
  }
}
impl std::error::Error for DocError {}
impl From<rusqlite::Error> for DocError {
  fn from(err: rusqlite::Error) -> DocError {
    DbError(err)
  }
}
use DocError::*;

pub struct DocumentManager {
  locked: bool,
}
impl DocumentManager {
  fn initialize_table() -> Result<()> {
    let conn = Self::conn()?;

    conn.execute(
      &format!(
        "CREATE TABLE [IF NOT EXISTS] {} (
        name TEXT NOT NULL,
        text TEXT NOT NULL,
        )
      ",
        DOC_TABLE
      ),
      [],
    )?;

    Ok(())
  }
  pub fn new() -> Result<Self> {
    Self::initialize_table()?;
    // If the file contains the scrypt header we can assume the file is encrypted.
    let locked = JFile::file_contains_header(DOCUMENTS_DB).unwrap_or(false);

    Ok(Self { locked })
  }

  fn conn() -> Result<Connection> {
    Connection::open(DOCUMENTS_DB).map_err(DbError)
  }

  fn get_doc_names() -> HashSet<String> {
    unimplemented!()
  }

  pub fn save_document(&self, doc: &Document) -> Result<()> {
    if self.locked {
      return Err(LockedError);
    }

    let conn = Self::conn()?;

    conn.execute(
      &format!("INSERT OR REPLACE INTO {} VALUES(:name, :text)", DOC_TABLE),
      &[(":name", &doc.document_name), (":text", &doc.text)],
    )?;

    Ok(())
  }

  pub fn find_doc_by_name(doc_name: String) -> Result<Document> {
    let conn = Self::conn()?;

    let stmt = conn.prepare("SELECT name, text FROM Documents where {} = :name")?;
    let rows = stmt.query(&[(":name", &doc_name)])?;

    if let Some(row) = rows.next()? {
      Ok(Document {
        document_name: row.get(0)?,
        text: row.get(1)?,
      })
    } else {
      Err(DocError::DocumentNotFound)
    }
  }

  pub fn load_document_names() -> Result<HashSet<String>> {
    let records =
      Self::conn()?.prepare(&format!("SELECT {} FROM {}", DOC_NAME_FIELD, DOC_TABLE))?;

    let mut doc_names = records.query_map([], |row| row.get(0))?;

    let mut names = HashSet::<String>::new();
    for name in doc_names {
      names.insert(name?);
    }

    Ok(names)
  }

  fn crypt_operation(password: &str, encrypting: bool) -> Result<(), PyroError> {
    /*TODO: log errors */
    let has_header = JFile::file_contains_header(DOCUMENTS_DB)?;
    // if no header and we are decrypting, this means this file is not encrypted.
    if !has_header && !encrypting {
      return Err(NotEncryptedError);
    }

    let crypt_method = if encrypting {
      encrypt_files
    } else {
      decrypt_files
    };

    crypt_method(password, &[DOCUMENTS_DB])
      .map_err(CryptError)?
      .try_for_each(|res| res)?;

    Ok(())
  }

  pub fn lock(&self, password: &str) -> Result<(), PyroError> {
    Self::crypt_operation(password, true)?;

    Ok(())
  }
  pub fn unlock(&self, password: &str) -> Result<(), PyroError> {
    Self::crypt_operation(password, false)?;

    Ok(())
  }
}

pub fn default_doc_names() -> HashSet<String> {
  let mut new_set = HashSet::<String>::new();

  todo!("get document names from sqlite db")
}

#[cfg(test)]
mod tests {
  use super::DocumentManager;

  #[test]
  fn test_crypt_operation() {}

  #[test]
  fn make_documents() {
    let mut manager = DocumentManager::new().expect("Could not initialize sqlite table!");
    manager.unlock("p123");
  }
}
