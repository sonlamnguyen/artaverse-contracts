#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, SubMsg, WasmMsg, ReplyOn, Reply, Addr};
// use cosmwasm_std::ReplyOn::Error;
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, StateResponse};
use crate::state::{CW721_CODE_ID, MINTER_ADDRESS, MINTER_CODE_ID};
use cw_utils::{parse_reply_instantiate_data};
use minter::msg::InstantiateMsg as MinterInstantiateMsg;

const INSTANTIATE_MINTER_REPLY_ID: u64 = 1;
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
            base_token_uri,
            num_tokens,
            max_tokens_per_batch_mint,
            name,
            symbol,
            royalty_percentage,
            royalty_payment_address, }
        => create_minter(deps,
                         info,
                         base_token_uri,
                         num_tokens,
                         max_tokens_per_batch_mint,
                         name,
                         symbol,
                         royalty_percentage,
                         royalty_payment_address, ),
    }
}

pub fn update_minter_code_id(deps: DepsMut, minter_code_id: u64) -> Result<Response, ContractError> {
    MINTER_CODE_ID.save(deps.storage, &minter_code_id)?;
    Ok(Response::new()
        .add_attribute("method", "update_minter_code_id")
        .add_attribute("minter_Code_ID", minter_code_id.to_string()))
}

pub fn update_cw721_code_id(deps: DepsMut, cw721_code_id: u64) -> Result<Response, ContractError> {
    CW721_CODE_ID.save(deps.storage, &cw721_code_id)?;
    Ok(Response::new()
        .add_attribute("method", "update_cw721_code_id")
        .add_attribute("minter_Code_ID", cw721_code_id.to_string()))
}

pub fn create_minter(
    deps: DepsMut,
    info: MessageInfo,
    base_token_uri: String,
    num_tokens: u32,
    max_tokens_per_batch_mint: u32,
    name: String,
    symbol: String,
    royalty_percentage: Option<u64>,
    royalty_payment_address: Option<String>)
    -> Result<Response, ContractError> {
    let minter_code_id = MINTER_CODE_ID.load(deps.storage)?;
    let cw721_code_id = CW721_CODE_ID.load(deps.storage)?;
    _execute_create_minter(
        info,
        base_token_uri,
        num_tokens,
        max_tokens_per_batch_mint,
        name,
        symbol,
        royalty_percentage,
        royalty_payment_address,
        minter_code_id,
        cw721_code_id,
    )
}

fn _execute_create_minter(
    info: MessageInfo,
    base_token_uri: String,
    num_tokens: u32,
    max_tokens_per_batch_mint: u32,
    name: String,
    symbol: String,
    royalty_percentage: Option<u64>,
    royalty_payment_address: Option<String>,
    minter_code_id: u64,
    cw721_code_id: u64)
    -> Result<Response, ContractError> {

    // Sub-message to instantiate minter contract
    let sub_msgs: Vec<SubMsg> = vec![SubMsg {
        id: INSTANTIATE_MINTER_REPLY_ID,
        msg: WasmMsg::Instantiate {
            admin: Some(info.sender.to_string()),
            code_id: minter_code_id,
            msg: to_binary(&MinterInstantiateMsg {
                name: String::from(name),
                symbol: String::from(symbol),
                base_token_uri: String::from(base_token_uri),
                max_tokens_per_batch_mint,
                royalty_percentage,
                royalty_payment_address,
                num_tokens,
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

#[cfg(test)]
mod tests {
    // use super::*;
    // use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
    // use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        // let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
        //
        // let msg = InstantiateMsg { count: 17 };
        // let info = mock_info("creator", &coins(1000, "earth"));
        //
        // // we can just call .unwrap() to assert this was a success
        // let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        // assert_eq!(0, res.messages.len());
        //
        // // it worked, let's query the state
        // let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        // let value: CountResponse = from_binary(&res).unwrap();
        // assert_eq!(17, value.count);
    }

    #[test]
    fn increment() {
        // let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
        //
        // let msg = InstantiateMsg { count: 17 };
        // let info = mock_info("creator", &coins(2, "token"));
        // let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        //
        // // beneficiary can release it
        // let info = mock_info("anyone", &coins(2, "token"));
        // let msg = ExecuteMsg::Increment {};
        // let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        //
        // // should increase counter by 1
        // let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        // let value: CountResponse = from_binary(&res).unwrap();
        // assert_eq!(18, value.count);
    }

    #[test]
    fn reset() {
        // let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
        //
        // let msg = InstantiateMsg { count: 17 };
        // let info = mock_info("creator", &coins(2, "token"));
        // let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        //
        // // beneficiary can release it
        // let unauth_info = mock_info("anyone", &coins(2, "token"));
        // let msg = ExecuteMsg::Reset { count: 5 };
        // let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        // match res {
        //     Err(ContractError::Unauthorized {}) => {}
        //     _ => panic!("Must return unauthorized error"),
        // }
        //
        // // only the original creator can reset the counter
        // let auth_info = mock_info("creator", &coins(2, "token"));
        // let msg = ExecuteMsg::Reset { count: 5 };
        // let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();
        //
        // // should now be 5
        // let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        // let value: CountResponse = from_binary(&res).unwrap();
        // assert_eq!(5, value.count);
    }
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
        Err(_) => Err(ContractError::InstantiateSg721Error {}),
    }
}
