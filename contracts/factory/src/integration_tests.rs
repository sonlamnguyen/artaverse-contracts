#[cfg(test)]
mod tests {
    use crate::helpers::CwTemplateContract;
    // use crate::msg::InstantiateMsg;
    use cosmwasm_std::{Addr, Coin, ContractResult, Empty, Reply, SubMsgExecutionResponse, Uint128};
    use cosmwasm_std::testing::mock_env;
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};
    use minter::contract::reply;
    use minter::msg::{InstantiateMsg as MinterInstantiateMsg};

    const INSTANTIATE_TOKEN_REPLY_ID: u64 = 1;
    // pub fn contract_template() -> Box<dyn Contract<Empty>> {
    //     let contract = ContractWrapper::new(
    //         crate::contract::execute,
    //         crate::contract::instantiate,
    //         crate::contract::query,
    //     );
    //     Box::new(contract)
    // }

    pub fn contract_minter() -> Box<dyn Contract<cosmwasm_std::Empty> + 'static> {
        let contract = ContractWrapper::new(
            minter::contract::execute,
            minter::contract::instantiate,
            minter::contract::query,
        );
        Box::new(contract)
    }

    // pub fn contract_cw721() -> Box<dyn Contract<dyn Contract<Empty>>> {
    //     let contract = ContractWrapper::new(
    //         cw721_base::execute,
    //         cw721_base::instantiate,
    //         cw721_base::query,
    //     );
    //     Box::new(contract)
    // }

    const USER: &str = "USER";
    const ADMIN: &str = "ADMIN";
    const NATIVE_DENOM: &str = "denom";

    fn mock_app() -> App {
        AppBuilder::new().build(|router, _, storage| {
            router
                .bank
                .init_balance(
                    storage,
                    &Addr::unchecked(USER),
                    vec![Coin {
                        denom: NATIVE_DENOM.to_string(),
                        amount: Uint128::new(1),
                    }],
                )
                .unwrap();
        })
    }

    fn proper_instantiate() -> (App, CwTemplateContract) {
        let mut app = mock_app();
        let cw_minter_id = app.store_code(contract_minter());
        // let cw_721_id = app.store_code(contract_cw721());

        let msg = MinterInstantiateMsg {
            base_token_uri: "http://ipfs/link".parse().unwrap(),
            num_tokens: 100,
            max_tokens_per_batch_mint: 10,
            cw721_code_id: 8,
            name: "LOVENFT".to_string(),
            symbol: "LOV".to_string(),
            royalty_percentage: Some(10),
            royalty_payment_address: Option::from("address".to_string()),
        };
        let cw_minter_contract_addr = app
            .instantiate_contract(
                cw_minter_id,
                Addr::unchecked(ADMIN),
                &msg,
                &[],
                "test",
                None,
            )
            .unwrap();

        let cw_template_contract = CwTemplateContract(cw_minter_contract_addr);

        (app, cw_template_contract)
    }

    mod count {
        use super::*;
        // use crate::msg::ExecuteMsg;

        #[test]
        fn count() {
            let (mut app, cw_template_contract) = proper_instantiate();

            // let msg = ExecuteMsg::Increment {};
            // let cosmos_msg = cw_template_contract.call(msg).unwrap();
            // app.execute(Addr::unchecked(USER), cosmos_msg).unwrap();
        }
    }
}
