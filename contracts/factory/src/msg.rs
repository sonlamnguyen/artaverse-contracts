use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    /// code id of minter contract was stored
    pub minter_code_id: u64,
    /// code id of cw721 contract was stored
    pub cw721_code_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// update code id of minter contract was stored
    UpdateMinterCodeId {
        minter_code_id: u64,
    },
    /// update code id of cw721 contract was stored
    UpdateCw721CodeId {
        cw721_code_id: u64,
    },
    /// call minter contract to instantiate a minter contract
    CreateMinter {
        minter_instantiate_msg: CreateMinterInstantiateMsg,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// GetState returns the current state as a json-encoded number
    GetState {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StateResponse {
    pub(crate) minter_code_id: u64,
    pub(crate) cw721_code_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CreateMinterInstantiateMsg {
    pub base_token_uri: String,
    pub num_tokens: u32,
    pub max_tokens_per_batch_mint: u32,
    pub max_tokens_per_batch_transfer: u32,
    pub name: String,
    pub symbol: String,
    pub royalty_percentage: Option<u64>,
    pub royalty_payment_address: Option<String>,
}
