use crate::error::RelayError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum RelayResponse<T> {
    Error(RelayError),
    Result(T),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FlashbotsEthResponse<T> {
    pub jsonrpc: String,
    pub id: u64,
    pub result: T,
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

/// The params used to query eth_sendBundle.
#[derive(Serialize, Debug)]
pub struct FlashbotsSendBundleParam {
    pub txs: Vec<String>,

    #[serde(rename = "blockNumber")]
    pub block_number: String,

    #[serde(rename = "minTimestamp", skip_serializing_if = "Option::is_none")]
    pub min_timestamp: Option<u64>,

    #[serde(rename = "maxTimestamp", skip_serializing_if = "Option::is_none")]
    pub max_timestamp: Option<u64>,

    #[serde(rename = "revertingTxHashes", skip_serializing_if = "Option::is_none")]
    pub reverting_tx_hashes: Option<Vec<String>>,
}

/// The response for eth_sendBundle.
#[derive(Serialize, Deserialize, Debug)]
pub struct SendBundleResponse {
    #[serde(rename = "bundleHash")]
    pub bundle_hash: String,
}

/// The params used to query eth_callBundle.
#[derive(Serialize, Debug)]
pub struct FlashbotsCallBundleParam {
    pub txs: Vec<String>,

    #[serde(rename = "blockNumber")]
    pub block_number: String,

    #[serde(rename = "stateBlockNumber")]
    pub state_block_number: String,

    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,

    #[serde(rename = "timeout", skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,

    #[serde(rename = "gasLimit", skip_serializing_if = "Option::is_none")]
    pub gas_limit: Option<u64>,

    #[serde(rename = "difficulty", skip_serializing_if = "Option::is_none")]
    pub difficulty: Option<u64>,

    #[serde(rename = "baseFee", skip_serializing_if = "Option::is_none")]
    pub base_fee: Option<u64>,
}

/// The response for eth_callBundle.
#[derive(Deserialize, Debug)]
pub struct CallBundleResponse {
    #[serde(rename = "bundleGasPrice")]
    pub bundle_gas_price: String,

    #[serde(rename = "bundleHash")]
    pub bundle_hash: String,

    #[serde(rename = "coinbaseDiff")]
    pub coinbase_diff: String,

    #[serde(rename = "ethSentToCoinbase")]
    pub eth_sent_to_coinbase: String,

    #[serde(rename = "gasFees")]
    pub gas_fees: String,

    pub results: Vec<CallBundleResult>,

    #[serde(rename = "stateBlockNumber")]
    pub state_block_number: i64,

    #[serde(rename = "totalGasUsed")]
    pub total_gas_used: i64,
}

#[derive(Deserialize, Debug)]
pub struct CallBundleResult {
    #[serde(rename = "coinbaseDiff")]
    pub coinbase_diff: String,

    #[serde(rename = "ethSentToCoinbase")]
    pub eth_sent_to_coinbase: String,

    #[serde(rename = "fromAddress")]
    pub from_address: String,

    #[serde(rename = "gasFees")]
    pub gas_fees: String,

    #[serde(rename = "gasPrice")]
    pub gas_price: String,

    #[serde(rename = "gasUsed")]
    pub gas_used: i64,

    #[serde(rename = "toAddress")]
    pub to_address: String,

    #[serde(rename = "txHash")]
    pub tx_hash: String,

    pub value: String,

    pub error: String,

    pub revert: String,
}
