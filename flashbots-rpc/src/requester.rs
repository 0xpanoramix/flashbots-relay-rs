use ethers_signers::{LocalWallet, Signer};
use reqwest::header::HeaderMap;
use reqwest::{Client, ClientBuilder, Response};
use serde_json::{json, Value};
use sha3::{Digest, Keccak256};

use crate::constants::{FLASHBOTS_AUTH_HEADER_NAME, FLASHBOTS_RELAY_RPC_ENDPOINT};
use crate::types::{
    BundleStats, FlashbotsCancelPrivateTransactionParam, FlashbotsEthResponse,
    FlashbotsGetBundleStatsParam, FlashbotsSendBundleParam, FlashbotsSendPrivateTransactionParam,
    SendBundleResponse, UserStats,
};

#[derive(Clone, Debug)]
pub struct Requester {
    client: Client,
    base_url: String,
}

// TODO: Handle errors in methods.
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

    fn new_request_payload(&self, method: &str, params: Vec<Value>) -> Value {
        json!({
            "id": 1,
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
        })
    }

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
        let user_stats: UserStats = response.json().await?;
        Ok(user_stats)
    }

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
        let bundle_stats: BundleStats = response.json().await?;
        Ok(bundle_stats)
    }

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
        let tx_hash: FlashbotsEthResponse<String> = response.json().await?;
        Ok(tx_hash)
    }

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
        let result: FlashbotsEthResponse<bool> = response.json().await?;
        Ok(result)
    }

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
        let result: FlashbotsEthResponse<SendBundleResponse> = response.json().await?;
        Ok(result)
    }
}

/// TODO: Migrate tests to testnet.
#[cfg(test)]
mod tests {
    use crate::constants::FLASHBOTS_RELAY_RPC_ENDPOINT;
    use crate::requester::Requester;
    use crate::types::FlashbotsGetBundleStatsParam;

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
}
