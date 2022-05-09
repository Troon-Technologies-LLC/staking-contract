use crate::*;

use near_sdk::json_types::U128;
use near_sdk::{ext_contract, log, Gas, PromiseOrValue, PromiseResult};

const BASE_GAS: Gas = Gas(5_000_000_000_000);

const THIRTY_DAYS: u64 = 2592000; //30 days in seconds
                                  //const THIRTY_DAYS: u64 = 2629800; //30days 10hours 30minutes

trait FTActionsReceiver {
    /*     fn staking_call(
        &mut self,
        ft_account_id: AccountId,
        amount: u128,
        duration: u64,
        symbol: String,
        decimal: u8,
        staker: AccountId,
    ); */

    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128>;

    fn ft_unstake(&mut self, stake_id: U128);

    fn did_promise_succeded() -> bool;

    fn unstake_callback(&mut self, stake_id: StakeId, staker_id: AccountId);

    fn claim_reward(&mut self, stake_id: StakeId);

    fn claim_reward_callback(
        &mut self,
        stake_id: StakeId,
        claim_history: Option<ClaimHistory>,
        claim_count: u64,
    );
}

#[ext_contract(ext_ft)]
trait FTCallbackReceiver {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);
}

#[ext_contract(this_contract)]
trait FTActionsSender {
    fn unstake_callback(&mut self, stake_id: StakeId, staker_id: AccountId);

    fn claim_reward_callback(
        &mut self,
        stake_id: StakeId,
        claim_history: Option<ClaimHistory>,
        claim_count: u64,
    );
}

#[near_bindgen]
impl FTActionsReceiver for Contract {
    /* fn staking_call(
        &mut self,
        ft_account_id: AccountId,
        amount: u128,
        duration: u64,
        symbol: String,
        decimal: u8,
        staker: AccountId,
    ) {
        //assert!(amount > 5000, "Cannot stake less than 5000 tokens");
        //let staker: AccountId = env::predecessor_account_id().try_into().unwrap();
        let staking_id = self
            .staking_nonce
            .checked_add(1)
            .expect("Exceeded u128 capacity");

        let staked_at = env::block_timestamp() / 1000000000;

        let stake = Stake {
            stake_id: staking_id,
            ft_symbol: symbol.clone(),
            ft_account_id: ft_account_id,
            decimal,
            amount,
            duration,
            staked_at: staked_at,
            staked_by
        };
        if let Some(mut staking_history) = self.amount_staked.get(&staker) {
            staking_history.push(stake);
            self.amount_staked.insert(&staker, &staking_history);
        } else {
            let mut staking_history: Vec<Stake> = Vec::new();
            staking_history.push(stake);
            self.amount_staked.insert(&staker, &staking_history);
        }

        log!("{} {} staked succssfully ", amount, symbol);
    } */

    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        let a: u128 = 0;
        let staking_id = self
            .staking_nonce
            .checked_add(1)
            .expect("Exceeded u128 capacity");

        log!("{} staking id", staking_id);
        let staked_at = env::block_timestamp() / 1000000000;
        let StakeArgs {
            ft_symbol,
            ft_account_id,
            decimal,
            duration,
            staked_by,
            staking_plan,
        } = near_sdk::serde_json::from_str(&msg).expect("Invalid Staking Arguments");

        let stake = Stake {
            stake_id: U128::from(staking_id),
            ft_symbol,
            ft_account_id,
            decimal,
            amount,
            duration,
            staked_at: staked_at,
            staked_by,
            staking_plan,
        };

        //fetch apy details from FT
        let ft = self.approved_fts.get(&stake.clone().ft_account_id).unwrap();
        let apy_map = ft.apy_against_duration.unwrap();
        let apy = apy_map.get(&stake.clone().staking_plan);
        //fetch minimum staking amount from APY
        let threshold: u128 = u128::from(apy.unwrap().clone().min_staking_amt);

        let calling_contrat = env::predecessor_account_id();
        assert!(
            self.approved_ft_token_ids.contains(&calling_contrat),
            "Only approved FT can be staked"
        );
        assert!(
            u128::from(amount) >= threshold,
            "Cannot stake less than {} tokens",
            threshold
        );
        assert!(
            stake.duration / THIRTY_DAYS >= apy.unwrap().min_duration.into(),
            "Invalid Duration"
        );

        if let Some(mut staking_history) = self.amount_staked.get(&sender_id) {
            log!("In IF");
            staking_history.push(stake);
            self.amount_staked.insert(&sender_id, &staking_history);
        } else {
            log!("In ELSE");
            let mut staking_history: Vec<Stake> = Vec::new();
            staking_history.push(stake);
            self.amount_staked.insert(&sender_id, &staking_history);
        }

        log!(
            "{:?} staked by {} with staking_id {}",
            amount,
            sender_id,
            staking_id
        );
        self.staking_nonce = staking_id;
        near_sdk::PromiseOrValue::Value(U128::from(a))
    }

    fn ft_unstake(&mut self, stake_id: StakeId) {
        // let stake_id = u128::from(stake_id);
        let staker_id: AccountId = env::predecessor_account_id().try_into().unwrap();
        let stake_history = self.amount_staked.get(&staker_id);

        let stake = stake_history
            .unwrap()
            .into_iter()
            .find(|i| i.stake_id == stake_id)
            .expect("No staking data with this id found for caller");

        let current_time = env::block_timestamp() / 1000000000;
        let staked_at = stake.staked_at;
        let duration = stake.duration;
        let amount = stake.amount;
        let staked_by = stake.staked_by;
        let ft_contract: AccountId = stake.ft_account_id.try_into().unwrap();
        let memo: Option<String> = Some("Unstaking with reward".to_string());

        assert_eq!(
            staked_by.to_string(),
            staker_id.to_string(),
            "Only owner of the tokens can unstake"
        );

        assert!(
            current_time >= staked_at + duration,
            "Cannot withdraw before locked time"
        );
        ext_ft::ft_transfer(
            staker_id.clone(),
            U128::from(amount),
            memo,
            ft_contract,
            1,
            BASE_GAS,
        )
        .then(this_contract::unstake_callback(
            stake_id,
            staker_id,
            env::current_account_id(),
            0,
            BASE_GAS,
        ));

        //remove staking info from vector
    }

    fn did_promise_succeded() -> bool {
        if env::promise_results_count() != 1 {
            log!("Expected a result on the callback");
            return false;
        }
        match env::promise_result(0) {
            PromiseResult::Successful(_) => true,
            _ => false,
        }
    }

    fn unstake_callback(&mut self, stake_id: StakeId, staker_id: AccountId) {
        if Self::did_promise_succeded() {
            let mut staking_history = self.amount_staked.get(&staker_id).unwrap();
            let index = &staking_history.iter().position(|i| i.stake_id == stake_id);

            let _ = &staking_history.remove(index.unwrap());

            self.amount_staked.insert(&staker_id, &staking_history);

            log!(
                "Staking ID {} removed from {}",
                u128::from(stake_id),
                index.unwrap()
            );
        }
    }

    fn claim_reward(&mut self, stake_id: StakeId) {
        let staker_id: AccountId = env::predecessor_account_id().try_into().unwrap();
        let stake_history = self
            .amount_staked
            .get(&staker_id)
            .expect("This user has not staked yet.");

        let stake = stake_history
            .into_iter()
            .find(|i| i.stake_id == stake_id)
            .expect("No staking data with this id found for caller");

        let current_time = env::block_timestamp() / 1000000000;
        // let current_time = 1653140036;
        let staked_at = stake.staked_at;
        // let duration = stake.duration / THIRTY_DAYS;
        let amount = u128::from(stake.amount);
        let staked_by = stake.staked_by;
        // let symbol = stake.ft_symbol;
        // let decimal = stake.decimal;

        let claim_history = self.claim_history.get(&stake_id);

        assert_eq!(
            staked_by.to_string(),
            staker_id.to_string(),
            "Only owner of the tokens can claim reward"
        );

        let difference: u64;
        if claim_history.is_none() {
            difference = (current_time - staked_at) / THIRTY_DAYS;
            // log!("{}", difference);
            assert!(
                difference >= 1,
                "Reward can be claimed after staking for 30 days"
            );
        } else {
            let claimed_at = claim_history.clone().unwrap().last_claimed_at;
            difference = (current_time - claimed_at) / THIRTY_DAYS;
            //log!("Current Time : {} Claimed at : {} Difference {}" ,current_time,claimed_at,difference);
            assert!(
                difference >= 1,
                "Reward can be claimed after 30 days of the last claim"
            );
        }

        //get FT details
        let ft = self.approved_fts.get(&stake.ft_account_id).unwrap();
        let apy_map = ft.apy_against_duration.unwrap();
        let apy = apy_map.get(&stake.staking_plan).unwrap();

        //log!("{:?}", apy);
        let ap = apy.interest_rate;

        let interest = (amount * (ap as u128)) / 100;
        let actual_amount = (interest / 100) * difference as u128;

        //log!("Actual amount for transfer {}", actual_amount);

        let memo: Option<String> = Some("Reward tokens".to_string());
        ext_ft::ft_transfer(
            staker_id.clone(),
            U128::from(actual_amount),
            memo,
            stake.ft_account_id,
            1,
            BASE_GAS,
        )
        .then(this_contract::claim_reward_callback(
            stake_id,
            claim_history.clone(),
            difference,
            env::current_account_id(),
            0,
            BASE_GAS,
        ));
    }

    fn claim_reward_callback(
        &mut self,
        stake_id: StakeId,
        claim_history: Option<ClaimHistory>,
        claim_count: u64,
    ) {
        let claim: ClaimHistory;
        let current_time = env::block_timestamp() / 1000000000;
        if claim_history.is_none() {
            let count = claim_count as u8;
            claim = ClaimHistory {
                last_claimed_at: current_time,
                claim_count: count,
            }
        } else {
            claim = ClaimHistory {
                last_claimed_at: current_time,
                claim_count: claim_history.unwrap().claim_count + 1,
            }
        }
        self.claim_history.insert(&stake_id, &claim);
        /*   let mut stake_history = self.amount_staked.get(&staker_id).unwrap();

        let mut stake = stake_history
            .clone()
            .into_iter()
            .find(|i| i.stake_id == stake_id)
            .expect("No staking data with this id found for caller");

        stake.duration = stake.duration - THIRTY_DAYS;

        let index = stake_history
            .iter()
            .position(|i| i.stake_id == stake_id)
            .unwrap();

        stake_history[index] = stake;

        self.amount_staked.insert(&staker_id, &stake_history); */
    }
}
