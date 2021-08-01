#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
extern crate log;
mod commands;
mod errors;
mod logging;
mod pyro_backend;

use errors::UserError;
use pyro_backend::*;
use std::{
  collections::HashMap,
  sync::{Arc, Mutex},
};

use tauri::{command, State};
#[command]
fn invoke(arg: &str, documents: State<Documents>) -> Result<(), UserError> {
  let mut documents = documents.inner().lock().unwrap();

  run_command(arg, &mut documents)
}
#[command]
fn load_documents(documents: State<Documents>) -> Result<HashMap<String, Document>, UserError> {
  let documents = &mut *documents.inner().lock().unwrap();

  Ok(documents.clone())
}

fn main() {
  logging::init_file_logger()
    .unwrap_or_else(|err| panic!("Could not initialize logger, reason: {}", err));

  tauri::Builder::default()
    .manage(Arc::new(Mutex::new(
      pyro_backend::load_documents().unwrap_or_default(),
    )))
    .invoke_handler(tauri::generate_handler![invoke, load_documents])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
