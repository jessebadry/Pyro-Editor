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
    /// In the event where the document manager is in memory mode, and a function attempts
    /// to use the non-existent database name, there is no physical db file to operate upon.
    NoPhysicalDb,
}

impl fmt::Display for DocError {
    fn fmt(&self, _fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            DocumentNotFound => "DocumentNotFound",
            SavedWhenLocked => "SavedWhenLocked",
            InternalDbError(err) => "An internal DB error has occurred, oops.",
            _ => unimplemented!()
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
    table_initialized: bool,
}

impl DocumentManager {
    pub fn new(db_name: impl AsRef<str>) -> Self {

        // If the file contains the scrypt header we can assume the file is encrypted.
        let locked = JFile::file_contains_header(&db_name.as_ref().to_string())
            .unwrap_or(false);

        Self {
            locked,
            db_name: Some(db_name.as_ref().to_string()),
            ..Default::default()
        }
    }


    pub fn new_in_memory() -> Self {
        Self {
            from_memory: true,
            ..Default::default()
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
            let db_name = self.db_name.ok_or(NoPhysicalDb)?;

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
        let sql = "SELECT id, name, text FROM Documents where name = :name";
        let mut stmt = conn.prepare(sql)?;

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
        let mut records = conn.prepare("select name from Documents")?;

        let mut doc_names = records.query_map([],
                                              |row| row.get(0))?;


        let mut names = HashSet::<String>::new();
        for name in doc_names {
            names.insert(name?);
        }

        Ok(names)
    }
    ///
    fn crypt_operation(&self, password: &str, encrypting: bool) -> Result<(), PyroError> {
        if let None = self.db_name {
            return Err(DocError::NoPhysicalDb);
        }


        let has_header = JFile::file_contains_header(self.db_name.unwrap().as_ref())?;

        if !(encrypting && has_header) {
            return Err(NotEncryptedError);
        }

        let crypt_method = if encrypting {
            jencrypt::encrypt_files
        } else {
            jencrypt::decrypt_files
        };

        crypt_method(password, &[&self.db_name.unwrap()])
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

impl Default for DocumentManager {
    fn default() -> Self {
        Self {
            db_name: None,
            from_memory: false,
            locked: false,
            table_initialized: false,
        }
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

        manager.find_doc_by_name("coolio");
    }
}
