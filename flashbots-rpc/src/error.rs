use serde::de::StdError;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Debug)]
pub struct RelayError {
    pub jsonrpc: String,
    pub id: u64,
    pub error: ErrorContent,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorContent {
    pub code: i64,
    pub message: String,
}

impl Display for ErrorContent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "error: {}. code: {}", self.message, self.code)
    }
}

impl StdError for ErrorContent {}
