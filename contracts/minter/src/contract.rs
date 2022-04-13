#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, StdResult, WasmMsg, ReplyOn, Reply, Addr, Response, SubMsg, Order, Empty, CosmosMsg};
use cw2::set_contract_version;
use cw721_base::{InstantiateMsg as Cw721InstantiateMsg, MintMsg, ExecuteMsg as Cw721ExecuteMsg};
use cw_utils::parse_reply_instantiate_data;

use crate::error::ContractError;
use crate::{Extension, Metadata, JsonSchema};
use crate::msg::{ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG, MINTABLE_TOKEN_IDS, MINTABLE_NUM_TOKENS, CW721_ADDRESS};
use crate::{Serialize, Deserialize};

pub type Cw721ArtaverseContract<'a> = cw721_base::Cw721Contract<'a, Extension, Empty>;
// pub type ExecuteMsg = cw721_base::ExecuteMsg<Extension>;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:artaverse-contracts";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// governance parameters
const MAX_TOKEN_LIMIT: u32 = 10000;
const MAX_TOKEN_PER_BATCH_LIMIT: u32 = 10000;
const INSTANTIATE_CW721_REPLY_ID: u64 = 1;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct TokensResponse {
    /// Contains all token_ids in lexicographical ordering
    /// If there are more than `limit`, use `start_from` in future queries
    /// to achieve pagination.
    pub tokens: Vec<String>,
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
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

    // Check the number of tokens per batch is more than zero and less than the max limit
    if msg.max_tokens_per_batch_mint == 0 || msg.max_tokens_per_batch_mint > MAX_TOKEN_PER_BATCH_LIMIT {
        return Err(ContractError::InvalidNumTokens {
            min: 1,
            max: MAX_TOKEN_PER_BATCH_LIMIT,
        });
    }

    let config = Config {
        owner: info.sender.clone(),
        cw721_code_id: msg.cw721_code_id,
        cw721_address: None,
        name: msg.name.clone(),
        symbol: msg.symbol.clone(),
        base_token_uri: msg.base_token_uri.clone(),
        max_tokens: msg.num_tokens,
        max_tokens_per_batch_mint: msg.max_tokens_per_batch_mint,
        royalty_percentage: msg.royalty_percentage,
        royalty_payment_address: msg.royalty_payment_address,
    };
    CONFIG.save(deps.storage, &config)?;
    MINTABLE_NUM_TOKENS.save(deps.storage, &msg.num_tokens)?;

    // Save mintable token ids map
    for token_id in 1..=msg.num_tokens {
        MINTABLE_TOKEN_IDS.save(deps.storage, token_id, &true)?;
    }

    // Sub-message to instantiate cw721 contract
    let sub_msgs: Vec<SubMsg> = vec![SubMsg {
        id: INSTANTIATE_CW721_REPLY_ID,
        msg: WasmMsg::Instantiate {
            admin: Some(info.sender.to_string()),
            code_id: msg.cw721_code_id,
            msg: to_binary(&Cw721InstantiateMsg {
                name: msg.name,
                symbol: msg.symbol,
                minter: env.contract.address.to_string(),
            })?,
            funds: vec![],
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
    // )
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
        ExecuteMsg::BatchMint { token_ids } => execute_batch_mint_sender(deps, info, token_ids),
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

pub fn execute_batch_mint_sender(
    deps: DepsMut,
    info: MessageInfo,
    token_ids: Vec<u32>,
) -> Result<Response, ContractError> {
    let recipient = info.sender.clone();
    _execute_batch_mint(deps, info, Some(recipient), token_ids)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => to_binary(&query_config(deps)?),
        _ => Cw721ArtaverseContract::default().query(deps, env, msg.into()),
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {
        owner: config.owner,
        cw721_code_id: config.cw721_code_id,
        cw721_address: config.cw721_address,
        max_tokens: config.max_tokens,
        max_tokens_per_mint: config.max_tokens_per_batch_mint,
        name: config.name,
        symbol: config.symbol,
        base_token_uri: config.base_token_uri,
        extension: Some(Metadata {
            royalty_percentage: config.royalty_percentage,
            royalty_payment_address: config.royalty_payment_address,
            ..Metadata::default()
        }),
    })
}


fn _execute_mint(
    deps: DepsMut,
    info: MessageInfo,
    recipient: Option<Addr>,
    token_id: Option<u32>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    let recipient_addr = match recipient {
        Some(some_recipient) => some_recipient,
        None => info.sender.clone(),
    };

    let mintable_token_id = match token_id {
        Some(token_id) => {
            if token_id == 0 || token_id > config.max_tokens {
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
    let msg = _create_cw721_mint(&info, &config, &recipient_addr, mintable_token_id);
    let msg_rs = match msg {
        Ok(msg) => { msg }
        Err(ctr_err) => return Err(ctr_err),
    };
    msgs.append(&mut vec![msg_rs]);

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

fn _execute_batch_mint(
    deps: DepsMut,
    info: MessageInfo,
    recipient: Option<Addr>,
    mut batch_token_ids: Vec<u32>,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    let recipient_addr = match recipient {
        Some(some_recipient) => some_recipient,
        None => info.sender.clone(),
    };
    let mut count: u32 = 0;
    let mut minted_token_ids: Vec<u32> = vec![];
    let mut msgs: Vec<CosmosMsg<Empty>> = vec![];
    while let Some(token_id) = batch_token_ids.pop() {
        if count >= config.max_tokens_per_batch_mint { break; }

        if token_id == 0 || token_id > config.max_tokens {
            return Err(ContractError::InvalidTokenId {});
        }
        // If token_id not on mintable map, throw err
        if !MINTABLE_TOKEN_IDS.has(deps.storage, token_id) {
            return Err(ContractError::TokenIdAlreadySold { token_id });
        }

        let msg = _create_cw721_mint(&info, &config, &recipient_addr, token_id);
        let msg_rs = match msg {
            Ok(msg) => { msg }
            Err(ctr_err) => return Err(ctr_err),
        };
        msgs.append(&mut vec![msg_rs]);

        // Remove mintable token id from map
        MINTABLE_TOKEN_IDS.remove(deps.storage, token_id);
        let mintable_num_tokens = MINTABLE_NUM_TOKENS.load(deps.storage)?;
        // Decrement mintable num tokens
        MINTABLE_NUM_TOKENS.save(deps.storage, &(mintable_num_tokens - 1))?;

        minted_token_ids.append(&mut vec![token_id]);
        count += 1;
    }
    let minted_token_ids_str = format!("{:?}", minted_token_ids);
    Ok(Response::new()
        .add_attribute("sender", info.sender)
        .add_attribute("recipient", recipient_addr)
        .add_attribute("token_id", minted_token_ids_str)
        .add_messages(msgs))
}

fn _create_cw721_mint<'a>(
    info: &'a MessageInfo,
    config: &'a Config,
    recipient_addr: &'a Addr,
    mintable_token_id: u32,
) -> Result<CosmosMsg, ContractError> {
    let mint_msg = Cw721ExecuteMsg::Mint(MintMsg::<Extension> {
        token_id: mintable_token_id.to_string(),
        owner: recipient_addr.to_string(),
        token_uri: Some(format!("{}/{}", config.base_token_uri, mintable_token_id.to_string())),
        extension: Some(Metadata {
            royalty_percentage: config.royalty_percentage,
            royalty_payment_address: config.royalty_payment_address.clone(),
            ..Metadata::default()
        }),
    });
    let msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.cw721_address.as_ref().unwrap().to_string(),
        msg: to_binary(&mint_msg)?,
        funds: info.funds.clone(),
    });
    Ok(msg)
}

// Reply callback triggered from cw721 contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    let mut config: Config = CONFIG.load(deps.storage)?;
    if msg.id != INSTANTIATE_CW721_REPLY_ID {
        return Err(ContractError::InvalidReplyID {});
    }

    let reply = parse_reply_instantiate_data(msg);
    match reply {
        Ok(res) => {
            config.cw721_address = Addr::unchecked(res.contract_address.clone()).into();
            CONFIG.save(deps.storage, &config)?;
            CW721_ADDRESS.save(deps.storage, &Addr::unchecked(res.contract_address))?;
            Ok(Response::default().add_attribute("action", "instantiate_cw721_reply"))
        }
        Err(_) => Err(ContractError::InstantiateCW721Error {}),
    }
}

#[cfg(test)]
mod tests {
    use std::ptr::null;
    use super::*;
    use cosmwasm_std::testing::{MOCK_CONTRACT_ADDR, mock_dependencies_with_balance, mock_env, mock_info};
    use cosmwasm_std::{coins, SubMsgExecutionResponse, SubMsgResult, from_binary, Decimal};
    use cw721_base::state::TokenInfo;
    use prost::Message;
    use crate::msg::ExecuteMsg::{BatchMint, Mint};

    // Type for replies to contract instantiate messes
    #[derive(Clone, PartialEq, Message)]
    struct MsgInstantiateContractResponse {
        #[prost(string, tag = "1")]
        pub contract_address: ::prost::alloc::string::String,
        #[prost(bytes, tag = "2")]
        pub data: ::prost::alloc::vec::Vec<u8>,
    }

    #[test]
    fn initialization() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg {
            base_token_uri: String::from("https://ipfs.io/ipfs/kaka"),
            num_tokens: 20,
            max_tokens_per_batch_mint: 10,
            cw721_code_id: 10u64,
            name: String::from("ARTAVERSER"),
            symbol: String::from("ATA"),
            royalty_percentage: None,
            royalty_payment_address: None,
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
                        name: msg.name.clone(),
                        symbol: msg.symbol.clone(),
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
        let query_msg = QueryMsg::GetConfig {};
        let res = query(deps.as_ref(), mock_env(), query_msg).unwrap();
        let config: ConfigResponse = from_binary(&res).unwrap();

        println!("ConfigResponse {:?}", config);
    }

    #[test]
    fn mint() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
        let info = mock_info("creator", &coins(1000, "earth"));

        let msg = InstantiateMsg {
            base_token_uri: String::from("https://ipfs.io/ipfs/kaka"),
            num_tokens: 20,
            max_tokens_per_batch_mint: 10,
            cw721_code_id: 10u64,
            name: String::from("ARTAVERSER"),
            symbol: String::from("ATA"),
            royalty_percentage: Some(12u64),
            royalty_payment_address: Some(String::from("aa")),
        };

        // we can just call .unwrap() to assert this was a success
        let res_instantiate = instantiate(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();
        println!("res_instantiate {:?}", res_instantiate);

        let instantiate_reply = MsgInstantiateContractResponse {
            contract_address: "nftcontract721".to_string(),
            data: vec![2u8; 32769],
        };

        let mut encoded_instantiate_reply =
            Vec::<u8>::with_capacity(instantiate_reply.encoded_len() as usize);
        instantiate_reply
            .encode(&mut encoded_instantiate_reply)
            .unwrap();

        let reply_msg = Reply {
            id: INSTANTIATE_CW721_REPLY_ID,
            result: SubMsgResult::Ok(SubMsgExecutionResponse {
                events: vec![],
                data: Some(encoded_instantiate_reply.into()),
            }),
        };
        reply(deps.as_mut(), mock_env(), reply_msg).unwrap();

        let query_msg = QueryMsg::GetConfig {};
        let res = query(deps.as_ref(), mock_env(), query_msg).unwrap();
        let config: ConfigResponse = from_binary(&res).unwrap();

        println!("ConfigResponse {:?}", config);

        // call mint NFT
        let msg_mint = Mint {
            token_id: 1
        };

        let res_execute = execute(deps.as_mut(), mock_env(), info, msg_mint).unwrap();
        println!("res_execute {:?}", res_execute);

        // call batch mint NFT
        // let msg_mint = BatchMint {
        //     token_ids: vec![1,2,3,4,5,6,7,8,9,10,11,12,13]
        //     // token_ids: vec![1,2]
        // };
        //
        // let res_execute = execute(deps.as_mut(), mock_env(), info, msg_mint).unwrap();
        // println!("res_execute_batch {:?}", res_execute);
        //
        // let token_id = 2
        //     .to_string();
        let query_msg = QueryMsg::AllTokens { start_after: None, limit: None };
        let res: TokensResponse = from_binary(&query(deps.as_ref(), mock_env(), query_msg).unwrap()).unwrap();
        // let config: TokenInfo<Extension> = from_binary(&rex).unwrap();

        println!("ConfigResponse {:?}", res);

        // let contract = setup_contract(deps.as_mut());
        // let token_id = "2".to_string();
        // // let info_msg = cw721_base::msg::QueryMsg::NftInfo {token_id: token_id.clone()};
        // // let contract = Cw721ArtaverseContract::default();
        // // let info = contract.query(deps.as_ref(),mock_env(),info_msg).unwrap();
        // // println!("info {:?}", info);
        //
        // let contract = Cw721ArtaverseContract::default();
        // let token_info = contract.tokens.load(deps.as_ref().storage, &token_id).unwrap();
        //
        // let royalty_percentage = match token_info.extension {
        //     Some(ref ext) => match ext.royalty_percentage {
        //         Some(percentage) => Decimal::percent(percentage),
        //         None => Decimal::percent(0),
        //     },
        //     None => Decimal::percent(0),
        // };
    }
}
