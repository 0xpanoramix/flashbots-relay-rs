#[derive(Serialize, Deserialize)]
pub struct FlashbotsUserStats {
    is_high_priority: bool,
    all_time_miner_payments: String,
    all_time_gas_simulated: String,
    last_7d_miner_payments: String,
    last_7d_gas_simulated: String,
    last_1d_miner_payments: String,
    last_1d_gas_simulated: String,
}