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
    let text_and_hash_payload = format!("\x19Ethereum Signed Message:\n{}{}", hashed_body_res_hex.len(), hashed_body_res_hex);

    let mut new_hasher = Keccak256::new();
    new_hasher.update(text_and_hash_payload.as_bytes());
    let signature_payload = new_hasher.finalize();
    let signature_payload_res: [u8; 32] = signature_payload.as_slice().try_into().expect("wrong length");
    println!("{:?}", signature_payload_res);

    let payload_signature = wallet.sign_message(signature_payload_res).await?;
    println!("{:?}", payload_signature.to_string());

    let flashbots_signature = format!("{:#?}:0x{}", wallet.address(), payload_signature.to_string());

    let _res = reqwest::Client::new()
        .post("https://relay.flashbots.net")
        .header("X-Flashbots-Signature", flashbots_signature)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&json_payload)
        .send()
        .await?;

    // println!("{:#?}", res);

    Ok(())
}