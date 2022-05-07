pub mod constants;
pub mod types;

use ethers_signers::LocalWallet;
use reqwest::{Client, ClientBuilder};

#[derive(Clone, Debug)]
pub struct Requester {
    client: Client,
    base_url: String,
}

impl Requester {
    pub fn new(cfg: RequesterConfig) -> Self {
        let builder = ClientBuilder::new();
        let client = builder.build().unwrap();

        Self {
            client,
            base_url: cfg.base_url,
        }
    }

    pub fn get_user_stats(&self, private_key: LocalWallet) {

    }
}

#[derive(Clone, Debug)]
pub struct RequesterConfig {
    base_url: String,
}

impl Default for RequesterConfig {
    fn default() -> Self {
        Self {
            base_url: constants::DEFAULT_BASE_URL.parse().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_create_requester_with_default_config() {
        let _client = Requester::new(RequesterConfig::default());
    }

    #[test]
    fn it_can_create_requester_with_custom_config() {
        let cfg = RequesterConfig {
            base_url: "".to_string(),
        };
        let _client = Requester::new(cfg);
    }
}
