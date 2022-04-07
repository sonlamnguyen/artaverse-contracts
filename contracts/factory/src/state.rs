// use schemars::JsonSchema;
// use serde::{Deserialize, Serialize};

// use cosmwasm_std::Addr;
use cw_storage_plus::Item;

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// pub struct State {
//     pub minter_code_id: i32
// }

pub const MINTER_CODE_ID: Item<u64> = Item::new("minter_code_id");
pub const CW721_CODE_ID: Item<u64> = Item::new("cw721_code_id");
