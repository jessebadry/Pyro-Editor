use crate::{commands::Cmd::*, documents::DocumentManager, errors::UserError};

pub fn run_command(arg: &str, doc_manager: &mut DocumentManager) -> Result<(), UserError> {
    match serde_json::from_str(arg).unwrap() {
        SaveDocument { document } => doc_manager.save_document(&document)?,
        Crypt { password, locking } => {
            if locking {
                doc_manager.lock(&password);
            } else {
                doc_manager.unlock(&password);
            }
        }

        _ => unimplemented!(),
    }
    Ok(())
}
