use ft_bkrt_contract::ContractContract as FungibleContract;
// use near_contract_standards::fungible_token::*
use near_sdk::json_types::U128;
use near_sdk::near_bindgen;
use near_sdk::{serde_json::json, AccountId};
use near_sdk_sim::{
    deploy, init_simulator, to_yocto, ContractAccount, UserAccount, DEFAULT_GAS, STORAGE_AMOUNT,
};
use staking_bkrt_contract::ContractContract;
use staking_bkrt_contract::{APY, FT};
use std::collections::HashMap;
// pub type APYKey = String;
// #[near_bindgen]
// pub struct APY {
//     pub apy_key: APYKey,
//     pub interest_rate: u16, // interest Ex 1000 for 10%
//     pub min_staking_amt: U128,
//     pub min_duration: u8, //in months Ex 6
//     pub plan_name: String,
// }
// #[near_bindgen]
// pub struct FT {
//     account_id: AccountId,
//     symbol: String,
//     apy_against_duration: Option<HashMap<APYKey, APY>>,
// }

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    // update `contract.wasm` for your contract's name
    CONTRACT_WASM_BYTES => "target/wasm32-unknown-unknown/release/ft_bkrt_contract.wasm",
    STAKING_WASM_BYTES => "target/wasm32-unknown-unknown/release/staking_bkrt_contract.wasm",

}
const FT_ID: &str = "ft";
const ST_ID: &str = "staking";

pub fn register_user(user: &near_sdk_sim::UserAccount) {
    user.call(
        FT_ID.parse().unwrap(),
        "storage_deposit",
        &json!({
            "account_id": user.account_id()
        })
        .to_string()
        .into_bytes(),
        near_sdk_sim::DEFAULT_GAS / 2,
        near_sdk::env::storage_byte_cost() * 125, // attached deposit
    )
    .assert_success();
}

pub fn init(
    initial_balance: u128,
) -> (
    UserAccount,
    ContractAccount<FungibleContract>,
    ContractAccount<ContractContract>,
    UserAccount,
) {
    let root = init_simulator(None);

    let ft = deploy!(
        contract: FungibleContract,
        contract_id: FT_ID,
        bytes: &CONTRACT_WASM_BYTES,
        signer_account: root,
        init_method: new_default_meta(root.account_id(),initial_balance.into())
    );

    let alice = root.create_user("alice".parse().unwrap(), to_yocto("1000"));
    register_user(&alice);
    let apy_data: Vec<APY> = [
        APY {
            apy_key: "BKRTPremium6".to_string(),
            interest_rate: 1000,
            min_staking_amt: U128::from(5000000000000000000000000000),
            min_duration: 6,
            plan_name: "Premium".to_string(),
        },
        APY {
            apy_key: "BKRTPremium12".to_string(),
            interest_rate: 2000,
            min_staking_amt: U128::from(5000000000000000000000000000),
            min_duration: 12,
            plan_name: "Premium".to_string(),
        },
    ]
    .to_vec();
    let approved_fts: Vec<FT> = [FT {
        account_id: ft.account_id(),
        symbol: "BKRT".to_string(),
        apy_against_duration: None,
    }]
    .to_vec();
    let staking = deploy!(
        contract: ContractContract,
        contract_id: ST_ID,
        bytes: &STAKING_WASM_BYTES,
        signer_account: alice,
        init_method: new(root.account_id(),approved_fts,apy_data)
        // init_method: new(root.account_id(),[{ft.account_id(), "BKRT".to_string(),None}].to_vec(),apy_data)
    );

    (root, ft, staking, alice)
}
