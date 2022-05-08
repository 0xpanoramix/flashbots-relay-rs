use ethers_signers::{LocalWallet, Signer};
use reqwest::header::HeaderMap;
use reqwest::{Client, ClientBuilder};
use serde_json::{json, Value};
use sha3::{Digest, Keccak256};

use crate::constants::{FLASHBOTS_AUTH_HEADER_NAME, FLASHBOTS_RELAY_RPC_ENDPOINT};
use crate::types::{BundleStats, FlashbotsGetBundleStatsParam, UserStats};

#[derive(Clone, Debug)]
pub struct Requester {
    client: Client,
    base_url: String,
}

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

    pub async fn get_user_stats(
        &self,
        private_key: &str,
        block_number: u64,
    ) -> Result<UserStats, Box<dyn std::error::Error>> {
        // Loads the ethereum wallet.
        let wallet = private_key.parse::<LocalWallet>()?;

        // Prepare the payload for POST request.
        let hex_block_number = format!("0x{:x}", block_number);
        let request_payload = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "method": "flashbots_getUserStats",
            "params": [
                hex_block_number,
            ],
        });

        // Sign the payload.
        let payload_signature = self.sign_request_payload(&wallet, &request_payload).await?;
        let signature = format!("{:#?}:0x{}", wallet.address(), payload_signature);

        // Send the request.
        let response = self
            .client
            .post(&self.base_url)
            .header(FLASHBOTS_AUTH_HEADER_NAME, signature)
            .json(&request_payload)
            .send()
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
        let request_payload = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "method": "flashbots_getBundleStats",
            "params": [
                params.block_hash,
                params.block_number,
            ],
        });

        // Sign the payload.
        let payload_signature = self.sign_request_payload(&wallet, &request_payload).await?;
        let signature = format!("{:#?}:0x{}", wallet.address(), payload_signature);

        // Send the request.
        let response = self
            .client
            .post(&self.base_url)
            .header(FLASHBOTS_AUTH_HEADER_NAME, signature)
            .json(&request_payload)
            .send()
            .await?;

        // Parse the response and return the data.
        let bundle_stats: BundleStats = response.json().await?;

        println!("{:?}", bundle_stats);
        Ok(bundle_stats)
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
}
