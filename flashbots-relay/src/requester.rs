use ethers_signers::{LocalWallet, Signer};
use reqwest::header::HeaderMap;
use reqwest::{Client, ClientBuilder, Response};
use serde_json::{json, Value};
use sha3::{Digest, Keccak256};

use crate::constants::{FLASHBOTS_AUTH_HEADER_NAME, FLASHBOTS_RELAY_RPC_ENDPOINT};
use crate::types::{
    BundleStats, CallBundleResponse, FlashbotsCallBundleParam,
    FlashbotsCancelPrivateTransactionParam, FlashbotsEthResponse, FlashbotsGetBundleStatsParam,
    FlashbotsSendBundleParam, FlashbotsSendPrivateTransactionParam, RelayResponse,
    SendBundleResponse, UserStats,
};

/// The object used to interact with the Flashbots' endpoint.
/// Contains both the client and the endpoint to target.
#[derive(Clone, Debug)]
pub struct Requester {
    client: Client,
    base_url: String,
}

/// Creates a default requester connected to the mai network.
impl Default for Requester {
    fn default() -> Self {
        let mut client_builder = ClientBuilder::new();
        let mut headers = HeaderMap::new();

        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("Accept", "application/json".parse().unwrap());
        client_builder = client_builder.default_headers(headers);

        Self {
            client: client_builder.build().unwrap(),
            base_url: FLASHBOTS_RELAY_RPC_ENDPOINT.to_string(),
        }
    }
}

impl Requester {
    /// Signs the JSON-RPC 2.0 request payload using the user's private key.
    async fn sign_request_payload(
        &self,
        wallet: &LocalWallet,
        request_payload: &Value,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Create a new hasher instance.
        let mut hasher = Keccak256::new();

        // Fill the hasher with the data to be hashed.
        hasher.update(request_payload.to_string().as_bytes());

        // Perform hash.
        let hashed_request_payload = hasher.finalize();
        let hex_hashed_request_payload = format!("0x{}", (hex::encode(hashed_request_payload)));
        let signature = wallet.sign_message(hex_hashed_request_payload).await?;

        Ok(signature.to_string())
    }

    /// Creates a new request payload using the parameters provided by the user.
    /// # Example
    ///
    /// For the `flashbots_getUserStats` endpoint it will return :
    /// ```json
    /// {
    ///   "jsonrpc": "2.0",
    ///   "id": 1,
    ///   "method": "flashbots_getUserStats",
    ///   "params": [
    ///       0x1,
    ///   ]
    /// }
    /// ```
    fn new_request_payload(&self, method: &str, params: Vec<Value>) -> Value {
        json!({
            "id": 1,
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
        })
    }

    /// Creates the request payload, signs it using the user's private key and submit the HTTP call.
    async fn call_with_flashbots_signature(
        &self,
        method: &str,
        private_key: &LocalWallet,
        request_params: Vec<Value>,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let request_payload = self.new_request_payload(method, request_params);

        // Sign the payload.
        let payload_signature = self
            .sign_request_payload(private_key, &request_payload)
            .await?;
        let signature = format!("{:#?}:0x{}", private_key.address(), payload_signature);

        // Send the request.
        let response = self
            .client
            .post(&self.base_url)
            .header(FLASHBOTS_AUTH_HEADER_NAME, signature)
            .json(&request_payload)
            .send()
            .await?;

        Ok(response)
    }

    /// Returns a quick summary of how a searcher is performing in the relay,
    /// including their reputation-based priority.
    ///
    /// It is currently updated once every hour.
    pub async fn get_user_stats(
        &self,
        private_key: &str,
        block_number: u64,
    ) -> Result<UserStats, Box<dyn std::error::Error>> {
        // Loads the ethereum wallet.
        let wallet = private_key.parse::<LocalWallet>()?;

        // Prepare the payload for POST request.
        let request_params: Vec<Value> =
            vec![serde_json::to_value(format!("0x{:x}", block_number)).unwrap()];

        // Call te relay.
        let response = self
            .call_with_flashbots_signature("flashbots_getUserStats", &wallet, request_params)
            .await?;

        // Parse the response and return the data.
        let user_stats: RelayResponse<UserStats> = response.json().await?;

        match user_stats {
            RelayResponse::Error(e) => Err(e.error.into()),
            RelayResponse::Result(r) => Ok(r),
        }
    }

    /// Returns stats for a single bundle.
    ///
    /// You must provide a blockNumber and the bundleHash, and the signing address must be the
    /// same as the one who submitted the bundle.
    pub async fn get_bundle_stats(
        &self,
        private_key: &str,
        params: &FlashbotsGetBundleStatsParam,
    ) -> Result<BundleStats, Box<dyn std::error::Error>> {
        // Loads the ethereum wallet.
        let wallet = private_key.parse::<LocalWallet>()?;

        // Prepare the payload for POST request.
        let request_params: Vec<Value> = vec![serde_json::to_value(&params).unwrap()];

        // Call te relay.
        let response = self
            .call_with_flashbots_signature("flashbots_getBundleStats", &wallet, request_params)
            .await?;

        // Parse the response and return the data.
        let bundle_stats: RelayResponse<BundleStats> = response.json().await?;

        match bundle_stats {
            RelayResponse::Error(e) => Err(e.error.into()),
            RelayResponse::Result(r) => Ok(r),
        }
    }

    /// Used to send a single transaction to Flashbots.
    ///
    /// Flashbots will attempt to send the transaction to miners for the next 25 blocks.
    pub async fn send_private_transaction(
        &self,
        private_key: &str,
        params: &FlashbotsSendPrivateTransactionParam,
    ) -> Result<FlashbotsEthResponse<String>, Box<dyn std::error::Error>> {
        // Loads the ethereum wallet.
        let wallet = private_key.parse::<LocalWallet>()?;

        // Prepare the payload for POST request.
        let request_params: Vec<Value> = vec![serde_json::to_value(&params).unwrap()];

        // Call te relay.
        let response = self
            .call_with_flashbots_signature("eth_sendPrivateTransaction", &wallet, request_params)
            .await?;

        // Parse the response and return the data.
        let tx_hash: RelayResponse<FlashbotsEthResponse<String>> = response.json().await?;

        match tx_hash {
            RelayResponse::Error(e) => Err(e.error.into()),
            RelayResponse::Result(r) => Ok(r),
        }
    }

    /// Stops private transactions from being submitted for future blocks.
    ///
    /// A transaction can only be cancelled if the request is signed by the same
    /// key as the `eth_sendPrivateTransaction` call submitting the transaction in first place.
    pub async fn cancel_private_transaction(
        &self,
        private_key: &str,
        params: &FlashbotsCancelPrivateTransactionParam,
    ) -> Result<FlashbotsEthResponse<bool>, Box<dyn std::error::Error>> {
        // Loads the ethereum wallet.
        let wallet = private_key.parse::<LocalWallet>()?;

        // Prepare the payload for POST request.
        let request_params: Vec<Value> = vec![serde_json::to_value(&params).unwrap()];

        // Call te relay.
        let response = self
            .call_with_flashbots_signature("eth_cancelPrivateTransaction", &wallet, request_params)
            .await?;

        // Parse the response and return the data.
        let result: RelayResponse<FlashbotsEthResponse<bool>> = response.json().await?;

        match result {
            RelayResponse::Error(e) => Err(e.error.into()),
            RelayResponse::Result(r) => Ok(r),
        }
    }

    /// Used to send your bundles to the relay.
    pub async fn send_bundle(
        &self,
        private_key: &str,
        params: &FlashbotsSendBundleParam,
    ) -> Result<FlashbotsEthResponse<SendBundleResponse>, Box<dyn std::error::Error>> {
        // Loads the ethereum wallet.
        let wallet = private_key.parse::<LocalWallet>()?;

        // Prepare the payload for POST request.
        let request_params: Vec<Value> = vec![serde_json::to_value(&params).unwrap()];

        // Call te relay.
        let response = self
            .call_with_flashbots_signature("eth_sendBundle", &wallet, request_params)
            .await?;

        // Parse the response and return the data.
        let result: RelayResponse<FlashbotsEthResponse<SendBundleResponse>> =
            response.json().await?;

        match result {
            RelayResponse::Error(e) => Err(e.error.into()),
            RelayResponse::Result(r) => Ok(r),
        }
    }

    /// Used to simulate a bundle against a specific block number, including simulating
    /// a bundle at the top of the next block.
    pub async fn call_bundle(
        &self,
        private_key: &str,
        params: &FlashbotsCallBundleParam,
    ) -> Result<FlashbotsEthResponse<CallBundleResponse>, Box<dyn std::error::Error>> {
        // Loads the ethereum wallet.
        let wallet = private_key.parse::<LocalWallet>()?;

        // Prepare the payload for POST request.
        let request_params: Vec<Value> = vec![serde_json::to_value(&params).unwrap()];

        // Call te relay.
        let response = self
            .call_with_flashbots_signature("eth_callBundle", &wallet, request_params)
            .await?;

        // Parse the response and return the data.
        let result: RelayResponse<FlashbotsEthResponse<CallBundleResponse>> =
            response.json().await?;

        match result {
            RelayResponse::Error(e) => Err(e.error.into()),
            RelayResponse::Result(r) => Ok(r),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::constants::FLASHBOTS_RELAY_RPC_ENDPOINT;
    use crate::requester::Requester;
    use crate::types::{FlashbotsGetBundleStatsParam, FlashbotsSendBundleParam};

    #[test]
    fn it_can_instantiate_requester_with_default_configuration() {
        let requester = Requester::default();

        assert_eq!(requester.base_url, FLASHBOTS_RELAY_RPC_ENDPOINT);
    }

    #[tokio::test]
    async fn it_can_get_user_stats() {
        let requester = Requester::default();
        let private_key = "dcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72f7";
        let result = requester.get_user_stats(private_key, 14737875).await;

        assert_eq!(result.is_err(), false);
    }

    #[tokio::test]
    async fn it_can_get_bundle_stats() {
        let requester = Requester::default();
        let private_key = "dcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72f7";
        let params: FlashbotsGetBundleStatsParam = FlashbotsGetBundleStatsParam {
            block_number: "0xe0e368".to_string(),
            block_hash: "0x49628419847f55d9c5c9b912cdab27d916880a12f7f816cc69a0fd4b0c5430f3"
                .to_string(),
        };
        let result = requester.get_bundle_stats(private_key, &params).await;

        assert_eq!(result.is_err(), false);
    }

    #[tokio::test]
    async fn it_can_send_private_transaction() -> Result<(), Box<dyn std::error::Error>> {
        /*
        let requester = Requester::default();
        let private_key = "dcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72f7";

        let wallet = private_key
            .parse::<LocalWallet>()?;

        // create a transaction
        let tx = TransactionRequest::new()
            .to("vitalik.eth") // this will use ENS
            .value(10000).into();

        // sign it
        let signature = wallet.sign_transaction(&tx).await?;

        let params = FlashbotsSendPrivateTransactionParam {
            tx: format!("0x{}", signature.to_string()),
            max_block_number: None,
            preferences: None
        };
        let result = requester.send_private_transaction(private_key, &params).await;

        assert_eq!(result.is_err(), false);
        */
        Ok(())
    }

    #[tokio::test]
    async fn it_can_send_bundle() -> Result<(), Box<dyn std::error::Error>> {
        let requester = Requester::default();
        let private_key = "dcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72f7";

        let params = FlashbotsSendBundleParam {
            txs: vec!["0xf86b808459682efe825208944592d8f8d7b001e72cb26a73e4fa1806a51ac79d880de0b6b3a7640000802ea0e96dfa6f3ae80f7b55e016bc9b140762cb86e2c08bfac6b20c5b6035bdf36611a00fdb494e6f842fcbc0bddb571c5035148446a77354a90e7dc4b0e9feafabaeda".to_string()],
            block_number: "0xcaa6fa".to_string(),
            min_timestamp: None,
            max_timestamp: None,
            reverting_tx_hashes: None
        };

        let result = requester.send_bundle(private_key, &params).await;
        assert_eq!(result.is_err(), false);

        let data = result.unwrap();
        assert_eq!(
            data.result.bundle_hash,
            "0xcf85838f6ef4946f285b991a70c9732902b70a98f9615754a406651e59bbcaec"
        );
        Ok(())
    }

    #[tokio::test]
    async fn it_fails_to_send_invalid_bundle() -> Result<(), Box<dyn std::error::Error>> {
        let requester = Requester::default();
        let private_key = "dcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72f7";

        let params = FlashbotsSendBundleParam {
            txs: vec!["0xdeadbeef".to_string()],
            block_number: "0xcaa6fa".to_string(),
            min_timestamp: None,
            max_timestamp: None,
            reverting_tx_hashes: None,
        };

        let result = requester.send_bundle(private_key, &params).await;
        assert_eq!(result.is_err(), true);
        Ok(())
    }
}
