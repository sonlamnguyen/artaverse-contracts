#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, StdResult, WasmMsg, ReplyOn, Reply, Addr, Response, SubMsg, Order, Empty, CosmosMsg};
use cw2::set_contract_version;
use cw721_base::{InstantiateMsg as Cw721InstantiateMsg, MintMsg, ExecuteMsg as Cw721ExecuteMsg};
use cw_utils::parse_reply_instantiate_data;

use crate::error::ContractError;
use crate::msg::{CountResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG, MINTABLE_TOKEN_IDS, MINTABLE_NUM_TOKENS, STATE, CW721_ADDRESS};


// version info for migration info
const CONTRACT_NAME: &str = "crates.io:artaverse-contracts";


const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// governance parameters
const MAX_TOKEN_LIMIT: u32 = 10000;
const INSTANTIATE_CW721_REPLY_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // Check the number of tokens is more than zero and less than the max limit
    if msg.num_tokens == 0 || msg.num_tokens > MAX_TOKEN_LIMIT {
        return Err(ContractError::InvalidNumTokens {
            min: 1,
            max: MAX_TOKEN_LIMIT,
        });
    }

    let config = Config {
        base_token_uri: msg.base_token_uri,
        num_tokens: msg.num_tokens,
        cw721_code_id: msg.cw721_code_id,
    };
    CONFIG.save(deps.storage, &config)?;
    MINTABLE_NUM_TOKENS.save(deps.storage, &msg.num_tokens)?;

    // Save mintable token ids map
    for token_id in 1..=msg.num_tokens {
        MINTABLE_TOKEN_IDS.save(deps.storage, token_id, &true)?;
    }

    // Submessage to instantiate cw721 contract
    let sub_msgs: Vec<SubMsg> = vec![SubMsg {
        id: INSTANTIATE_CW721_REPLY_ID,
        msg: WasmMsg::Instantiate {
            admin: Some(info.sender.to_string()),
            code_id: msg.cw721_code_id,
            msg: to_binary(&Cw721InstantiateMsg {
                name: msg.cw721_instantiate_msg.name,
                symbol: msg.cw721_instantiate_msg.symbol,
                minter: _env.contract.address.to_string(),
            })?,
            funds: info.funds,
            label: String::from("Check CW721"),
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
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Mint { token_id } => execute_mint_sender(deps, info, token_id),
        ExecuteMsg::MintTo { token_id, recipient } => execute_mint_to(deps, info, recipient, token_id),
    }
}

pub fn execute_mint_sender(
    deps: DepsMut,
    info: MessageInfo,
    token_id: u32,
) -> Result<Response, ContractError> {
    let recipient = info.sender.clone();
    _execute_mint(deps, info, Some(recipient), Some(token_id))
}

pub fn execute_mint_to(
    deps: DepsMut,
    info: MessageInfo,
    recipient: String,
    token_id: u32,
) -> Result<Response, ContractError> {
    let recipient = deps.api.addr_validate(&recipient)?;
    _execute_mint(deps, info, Some(recipient), Some(token_id))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_binary(&query_count(deps)?),
    }
}

fn query_count(deps: Deps) -> StdResult<CountResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(CountResponse { count: state.count })
}

fn _execute_mint(
    deps: DepsMut,
    info: MessageInfo,
    recipient: Option<Addr>,
    token_id: Option<u32>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let cw721_address = CW721_ADDRESS.load(deps.storage)?;

    let recipient_addr = match recipient {
        Some(some_recipient) => some_recipient,
        None => info.sender.clone(),
    };

    let mintable_token_id = match token_id {
        Some(token_id) => {
            if token_id == 0 || token_id > config.num_tokens {
                return Err(ContractError::InvalidTokenId {});
            }
            // If token_id not on mintable map, throw err
            if !MINTABLE_TOKEN_IDS.has(deps.storage, token_id) {
                return Err(ContractError::TokenIdAlreadySold { token_id });
            }
            token_id
        }

        None => {
            let mintable_tokens_result: StdResult<Vec<u32>> = MINTABLE_TOKEN_IDS
                .keys(deps.storage, None, None, Order::Ascending)
                .take(1)
                .collect();
            let mintable_tokens = mintable_tokens_result?;
            if mintable_tokens.is_empty() {
                return Err(ContractError::SoldOut {});
            }
            mintable_tokens[0]
        }
    };

    let mut msgs: Vec<CosmosMsg<Empty>> = vec![];

    // Create mint msgs
    let mint_msg = Cw721ExecuteMsg::Mint(MintMsg::<Empty> {
        token_id: mintable_token_id.to_string(),
        owner: recipient_addr.to_string(),
        token_uri: Some(format!("{}/{}", config.base_token_uri, mintable_token_id)),
        extension: Empty {}, // TODO: add format for royalty
    });
    let msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: cw721_address.to_string(),
        msg: to_binary(&mint_msg)?,
        funds: info.funds.clone(),
    });

    msgs.append(&mut vec![msg]);

    // Remove mintable token id from map
    MINTABLE_TOKEN_IDS.remove(deps.storage, mintable_token_id);
    let mintable_num_tokens = MINTABLE_NUM_TOKENS.load(deps.storage)?;
    // Decrement mintable num tokens
    MINTABLE_NUM_TOKENS.save(deps.storage, &(mintable_num_tokens - 1))?;

    Ok(Response::new()
        .add_attribute("sender", info.sender)
        .add_attribute("recipient", recipient_addr)
        .add_attribute("token_id", mintable_token_id.to_string())
        .add_messages(msgs))
}

// Reply callback triggered from cw721 contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    if msg.id != INSTANTIATE_CW721_REPLY_ID {
        return Err(ContractError::InvalidReplyID {});
    }

    let reply = parse_reply_instantiate_data(msg);
    match reply {
        Ok(res) => {
            CW721_ADDRESS.save(deps.storage, &Addr::unchecked(res.contract_address))?;
            Ok(Response::default().add_attribute("action", "instantiate_cw721_reply"))
        }
        Err(_) => Err(ContractError::InstantiateCW721Error {}),
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{MOCK_CONTRACT_ADDR, mock_dependencies_with_balance, mock_env, mock_info};
    use cosmwasm_std::{coins};

    #[test]
    fn initialization() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg {
            base_token_uri: String::from("https://ipfs.io/ipfs/kaka"),
            num_tokens: 20,
            cw721_code_id: 10u64,
            cw721_instantiate_msg: Cw721InstantiateMsg {
                name: String::from("ARTAVERSER"),
                symbol: String::from("ATA"),
                minter: Addr::unchecked(MOCK_CONTRACT_ADDR).to_string(),
            },
        };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();
        println!("{:?}", res);
        assert_eq!(
            res.messages,
            vec![SubMsg {
                msg: WasmMsg::Instantiate {
                    code_id: msg.cw721_code_id,
                    msg: to_binary(&Cw721InstantiateMsg {
                        name: msg.cw721_instantiate_msg.name.clone(),
                        symbol: msg.cw721_instantiate_msg.symbol.clone(),
                        minter: MOCK_CONTRACT_ADDR.to_string(),
                    }).unwrap(),
                    funds: info.funds.clone(),
                    admin: Some(info.sender.to_string()),
                    label: String::from("Check CW721"),
                }.into(),
                id: INSTANTIATE_CW721_REPLY_ID,
                gas_limit: None,
                reply_on: ReplyOn::Success,
            }]
        );
    }
}
