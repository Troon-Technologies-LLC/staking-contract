use ft_stkn_contract::ContractContract as FungibleContract;
use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use near_sdk_sim::{deploy, init_simulator, to_yocto, ContractAccount, UserAccount};
use staking_stkn_contract::ContractContract;
use staking_stkn_contract::{APY, FT};

near_sdk_sim::lazy_static_include::lazy_static_include_bytes! {
    // update `contract.wasm` for your contract's name
    CONTRACT_WASM_BYTES => "target/wasm32-unknown-unknown/release/ft_stkn_contract.wasm",
    STAKING_WASM_BYTES => "target/wasm32-unknown-unknown/release/staking_stkn_contract.wasm",

    // CONTRACT_WASM_BYTES => "res/ft_stkn_contract.wasm",
    // STAKING_WASM_BYTES => "res/staking_stkn_contract.wasm",

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
            apy_key: "stknPremium6".to_string(),
            interest_rate: 1000,
            min_staking_amt: U128::from(5000000000000000000000000000),
            min_duration: 6,
            plan_name: "Premium".to_string(),
        },
        APY {
            apy_key: "stknPremium12".to_string(),
            interest_rate: 2000,
            min_staking_amt: U128::from(5000000000000000000000000000),
            min_duration: 12,
            plan_name: "Premium".to_string(),
        },
    ]
    .to_vec();
    let approved_fts: Vec<FT> = [FT {
        account_id: ft.account_id(),
        symbol: "stkn".to_string(),
        apy_against_duration: None,
    }]
    .to_vec();
    let staking = deploy!(
        contract: ContractContract,
        contract_id: ST_ID,
        bytes: &STAKING_WASM_BYTES,
        signer_account: alice,
        init_method: new(root.account_id(),approved_fts,apy_data)
        // init_method: new(root.account_id(),[{ft.account_id(), "stkn".to_string(),None}].to_vec(),apy_data)
    );

    (root, ft, staking, alice)
}
