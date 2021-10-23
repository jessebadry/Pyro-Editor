use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, PartialOrd, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Document {
  id: Option<i64>,
  document_name: String,
  text: String,
}
impl Document {
  pub fn new(id: Option<i64>, doc_name: String, text: String) -> Self {
    Self {
      id,
      document_name: doc_name,
      text,
    }
  }
  pub fn get_document_name(&self) -> &str {
    &self.document_name
  }
  pub fn get_text(&self) -> &str {
    &self.text
  }
}
