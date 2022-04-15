use cosmwasm_std::Addr;
use cw_storage_plus::Item;

pub const MINTER_CODE_ID: Item<u64> = Item::new("minter_code_id");
pub const CW721_CODE_ID: Item<u64> = Item::new("cw721_code_id");
pub const MINTER_ADDRESS: Item<Addr> = Item::new("minter_address");
