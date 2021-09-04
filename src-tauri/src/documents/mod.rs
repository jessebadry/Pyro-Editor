use crate::errors::{PyroError, PyroError::*};

use jencrypt::{decrypt_files, encrypt_files, j_file::JFile};
use serde::{Deserialize, Serialize};
use std::{
  collections::HashSet,
  fs::{read, write, File, OpenOptions},
  io,
  io::{Read, Write},
  path::Path,
};

use zip::{result::ZipResult, write::FileOptions as ZipFileOptions, ZipArchive, ZipWriter};
pub const DOCUMENTS_FILE: &str = "documents.zip";
pub const DOCUMENT_NAMES_FILE: &str = "docs.json";
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
pub struct DocumentManager {
  pub document_names: HashSet<String>,
  locked: bool,
}
impl DocumentManager {
  pub fn new() -> Self {
    // If the file contains the scrypt header we can assume the file is encrypted.
    let locked = JFile::file_contains_header(DOCUMENTS_FILE).unwrap_or(false);

    Self {
      document_names: load_document_names().unwrap_or(default_doc_names()),
      locked,
    }
  }
  pub fn ensure_file_exists(
    &self,
    file_path: impl AsRef<Path>,
    default_content: &[u8],
  ) -> Result<(), PyroError> {
    if !file_path.as_ref().exists() {
      write(file_path, default_content)?;
    }
    Ok(())
  }
  pub fn ensure_resources(&self) -> Result<(), PyroError> {
    // if we are unlocked, this means the files are also extracted
    if !self.locked {
      self.ensure_file_exists(DOCUMENT_NAMES_FILE, b"[]")?;
    }
    self.ensure_file_exists(DOCUMENTS_FILE, b"")?;

    Ok(())
  }
  /// Creates or overwrites a document, then saves to disk using `doc_name` as its save name.
  pub fn save_document(&mut self, doc_name: String, text: String) -> Result<(), PyroError> {
    let doc_name = doc_name.to_lowercase();

    let (document, save_names) = if self.document_names.contains(&doc_name) {
      let mut document = load_document(&doc_name)?;

      document.text = text;

      (document, false)
    } else {
      self.document_names.insert(doc_name.clone());
      let document = Document::new(doc_name, text);
      self.save_document_names()?;

      (document, true)
    };
    write_document(&document)?;
    Ok(())
  }
  fn save_document_names(&self) -> Result<(), PyroError> {
    let doc_json = serde_json::to_string(&self.document_names)
      .expect("Document names cannot be converted to Json!");
    write(DOCUMENT_NAMES_FILE, doc_json)?;
    Ok(())
  }
  pub fn unzip_documents() -> Result<(), PyroError> {
    let mut zip_archive = get_zip_archive()?;
    let mut document_contents = String::new();

    for index in 0..zip_archive.len() {
      let mut zip_file = zip_archive.by_index(index)?;
      zip_file.read_to_string(&mut document_contents)?;
      write(
        zip_file.enclosed_name().ok_or(CorruptZipError)?,
        &document_contents,
      )?;
      document_contents.clear();
    }

    Ok(())
  }
  pub fn zip_documents(&self) -> Result<(), PyroError> {
    let mut zip_writer = create_zip_writer()?;

    let options = ZipFileOptions::default()
      .compression_method(zip::CompressionMethod::Stored)
      .unix_permissions(0o755);

    for doc_name in &self.document_names {
      zip_writer.start_file(doc_name, options)?;
      zip_writer.write_all(&read(doc_name)?)?;
    }

    Ok(())
  }

  fn crypt_operation(password: &str, encrypting: bool) -> Result<(), PyroError> {
    /*TODO: log errors */
    let has_header =
      JFile::file_contains_header(DOCUMENTS_FILE).expect("Couldn't read Document header!");
    // if no header and we are decrypting, this means this file is not encrypted.
    if !has_header && !encrypting {
      return Err(NotEncryptedError);
    }

    let crypt_method = if encrypting {
      encrypt_files
    } else {
      decrypt_files
    };

    crypt_method(password, &[DOCUMENTS_FILE])
      .map_err(CryptError)?
      .try_for_each(|res| res)?;

    Ok(())
  }
  pub fn lock(&self, password: &str) -> Result<(), PyroError> {
    self.zip_documents()?;
    Self::crypt_operation(password, true)?;
    Ok(())
  }
  pub fn unlock(&self, password: &str) -> Result<(), PyroError> {
    Self::crypt_operation(password, false)?;
    Self::unzip_documents()?;
    Ok(())
  }
}

fn get_zip_archive() -> ZipResult<ZipArchive<File>> {
  ZipArchive::new(
    OpenOptions::new()
      .read(true)
      .write(true)
      .create(true)
      .open(DOCUMENTS_FILE)?,
  )
}
fn create_zip_writer() -> io::Result<ZipWriter<File>> {
  Ok(ZipWriter::new(File::create(DOCUMENTS_FILE)?))
}

pub fn default_doc_names() -> HashSet<String> {
  let mut new_set = HashSet::new();

  new_set.insert(DOCUMENT_NAMES_FILE.to_string());

  new_set
}

/// Retrieves all text document filenames from the documents folder
pub fn load_document_names() -> Result<HashSet<String>, PyroError> {
  Ok(serde_json::from_slice(&read(DOCUMENTS_FILE)?)?)
}

fn load_document(doc_name: &str) -> Result<Document, PyroError> {
  Ok(serde_json::from_slice(&read(doc_name)?)?)
}
fn write_document(document: &Document) -> io::Result<()> {
  write(&document.document_name, &document.text)
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_crypt_operation() {
    
  }
}
