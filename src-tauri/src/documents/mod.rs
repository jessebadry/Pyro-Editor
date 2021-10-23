mod document;

use crate::errors::{PyroError, PyroError::*};

use jencrypt::{decrypt_files, encrypt_files, j_file::JFile};
use rusqlite::Connection;
use std::{collections::HashSet, fmt};
use tauri::plugin::Plugin;

pub use document::Document;

type Result<T, E = DocError> = std::result::Result<T, E>;


const DOC_NAME_FIELD: &str = "name";
const DOC_TABLE: &str = "Documents";

#[derive(Debug)]
pub enum DocError {
    InternalDbError(rusqlite::Error),
    DocumentNotFound,
    SavedWhenLocked,
}

impl fmt::Display for DocError {
    fn fmt(&self, _fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            DocumentNotFound => "DocumentNotFound",
            SavedWhenLocked => "SavedWhenLocked",
            /* TODO LOGGING */
            InternalDbError(err) => "An internal DB error has occurred, oops.",
        };

        write!(_fmt, "{}", name)
    }
}

impl std::error::Error for DocError {}

impl From<rusqlite::Error> for DocError {
    fn from(err: rusqlite::Error) -> DocError {
        InternalDbError(err)
    }
}

use DocError::*;

#[derive(Debug, Clone)]
pub struct DocumentManager {
    locked: bool,
    from_memory: bool,
    db_name: Option<String>,
}

impl DocumentManager {
    pub fn new(db_name: impl AsRef<str>) -> Self {

        // If the file contains the scrypt header we can assume the file is encrypted.
        let locked = JFile::file_contains_header(db_name).unwrap_or(false);

        Self {
            locked,
            from_memory: false,
            db_name: Some(db_name.as_ref().to_string()),
        }
    }


    pub fn new_in_memory() -> Self {
        Self {
            locked: false,
            from_memory: true,
            db_name: None,
        }
    }

     /// Attempt's to create the specified DB file of the manager.
     pub fn initialize_table(&self) -> Result<()> {
        let conn = self.conn()?;
        let table_sql = &format!(
            "CREATE TABLE [IF NOT EXISTS] {} (
                 id Integer primary key,
                 name TEXT NOT NULL,
                 text TEXT NOT NULL,
                 )
                ",
            DOC_TABLE);
        conn.execute(table_sql, [1, 2, 3])?;

        Ok(())
    }
    fn conn(&self) -> Result<Connection> {
        if self.from_memory {
            Connection::open_in_memory().map_err(InternalDbError)
        } else {
            let db_name = self.db_name.expect("db name cannot be None, \
                                        from_memory must be true within this instance.");

            Connection::open(&db_name).map_err(InternalDbError)
        }
    }

    /// Saves a document.
    /// # Arguments
    /// * `doc` -
    pub fn save_document(&self, doc: &Document) -> Result<()> {
        if self.locked {
            return Err(SavedWhenLocked);
        }

        let conn = self.conn()?;

        conn.execute(
            &format!("INSERT OR REPLACE INTO {} VALUES(:name, :text)", DOC_TABLE),
            &[
                (":name", &doc.get_document_name()),
                (":text", &doc.get_text()),
            ],
        )?;

        Ok(())
    }
    /// Finds the first document where it's name equals the document name.
    ///
    /// # Arguments
    /// * `doc_name` - A string that holds the name of the targeted document
    /// # Examples
    /// ```
    /// let mut doc_manager =
    /// let document: Result<Document, DocError> =
    /// ```
    pub fn find_doc_by_name(&self, doc_name: String) -> Result<Document> {
        let conn = self.conn()?;

        let mut stmt = conn.prepare("SELECT id, name, text FROM Documents where {} = :name")?;

        let mut rows = stmt.query(&[(":name", &doc_name)])?;
        let first_row = rows.next()?;

        if let Some(row) = first_row {
            Ok(Document::new(row.get(0)?, row.get(1)?, row.get(2)?))
        } else {
            Err(DocError::DocumentNotFound)
        }
    }

    pub fn load_document_names(&self) -> Result<HashSet<String>> {
        let conn = self.conn()?;
        let mut records = conn.prepare(&format!("SELECT {} FROM {}", DOC_NAME_FIELD, DOC_TABLE))?;

        let mut doc_names = records.query_map([], |row| row.get(0))?;


        let mut names = HashSet::<String>::new();
        for name in doc_names {
            names.insert(name?);
        }

        Ok(names)
    }
    ///
    fn crypt_operation(&self, password: &str, encrypting: bool) -> Result<(), PyroError> {
        let has_header = JFile::file_contains_header(DOCUMENTS_DB)?;
        // the file is not decryptable, we assume this file is not encrypted.
        if !(encrypting && has_header) {
            return Err(NotEncryptedError);
        }

        let crypt_method = if encrypting {
            jencrypt::encrypt_files
        } else {
            jencrypt::decrypt_files
        };

        crypt_method(password, &[DOCUMENTS_DB])
            .map_err(CryptError)?
            .try_for_each(|res| res)?;

        Ok(())
    }

    pub fn lock(&self, password: &str) -> Result<(), PyroError> {
        self.crypt_operation(password, true)?;

        Ok(())
    }
    pub fn unlock(&self, password: &str) -> Result<(), PyroError> {
        self.crypt_operation(password, false)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::DocumentManager;
    macro_rules! setup {
    ($e:ident) => {
      let mut $e =  DocumentManager::new_in_memory().expect("Could not initialize sqlite table!");
    };
  }

    // fn setup()-> DocumentManager{
    //   DocumentManager::new().expect("Could not initialize sqlite table!")
    // }

    #[test]
    fn test_crypt_operation() {}

    #[test]
    fn make_documents() {
        setup!(manager);
        manager.unlock("p123");
    }

    #[test]
    fn find_doc_by_name() {
        setup!(manager);

        manager.find_doc_by_name("");
    }
}
