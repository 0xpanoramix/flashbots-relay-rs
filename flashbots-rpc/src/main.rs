use ethers_signers::{LocalWallet, Signer};
use serde_json::json;
use sha3::{Keccak256, Digest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wallet = "dcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72f7"
        .parse::<LocalWallet>()?;

    let json_payload = json!({
        "id": 1,
        "jsonrpc": "2.0",
        "method": "flashbots_getUserStats",
        "params": [
            "0xe0df5e", // needs to be lower hex.
        ],
    });
    let serialized_payload = json_payload.to_string();

    let mut hasher = Keccak256::new();
    hasher.update(serialized_payload.as_bytes());
    let hashed_body = hasher.finalize();
    let hashed_body_res: [u8; 32] = hashed_body.as_slice().try_into().expect("wrong length");

    let hashed_body_res_hex = format!("0x{}", (hex::encode(hashed_body_res)));

    let payload_signature = wallet.sign_message(hashed_body_res_hex).await?;

    let flashbots_signature = format!("{:#?}:0x{}", wallet.address(), payload_signature.to_string());
    println!("{:?}", flashbots_signature);

    let res = reqwest::Client::new()
        .post("https://relay.flashbots.net")
        .header("X-Flashbots-Signature", flashbots_signature)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&json_payload)
        .send()
        .await?;

    println!("{:#?}", res.text().await?);

    Ok(())
}