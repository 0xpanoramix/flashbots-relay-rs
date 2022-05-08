use serde::Deserialize;

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
