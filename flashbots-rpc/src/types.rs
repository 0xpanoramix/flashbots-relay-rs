use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct SentPrivateTransactionResponse {
    pub jsonrpc: String,
    pub id: u64,
    pub result: String,
}

#[derive(Deserialize, Debug)]
pub struct CancelledPrivateTransactionResponse {
    pub jsonrpc: String,
    pub id: u64,
    pub result: bool,
}

/// The response for flashbots_getUserStats.
#[derive(Deserialize, Debug)]
pub struct UserStats {
    pub is_high_priority: Option<bool>,
    pub all_time_miner_payments: Option<String>,
    pub all_time_gas_simulated: Option<String>,
    pub last_7d_miner_payments: Option<String>,
    pub last_7d_gas_simulated: Option<String>,
    pub last_1d_miner_payments: Option<String>,
    pub last_1d_gas_simulated: Option<String>,
}

/// The params used to query flashbots_getBundleStats.
#[derive(Serialize, Debug)]
pub struct FlashbotsGetBundleStatsParam {
    #[serde(rename = "blockNumber")]
    pub block_number: String,

    #[serde(rename = "blockHash")]
    pub block_hash: String,
}

/// The response for flashbots_getBundleStats.
#[derive(Deserialize, Debug)]
pub struct BundleStats {
    #[serde(rename = "isSimulated")]
    pub is_simulated: Option<bool>,

    #[serde(rename = "isSentToMiners")]
    pub is_sent_to_miners: Option<bool>,

    #[serde(rename = "isHighPriority")]
    pub is_high_priority: Option<bool>,

    #[serde(rename = "simulatedAt")]
    pub simulated_at: Option<String>,

    #[serde(rename = "submittedAt")]
    pub submitted_at: Option<String>,

    #[serde(rename = "sentToMinersAt")]
    pub sent_to_miners_at: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct FlashbotsPrivateTxPreferences {
    fast: bool,
}

/// The params used to query eth_sendPrivateTransaction.
#[derive(Serialize, Debug)]
pub struct FlashbotsSendPrivateTransactionParam {
    pub tx: String,

    #[serde(rename = "maxBlockNumber", skip_serializing_if = "Option::is_none")]
    pub max_block_number: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferences: Option<FlashbotsPrivateTxPreferences>,
}

/// The params used to query eth_cancelPrivateTransaction.
#[derive(Serialize, Debug)]
pub struct FlashbotsCancelPrivateTransactionParam {
    #[serde(rename = "txHash")]
    pub tx_hash: String,
}
