#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]
extern crate log;

mod commands;
mod documents;
mod errors;
mod logging;
mod pyro_backend;

use documents::DocumentManager;
use errors::UserError;
use pyro_backend::*;
use tauri::{command, State};
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

type SafeDocManager = Arc<Mutex<DocumentManager>>;

const PYRO_DB_NAME: &str = "pyroDB.sqlite3";


#[command]
fn invoke(arg: &str, documents: State<SafeDocManager>) -> Result<(), UserError> {
    let mut documents = documents.inner().lock().unwrap();

    run_command(arg, &mut documents)
}

#[command]
fn load_documents(documents: State<SafeDocManager>) -> Result<HashSet<String>, UserError> {
    let doc_manager = &mut *documents.inner().lock().unwrap();

    Ok(doc_manager.load_document_names()?.clone())
}

fn main() {
    let mut document_manager = DocumentManager::new(PYRO_DB_NAME);

    document_manager.initialize_table()?;

    logging::init_file_logger().expect("Could not initialize logger");

    tauri::Builder::default()
        .manage(Arc::new(Mutex::new(document_manager)))
        .invoke_handler(tauri::generate_handler![invoke])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
