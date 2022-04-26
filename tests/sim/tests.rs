use near_sdk::json_types::U128;
use near_sdk::serde_json::json;
use near_sdk_sim::{call, to_yocto, view, DEFAULT_GAS};
// use staking_bkrt_contract::Stake;
use std::{thread, time};

use crate::utils::{init, register_user};

#[test]
fn simulate_total_supply() {
    let initial_balance = to_yocto("100");

    let (_, ftt, _, _) = init(initial_balance);

    let total_supply: U128 = view!(ftt.ft_total_supply()).unwrap_json();
    assert_eq!(initial_balance, total_supply.0);
}
#[test]
fn simulate_token_transfer() {
    let amount = to_yocto("2000");
    let initial_balance = to_yocto("1000000");
    let (root, ft, _, alice) = init(initial_balance);
    //===> With Macro<========//
    call!(
        root,
        ft.ft_transfer(alice.account_id(), amount.into(), None),
        deposit = 1
    )
    .assert_success();

    let root_balance: U128 = view!(ft.ft_balance_of(root.account_id())).unwrap_json();
    println!("root balance {:?}", root_balance);
    let alice_balance: U128 = view!(ft.ft_balance_of(alice.account_id())).unwrap_json();
    println!("alice balance {:?}", alice_balance);
    assert_eq!(initial_balance - amount, root_balance.0);
}

#[test]
pub fn stimulate_staking_fungible_tokens() {
    let amount = to_yocto("6000");
    let initial_balance = to_yocto("6000");
    let (root, ft, staking, alice) = init(initial_balance);

    register_user(&staking.user_account);
    // let root_balance: U128 = view!(ft.ft_balance_of(root.account_id())).unwrap_json();
    // println!("Root account balance {:?}", root_balance);
    //===> With Macro<========//
    // call!(root,ft.ft_transfer_call(staking.account_id(),amount.into(),None,"{\"ft_symbol\":\"BKRT\",\"ft_account_id\":\"ft\",\"decimal\":24,\"duration\":15778800,\"staked_by\":\"alice\",\"staking_plan\":\"BKRTPremium6\"}".to_string()),
    // deposit =1).assert_success();
    //===> Without Macro<========//
    let outcome = root
        .create_transaction(ft.account_id())
        .function_call(
            "ft_transfer_call".to_string(),
            json!({
                "receiver_id": staking.account_id(),
                "amount": amount.to_string(),
                "msg": "{\"ft_symbol\":\"BKRT\",\"ft_account_id\":\"root\",\"staked_by\":\"root\",\"decimal\":24,\"duration\":145}"
              })
            .to_string()
            .into_bytes(),
            DEFAULT_GAS / 2,
            1,
        )
    .function_call(
        "storage_unregister".to_string(),
        json!({"force":true}).to_string().into_bytes(),
        DEFAULT_GAS / 2,
        1,
    )
    .submit();
    println!(" outcome.....{:?}", outcome.receipt_ids());

    let root_balance: U128 = view!(ft.ft_balance_of(root.account_id())).unwrap_json();
    let staking_balance: U128 = view!(ft.ft_balance_of(staking.account_id())).unwrap_json();
    println!("staking_balance {:?}", staking_balance);

    assert_eq!(initial_balance - amount, root_balance.0);
    assert_eq!(amount, staking_balance.0);
}

#[test]
pub fn stimulate_get_staking_history() {
    let amount = to_yocto("6000");
    let initial_balance = to_yocto("6000");
    let (root, ft, staking, alice) = init(initial_balance);

    register_user(&staking.user_account);
    let root_balance: U128 = view!(ft.ft_balance_of(root.account_id())).unwrap_json();
    println!("Root account balance {:?}", root_balance);
    //===>With Macro<========//
    call!(root,ft.ft_transfer_call(staking.account_id(),amount.into(),None,"{\"ft_symbol\":\"BKRT\",\"ft_account_id\":\"ft\",\"decimal\":24,\"duration\":15778800,\"staked_by\":\"alice\",\"staking_plan\":\"BKRTPremium6\"}".to_string()), deposit=1).assert_success();
    //===> Without Macro<========//
    // let outcome = root
    //     .create_transaction(ft.account_id())
    //     .function_call(
    //         "ft_transfer_call".to_string(),
    //         json!({
    //             "receiver_id": staking.account_id(),
    //             "amount": amount.to_string(),
    //             "msg": "{\"ft_symbol\":\"BKRT\",\"ft_account_id\":\"ft\",\"staked_by\":\"root\",\"decimal\":24,\"duration\":15778800}"
    //           })
    //         .to_string()
    //         .into_bytes(),
    //         DEFAULT_GAS / 2,
    //         1,
    //     )
    // .function_call(
    //     "storage_unregister".to_string(),
    //     json!({"force":true}).to_string().into_bytes(),
    //     DEFAULT_GAS / 2,
    //     1,
    // )
    // .submit();
    // // println!(" outcome.....{:?}", outcome.receipt_ids());

    let root_balance: U128 = view!(ft.ft_balance_of(root.account_id())).unwrap_json();
    let staking_balance: U128 = view!(ft.ft_balance_of(staking.account_id())).unwrap_json();
    println!("staking_balance {:?}", staking_balance);

    assert_eq!(initial_balance - amount, root_balance.0);
    assert_eq!(amount, staking_balance.0);

    let staking_history =
        view!(staking.get_staking_history(root.account_id(), None, None)).unwrap_json_value();
    // let staking_history = view!(staking.get_staking_history(root.account_id(), None, None));

    println!("stake history = {:?}", staking_history);
}

#[test]
// #[ignore = "u128 is not supported"]
pub fn stimulate_unstake_fungible_token() {
    let amount = to_yocto("6000");
    let initial_balance = to_yocto("6000");
    let (root, ft, staking, alice) = init(initial_balance);

    register_user(&staking.user_account);
    let root_balance: U128 = view!(ft.ft_balance_of(root.account_id())).unwrap_json();
    println!("Root account balance {:?}", root_balance);
    call!(
        root,
        ft.ft_transfer(alice.account_id(), to_yocto("6000").into(), None),
        deposit = 1
    )
    .assert_success();
    let _alice_balance: U128 = view!(ft.ft_balance_of(alice.account_id())).unwrap_json();
    println!("Alice balance from root = {:?}", _alice_balance);
    call!(alice,ft.ft_transfer_call(staking.account_id(),amount.into(),None,"{\"ft_symbol\":\"BKRT\",\"ft_account_id\":\"ft\",\"decimal\":24,\"duration\":15778800,\"staked_by\":\"alice\",\"staking_plan\":\"BKRTPremium6\"}".to_string()),
    deposit =1).assert_success();

    let _alice_balance: U128 = view!(ft.ft_balance_of(alice.account_id())).unwrap_json();
    println!("Alice balance after stake = {:?}", _alice_balance);

    let root_balance: U128 = view!(ft.ft_balance_of(root.account_id())).unwrap_json();
    println!("root balance  {:?}", root_balance);
    let staking_balance: U128 = view!(ft.ft_balance_of(staking.account_id())).unwrap_json();
    println!("staking_balance {:?}", staking_balance);

    assert_eq!(initial_balance - amount, root_balance.0);
    assert_eq!(amount, staking_balance.0);

    let ten_millis = time::Duration::from_secs(10);
    // let num: U128 = "1".to_string();
    thread::sleep(ten_millis);
    let id: U128 = U128::from(1);
    call!(alice, staking.ft_unstake(id)).assert_success();

    let _alice_balance: U128 = view!(ft.ft_balance_of(alice.account_id())).unwrap_json();
    // println!("Alice balance After Unstake = {:?}", _alice_balance);

    // let staking_balance: U128 = view!(ft.ft_balance_of(staking.account_id())).unwrap_json();
    // println!("staking_balance {:?}", staking_balance);

    // let root_balance: U128 = view!(ft.ft_balance_of(root.account_id())).unwrap_json();

    assert_eq!(amount, _alice_balance.0);
}
#[test]
pub fn stimulate_claim_reward() {
    let amount = to_yocto("6000");
    let initial_balance = to_yocto("6000");
    let (root, ft, staking, alice) = init(initial_balance);

    register_user(&staking.user_account);
    let root_balance: U128 = view!(ft.ft_balance_of(root.account_id())).unwrap_json();
    println!("Root account balance {:?}", root_balance);
    call!(
        root,
        ft.ft_transfer(alice.account_id(), to_yocto("6000").into(), None),
        deposit = 1
    )
    .assert_success();
    let _alice_balance: U128 = view!(ft.ft_balance_of(alice.account_id())).unwrap_json();
    println!("Alice balance from root = {:?}", _alice_balance);
    call!(alice,ft.ft_transfer_call(staking.account_id(),amount.into(),None,"{\"ft_symbol\":\"BKRT\",\"ft_account_id\":\"ft\",\"decimal\":24,\"duration\":15778800,\"staked_by\":\"alice\",\"staking_plan\":\"BKRTPremium6\"}".to_string()),
    deposit =1).assert_success();

    let _alice_balance: U128 = view!(ft.ft_balance_of(alice.account_id())).unwrap_json();
    println!("Alice balance after stake = {:?}", _alice_balance);

    let root_balance: U128 = view!(ft.ft_balance_of(root.account_id())).unwrap_json();
    println!("root balance  {:?}", root_balance);
    let staking_balance: U128 = view!(ft.ft_balance_of(staking.account_id())).unwrap_json();
    println!("staking_balance {:?}", staking_balance);

    assert_eq!(initial_balance - amount, root_balance.0);
    assert_eq!(amount, staking_balance.0);

    let ten_millis = time::Duration::from_secs(10);
    // let num: U128 = "1".to_string();
    thread::sleep(ten_millis);
    let id: U128 = U128::from(1);
    // call!(alice, staking.ft_unstake(id)).assert_success();

    let _alice_balance: U128 = view!(ft.ft_balance_of(alice.account_id())).unwrap_json();
    println!("Alice balance After Unstake = {:?}", _alice_balance);

    // let staking_balance: U128 = view!(ft.ft_balance_of(staking.account_id())).unwrap_json();
    // println!("staking_balance {:?}", staking_balance);

    // let root_balance: U128 = view!(ft.ft_balance_of(root.account_id())).unwrap_json();

    // assert_eq!(amount, _alice_balance.0);
    let id: U128 = U128::from(1);
    call!(alice, staking.claim_reward(id)).assert_success();

    let _alice_balance: U128 = view!(ft.ft_balance_of(alice.account_id())).unwrap_json();
    println!("Alice balance After Unstake = {:?}", _alice_balance);
}
#[test]
#[should_panic]
pub fn stimulate_staking_fungible_tokens_should_fail_with_less_than_4000_tokens() {
    let amount = to_yocto("3000");
    let initial_balance = to_yocto("3000");
    let (root, ft, staking, alice) = init(initial_balance);

    register_user(&staking.user_account);
    // let root_balance: U128 = view!(ft.ft_balance_of(root.account_id())).unwrap_json();
    // println!("Root account balance {:?}", root_balance);
    //===> With Macro<========//
    call!(root,ft.ft_transfer_call(staking.account_id(),amount.into(),None,"{\"ft_symbol\":\"BKRT\",\"ft_account_id\":\"ft\",\"decimal\":24,\"duration\":15778800,\"staked_by\":\"alice\",\"staking_plan\":\"BKRTPremium6\"}".to_string()),
    deposit =1).assert_success();

    let root_balance: U128 = view!(ft.ft_balance_of(root.account_id())).unwrap_json();
    let staking_balance: U128 = view!(ft.ft_balance_of(staking.account_id())).unwrap_json();
    println!("staking_balance {:?}", staking_balance);

    assert_eq!(initial_balance - amount, root_balance.0);
    assert_eq!(amount, staking_balance.0);
}

#[test]
// #[should_panic]
pub fn stimulate_staking_fungible_tokens_should_fail_only_approved_FTs_can_be_staked() {
    let amount = to_yocto("6000");
    let initial_balance = to_yocto("6000");
    let (root, ftt, staking, alice) = init(initial_balance);

    register_user(&staking.user_account);
    // let root_balance: U128 = view!(ft.ft_balance_of(root.account_id())).unwrap_json();
    // println!("Root account balance {:?}", root_balance);
    //===> With Macro<========//
    let c=call!(root,ftt.ft_transfer_call(staking.account_id(),amount.into(),None,"{\"ft_symbol\":\"BKRT\",\"ft_account_id\":\"ft\",\"decimal\":24,\"duration\":15778800,\"staked_by\":\"root\",\"staking_plan\":\"BKRTPremium6\"}".to_string()),
    deposit =1);
    println!("{:?}", c);
    assert!(c.is_ok());

    let root_balance: U128 = view!(ftt.ft_balance_of(root.account_id())).unwrap_json();
    let staking_balance: U128 = view!(ftt.ft_balance_of(staking.account_id())).unwrap_json();
    println!("staking_balance {:?}", staking_balance);

    // assert_eq!(initial_balance - amount, root_balance.0);
    // assert_eq!(amount, staking_balance.0);
}
