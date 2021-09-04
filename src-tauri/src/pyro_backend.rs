use crate::{commands::Cmd::*, documents::DocumentManager, errors::UserError};

pub fn run_command(arg: &str, doc_manager: &mut DocumentManager) -> Result<(), UserError> {
  doc_manager.ensure_resources()?;

  match serde_json::from_str(arg).unwrap() {
    SaveDocument { doc_name, text } => doc_manager.save_document(doc_name, text)?,
    Crypt { password, locking } => {
      // if locking {
      //   documents::zip_documents(&documents)?;
      // }
      // crypt_operation(&password, locking)?;
      // // After decrypting unzip the archive
      // if !locking {
      //   documents::unzip_documents()?;
      // }
    }

    _ => unimplemented!(),
  }
  Ok(())
}
