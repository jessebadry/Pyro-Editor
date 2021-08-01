use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
  SaveDocument { doc_name: String, text: String },

  Crypt { password: String, locking: bool },
}
