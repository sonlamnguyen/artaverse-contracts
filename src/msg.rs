use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw721_base::InstantiateMsg as CW721InstantiateMsg;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub base_token_uri: String,
    pub num_tokens: u32,
    pub cw721_code_id: u64,
    pub cw721_instantiate_msg: CW721InstantiateMsg,
    pub royalty_percentage: Option<u64>,
    pub royalty_payment_address: Option<String>,

}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Mint { token_id: u32 },
    MintTo { token_id: u32, recipient: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetCount {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CountResponse {
    pub count: i32,
}
