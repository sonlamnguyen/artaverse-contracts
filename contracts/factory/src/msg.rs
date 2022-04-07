use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub minter_code_id: u64,
    pub cw721_code_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    UpdateMinterCodeId { minter_code_id: u64 },
    UpdateCw721CodeId { cw721_code_id: u64 },
    CreateMinter {
        base_token_uri: String,
        num_tokens: u32,
        max_tokens_per_batch_mint: u32,
        name: String,
        symbol: String,
        royalty_percentage: Option<u64>,
        royalty_payment_address: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetState returns the current state as a json-encoded number
    GetState {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StateResponse {
    pub(crate) minter_code_id: u64,
    pub(crate) cw721_code_id: u64,
}
