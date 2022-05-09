use serde::{Deserialize, Serialize};

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
