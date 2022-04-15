use cw721_base::{InstantiateMsg as Cw721InstantiateMsg};
use crate::contract::{instantiate, execute, query, reply};
use crate::contract::{MAX_TOKEN_LIMIT, MAX_TOKEN_PER_BATCH_LIMIT, INSTANTIATE_CW721_REPLY_ID};
use crate::msg::{ConfigResponse, InstantiateMsg, QueryMsg};

#[cfg(test)]
mod tests {
    // use std::ptr::null;
    use super::*;
    use cosmwasm_std::testing::{MOCK_CONTRACT_ADDR, mock_dependencies_with_balance, mock_env, mock_info};
    use cosmwasm_std::{coins, SubMsgExecutionResponse, SubMsgResult, from_binary, SubMsg, WasmMsg, to_binary, ReplyOn, Reply, Addr};

    use prost::Message;
    use crate::{Metadata};
    use crate::error::ContractError;
    use crate::msg::ExecuteMsg::{BatchMint, Mint, MintTo};

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
        let info = mock_info("creator", &coins(1000, "earth"));
        // num_token is zero returns error
        let msg = InstantiateMsg {
            base_token_uri: String::from("ipfs://Fhgihgkdfhgdfgdgdfgdfhfvbnykorkjojroiwoiwmgdmg"),
            num_tokens: 0,
            max_tokens_per_batch_mint: 10,
            cw721_code_id: 10u64,
            name: String::from("ARTAVERSER"),
            symbol: String::from("ATA"),
            royalty_percentage: None,
            royalty_payment_address: None,
        };
        instantiate(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap_err();

        // num_token is over max token limit return error
        let msg = InstantiateMsg {
            base_token_uri: String::from("ipfs://Fhgihgkdfhgdfgdgdfgdfhfvbnykorkjojroiwoiwmgdmg"),
            num_tokens: MAX_TOKEN_LIMIT + 1,
            max_tokens_per_batch_mint: 10,
            cw721_code_id: 10u64,
            name: String::from("ARTAVERSER"),
            symbol: String::from("ATA"),
            royalty_percentage: None,
            royalty_payment_address: None,
        };
        instantiate(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap_err();

        // num_token is over max token limit return error
        let msg = InstantiateMsg {
            base_token_uri: String::from("ipfs://Fhgihgkdfhgdfgdgdfgdfhfvbnykorkjojroiwoiwmgdmg"),
            num_tokens: 20,
            max_tokens_per_batch_mint: MAX_TOKEN_PER_BATCH_LIMIT + 1,
            cw721_code_id: 10u64,
            name: String::from("ARTAVERSER"),
            symbol: String::from("ATA"),
            royalty_percentage: None,
            royalty_payment_address: None,
        };
        instantiate(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap_err();

        // max_tokens_per_batch_mint is zero returns error
        let msg = InstantiateMsg {
            base_token_uri: String::from("ipfs://Fhgihgkdfhgdfgdgdfgdfhfvbnykorkjojroiwoiwmgdmg"),
            num_tokens: 20,
            max_tokens_per_batch_mint: 0,
            cw721_code_id: 10u64,
            name: String::from("ARTAVERSER"),
            symbol: String::from("ATA"),
            royalty_percentage: None,
            royalty_payment_address: None,
        };
        instantiate(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap_err();

        // Invalid uri returns error
        let msg = InstantiateMsg {
            base_token_uri: String::from("Fhgihgkdfhgdfgdgdfgdfhfvbnykorkjojroiwoiwmgdmg"),
            num_tokens: 20,
            max_tokens_per_batch_mint: 10,
            cw721_code_id: 10u64,
            name: String::from("ARTAVERSER"),
            symbol: String::from("ATA"),
            royalty_percentage: None,
            royalty_payment_address: None,
        };
        instantiate(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap_err();

        let msg = InstantiateMsg {
            base_token_uri: String::from("ipfs://Sdjbfsdkjfgbdkfjgbdsfgbkiufbguydfguybfsdfjkdnsk"),
            num_tokens: 20,
            max_tokens_per_batch_mint: 10,
            cw721_code_id: 10u64,
            name: String::from("ARTAVERSER"),
            symbol: String::from("ATA"),
            royalty_percentage: Option::from(10u64),
            royalty_payment_address: Option::from(String::from("creator_address")),
        };

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap();
        // println!("{:?}", res);
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
                    funds: vec![],
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
        assert_eq!(
            config,
            ConfigResponse {
                owner: info.sender.clone(),
                cw721_code_id: 10,
                cw721_address: None,
                max_tokens: 20,
                max_tokens_per_mint: 10,
                name: String::from("ARTAVERSER"),
                symbol: String::from("ATA"),
                base_token_uri: String::from("ipfs://Sdjbfsdkjfgbdkfjgbdsfgbkiufbguydfguybfsdfjkdnsk"),
                extension: Some(
                    Metadata
                    {
                        image: None,
                        image_data: None,
                        external_url: None,
                        description: None,
                        name: None,
                        attributes: None,
                        background_color: None,
                        animation_url: None,
                        youtube_url: None,
                        royalty_percentage: Some(10),
                        royalty_payment_address: Some(String::from("creator_address")),
                    }),
            }
        );
    }

    #[test]
    fn mint_test() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
        let info = mock_info("creator", &coins(1000, "earth"));
        let buyer = Addr::unchecked("buyer");
        let msg = InstantiateMsg {
            base_token_uri: String::from("ipfs://Sdjbfsdkjfgbdkfjgbdsfgbkiufbguydfguybfsdfjkdnsk"),
            num_tokens: 20,
            max_tokens_per_batch_mint: 10,
            cw721_code_id: 10u64,
            name: String::from("ARTAVERSER"),
            symbol: String::from("ATA"),
            royalty_percentage: Option::from(10u64),
            royalty_payment_address: Option::from(String::from("creator_address")),
        };
        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg.clone());
        assert!(res.is_ok());

        let query_msg = QueryMsg::GetConfig {};
        let res = query(deps.as_ref(), mock_env(), query_msg).unwrap();
        let config: ConfigResponse = from_binary(&res).unwrap();

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

        // call mint NFT
        let msg_mint = Mint {
            token_id: 1
        };
        let res_execute = execute(deps.as_mut(), mock_env(), info.clone(), msg_mint);
        assert!(res_execute.is_ok());

        // call batch mint NFT
        let msg_mint = BatchMint {
            token_ids: vec![2, 3, 4]
        };
        let res_execute = execute(deps.as_mut(), mock_env(), info.clone(), msg_mint);
        assert!(res_execute.is_ok());

        // call batch mintTo NFT
        let msg_mint = MintTo {
            token_id: 10,
            recipient: buyer.to_string(),
        };

        let res_execute = execute(deps.as_mut(), mock_env(), info.clone(), msg_mint);
        assert!(res_execute.is_ok());

        // token_id is zero returns error
        let msg_mint = BatchMint {
            token_ids: vec![5, 6, 0]
        };
        let err = execute(deps.as_mut(), mock_env(), info.clone(), msg_mint).unwrap_err();
        assert_eq!(err, ContractError::InvalidTokenId {});

        // token_id is over num tokens error
        let msg_mint = BatchMint {
            token_ids: vec![5, 6, config.max_tokens + 1]
        };
        let err = execute(deps.as_mut(), mock_env(), info.clone(), msg_mint).unwrap_err();
        assert_eq!(err, ContractError::InvalidTokenId {});

        //token_id not on mintable map
        let msg_mint = BatchMint {
            token_ids: vec![1, 6, 7]
        };
        let err = execute(deps.as_mut(), mock_env(), info.clone(), msg_mint).unwrap_err();
        assert_eq!(err, ContractError::TokenIdAlreadySold { token_id: 1 });
    }
}
