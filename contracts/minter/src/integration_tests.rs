#[cfg(test)]
mod tests {
    use std::ptr::null;
    use crate::helpers::CwTemplateContract;
    use crate::msg::InstantiateMsg;
    use cosmwasm_std::{Addr, Coin, Empty, Uint128};
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};

    pub fn contract_template() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        );
        Box::new(contract)
    }

    pub fn contract_cw721() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        )
            .with_reply(crate::contract::reply);
        Box::new(contract)
    }

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

    fn proper_instantiate() -> (Addr) {
        let mut app = mock_app();
        let cw_template_id = app.store_code(contract_template());
        let cw_cw721_id = app.store_code(contract_cw721());

        let msg = InstantiateMsg {
            base_token_uri: String::from("https://ipfs.io/ipfs/kaka"),
            num_tokens: 100,
            max_tokens_per_batch_mint: 10,
            cw721_code_id: cw_cw721_id,
            name: String::from("ARTAVERSER"),
            symbol: String::from("ATA"),
            royalty_percentage: Option::from(10u64),
            royalty_payment_address: Option::from("xxxix".to_string()),
        };
        let cw_template_contract_addr = app
            .instantiate_contract(
                cw_template_id,
                Addr::unchecked(ADMIN),
                &msg,
                &[],
                "Minter",
                None,
            )
            .unwrap();

        // let cw_template_contract = CwTemplateContract(cw_template_contract_addr);

        // (app, cw_template_contract)
        (cw_template_contract_addr)
    }

    mod count {
        use super::*;
        use crate::msg::ExecuteMsg;

        #[test]
        fn count() {
            let mut app = mock_app();
            let (cw_template_contract) = proper_instantiate();
            const ADMIN_MINT_PRICE: u128 = 0;
            let msg = ExecuteMsg::Mint { token_id: 1 };
            let cosmos_msg = cw_template_contract.call(msg).unwrap();
            // app.execute_contract(Addr::unchecked(USER), cw_template_contract.clone(),).unwrap();

            // Creator mints an extra NFT for the buyer (who is a friend)
            let res = app.execute_contract(
                Addr::unchecked(USER),
                cw_template_contract.clone(),
                &msg,
                &coins_for_msg(Coin {
                    amount: Uint128::from(ADMIN_MINT_PRICE),
                    denom: NATIVE_DENOM.to_string(),
                }),
            );
        }
    }
}
