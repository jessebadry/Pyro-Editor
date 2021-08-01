/// Using webview, set of functions to evaluate in index.js.
use crate::errors::{PyroError, UserError};
use crate::Document;
use std::collections::HashMap;

pub fn propagate_error<'a>(pyro_error: PyroError) {
    let pyro_json = serde_json::to_string(&UserError::from(pyro_error))
        .expect("Could not convert UserError to JSON");
    // webview
    //     .eval(&format!("ffi.onError({});", pyro_json))
    //     .expect("Could not evaluate error");
}
