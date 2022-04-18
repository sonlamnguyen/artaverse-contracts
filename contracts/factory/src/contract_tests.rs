use crate::contract::{execute, instantiate, query};
use crate::msg::{InstantiateMsg, QueryMsg, StateResponse};

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    use crate::error::ContractError;
    use crate::msg::ExecuteMsg;

    #[test]
    fn initialization() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
        let info = mock_info("creator", &coins(1000, "earth"));

        // minter_code_id is zero error
        let msg = InstantiateMsg {
            minter_code_id: 0u64,
            cw721_code_id: 2u64,
        };

        // we can assert this was a success
        let err = instantiate(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap_err();
        assert_eq!(err, ContractError::InvalidID {});

        // cw721_code_id is zero error
        let msg = InstantiateMsg {
            minter_code_id: 1u64,
            cw721_code_id: 0u64,
        };

        // we can assert this was a success
        let err = instantiate(deps.as_mut(), mock_env(), info.clone(), msg.clone()).unwrap_err();
        assert_eq!(err, ContractError::InvalidID {});

        let msg = InstantiateMsg {
            minter_code_id: 1u64,
            cw721_code_id: 2u64,
        };

        // we can assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg.clone());
        assert!(res.is_ok());

        let query_msg = QueryMsg::GetState {};
        let res = query(deps.as_ref(), mock_env(), query_msg).unwrap();
        let config: StateResponse = from_binary(&res).unwrap();
        assert_eq!(
            config,
            StateResponse {
                minter_code_id: 1u64,
                cw721_code_id: 2u64,
            }
        );
    }

    #[test]
    fn update_minter_code_id() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
        let info = mock_info("creator", &coins(1000, "earth"));
        let msg = InstantiateMsg {
            minter_code_id: 1u64,
            cw721_code_id: 2u64,
        };

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg.clone());
        assert!(res.is_ok());

        let update_msg = ExecuteMsg::UpdateMinterCodeId { minter_code_id: 0 };
        let err = execute(deps.as_mut(), mock_env(), info.clone(), update_msg).unwrap_err();
        assert_eq!(err, ContractError::InvalidID {});

        let update_msg = ExecuteMsg::UpdateMinterCodeId { minter_code_id: 10 };
        let res_execute = execute(deps.as_mut(), mock_env(), info.clone(), update_msg);
        assert!(res_execute.is_ok());

        //check config value
        let query_msg = QueryMsg::GetState {};
        let res = query(deps.as_ref(), mock_env(), query_msg).unwrap();
        let config: StateResponse = from_binary(&res).unwrap();
        assert_eq!(
            config,
            StateResponse {
                minter_code_id: 10u64,
                cw721_code_id: 2u64,
            }
        );
    }

    #[test]
    fn update_cw721_code_id() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));
        let info = mock_info("creator", &coins(1000, "earth"));
        let msg = InstantiateMsg {
            minter_code_id: 1u64,
            cw721_code_id: 2u64,
        };

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg.clone());
        assert!(res.is_ok());

        let update_msg = ExecuteMsg::UpdateCw721CodeId { cw721_code_id: 0 };
        let err = execute(deps.as_mut(), mock_env(), info.clone(), update_msg).unwrap_err();
        assert_eq!(err, ContractError::InvalidID {});

        let update_msg = ExecuteMsg::UpdateCw721CodeId { cw721_code_id: 10 };
        let res_execute = execute(deps.as_mut(), mock_env(), info.clone(), update_msg);
        assert!(res_execute.is_ok());

        //check config value
        let query_msg = QueryMsg::GetState {};
        let res = query(deps.as_ref(), mock_env(), query_msg).unwrap();
        let config: StateResponse = from_binary(&res).unwrap();
        assert_eq!(
            config,
            StateResponse {
                minter_code_id: 1u64,
                cw721_code_id: 10u64,
            }
        );
    }
}
