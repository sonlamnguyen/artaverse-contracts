#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, SubMsg, WasmMsg, ReplyOn, Reply, Addr};
// use cosmwasm_std::ReplyOn::Error;
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{CreateMinterInstantiateMsg, ExecuteMsg, InstantiateMsg, QueryMsg, StateResponse};
use crate::state::{CW721_CODE_ID, MINTER_ADDRESS, MINTER_CODE_ID};
use cw_utils::{parse_reply_instantiate_data};
use minter::msg::InstantiateMsg as MinterInstantiateMsg;

pub(crate) const INSTANTIATE_MINTER_REPLY_ID: u64 = 1;
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:factory";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // Check the id is bigger than zero
    if msg.minter_code_id == 0 || msg.cw721_code_id == 0 {
        return Err(ContractError::InvalidID);
    }

    MINTER_CODE_ID.save(deps.storage, &msg.minter_code_id)?;
    CW721_CODE_ID.save(deps.storage, &msg.cw721_code_id)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("contract_name", CONTRACT_NAME)
        .add_attribute("contract_version", CONTRACT_VERSION)
        .add_attribute("minter_code_id", msg.minter_code_id.to_string())
        .add_attribute("cw721_code_id", msg.cw721_code_id.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateMinterCodeId { minter_code_id } => update_minter_code_id(deps, minter_code_id),
        ExecuteMsg::UpdateCw721CodeId { cw721_code_id } => update_cw721_code_id(deps, cw721_code_id),
        ExecuteMsg::CreateMinter {
            minter_instantiate_msg
        }
        => create_minter(deps,
                         info,
                         minter_instantiate_msg),
    }
}

pub fn update_minter_code_id(deps: DepsMut, minter_code_id: u64) -> Result<Response, ContractError> {
    // Check the id is bigger than zero
    if minter_code_id == 0 {
        return Err(ContractError::InvalidID);
    }
    MINTER_CODE_ID.save(deps.storage, &minter_code_id)?;
    Ok(Response::new()
        .add_attribute("method", "update_minter_code_id")
        .add_attribute("minter_Code_ID", minter_code_id.to_string()))
}

pub fn update_cw721_code_id(deps: DepsMut, cw721_code_id: u64) -> Result<Response, ContractError> {
    // Check the id is bigger than zero
    if cw721_code_id == 0 {
        return Err(ContractError::InvalidID);
    }
    CW721_CODE_ID.save(deps.storage, &cw721_code_id)?;
    Ok(Response::new()
        .add_attribute("method", "update_cw721_code_id")
        .add_attribute("minter_Code_ID", cw721_code_id.to_string()))
}

pub fn create_minter(
    deps: DepsMut,
    info: MessageInfo,
    minter_instantiate_msg: CreateMinterInstantiateMsg)
    -> Result<Response, ContractError> {
    let minter_code_id = MINTER_CODE_ID.load(deps.storage)?;
    let cw721_code_id = CW721_CODE_ID.load(deps.storage)?;
    _execute_create_minter(
        info,
        minter_instantiate_msg,
        minter_code_id,
        cw721_code_id,
    )
}

fn _execute_create_minter(
    info: MessageInfo,
    minter_instantiate_msg: CreateMinterInstantiateMsg,
    minter_code_id: u64,
    cw721_code_id: u64)
    -> Result<Response, ContractError> {

    // Sub-message to instantiate minter contract
    let sub_msgs: Vec<SubMsg> = vec![SubMsg {
        id: INSTANTIATE_MINTER_REPLY_ID,
        msg: WasmMsg::Instantiate {
            admin: Some(info.sender.to_string()),
            code_id: minter_code_id,
            msg: to_binary(
                &MinterInstantiateMsg {
                    name: minter_instantiate_msg.name,
                    symbol: minter_instantiate_msg.symbol,
                    base_token_uri: minter_instantiate_msg.base_token_uri,
                    max_tokens_per_batch_mint: minter_instantiate_msg.max_tokens_per_batch_mint,
                    royalty_percentage: minter_instantiate_msg.royalty_percentage,
                    royalty_payment_address: minter_instantiate_msg.royalty_payment_address,
                    num_tokens: minter_instantiate_msg.num_tokens,
                    cw721_code_id,
                })?,
            funds: vec![],
            label: String::from("Create minter"),
        }.into(),
        gas_limit: None,
        reply_on: ReplyOn::Success,
    }];

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("contract_name", CONTRACT_NAME)
        .add_attribute("contract_version", CONTRACT_VERSION)
        .add_submessages(sub_msgs))
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetState {} => to_binary(&_query_state(deps)?),
    }
}

fn _query_state(deps: Deps) -> StdResult<StateResponse> {
    let minter_code_id = MINTER_CODE_ID.load(deps.storage)?;
    let cw721_code_id = CW721_CODE_ID.load(deps.storage)?;
    Ok(StateResponse { minter_code_id, cw721_code_id })
}

// Reply callback triggered from minter contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    if msg.id != INSTANTIATE_MINTER_REPLY_ID {
        return Err(ContractError::InvalidReplyID {});
    }

    let reply = parse_reply_instantiate_data(msg);
    match reply {
        Ok(res) => {
            MINTER_ADDRESS.save(deps.storage, &Addr::unchecked(res.contract_address))?;
            Ok(Response::default().add_attribute("action", "instantiate_minter_reply"))
        }
        Err(_) => Err(ContractError::InstantiateMinterError {}),
    }
}
