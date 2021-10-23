use serde::{Deserialize, Serialize};

use crate::documents::Document;

#[derive(Serialize, Deserialize)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
  SaveDocument { document: Document },

  Crypt { password: String, locking: bool },
}
