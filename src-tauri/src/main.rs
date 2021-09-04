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

use std::{
  collections::HashSet,
  os::windows::thread,
  sync::{Arc, Mutex},
};
type SafeDocManager = Arc<Mutex<DocumentManager>>;

use tauri::{command, State};

#[command]
fn invoke(arg: &str, documents: State<SafeDocManager>) -> Result<(), UserError> {
  let mut documents = documents.inner().lock().unwrap();

  run_command(arg, &mut documents)
}

#[command]
fn load_documents(documents: State<SafeDocManager>) -> Result<HashSet<String>, UserError> {
  let doc_manager = &mut *documents.inner().lock().unwrap();

  Ok(doc_manager.document_names.clone())
}

fn main() {
  logging::init_file_logger().expect("Could not initialize logger");

  tauri::Builder::default()
    .manage(Arc::new(Mutex::new(DocumentManager::new())))
    .invoke_handler(tauri::generate_handler![invoke, load_documents])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
