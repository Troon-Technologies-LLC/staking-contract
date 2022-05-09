use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault};
use std::cmp::min;
use std::collections::HashMap;

pub type StakingId = u128;

pub use crate::ft_calls::*;
pub use crate::internals::*;

mod ft_calls;
mod internals;

// symbol + plan + duration Ex : BKRTPremium6
pub type APYKey = String;
pub type StakeId = U128;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner_id: AccountId,
    approved_ft_token_ids: UnorderedSet<AccountId>,
    approved_fts: LookupMap<AccountId, FT>,
    amount_staked: LookupMap<AccountId, Vec<Stake>>,
    claim_history: LookupMap<StakeId, ClaimHistory>,
    staking_nonce: u128,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct FT {
    pub account_id: AccountId,
    pub symbol: String,
    pub apy_against_duration: Option<HashMap<APYKey, APY>>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct APY {
    pub apy_key: APYKey,
    pub interest_rate: u16, // interest Ex 1000 for 10%
    pub min_staking_amt: U128,
    pub min_duration: u8, //in months Ex 6
    pub plan_name: String,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Stake {
    stake_id: StakeId,
    ft_symbol: String,
    ft_account_id: AccountId,
    decimal: u8,
    amount: U128,
    duration: u64,
    staked_at: u64,
    staked_by: AccountId,
    staking_plan: String, //Ex BKRTPremium6
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct StakeArgs {
    ft_symbol: String,
    ft_account_id: AccountId,
    decimal: u8,
    duration: u64, //duration in milliseconds Ex 30 days = 2629800
    staked_by: AccountId,
    staking_plan: String, //Ex BKRTPremium6
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ClaimHistory {
    last_claimed_at: u64,
    claim_count: u8,
}

#[derive(BorshStorageKey, BorshSerialize)]
enum StorageKeys {
    ApproveFungibleTokens,
    AmountStaked,
    ClaimHistory,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId, approved_ft_token_ids: Vec<FT>, ft_apy: Vec<APY>) -> Self {
        let mut this = Self {
            owner_id: owner_id.into(),
            approved_ft_token_ids: UnorderedSet::new(StorageKeys::ApproveFungibleTokens),
            approved_fts: LookupMap::new(StorageKeys::ApproveFungibleTokens),
            amount_staked: LookupMap::new(StorageKeys::AmountStaked),
            claim_history: LookupMap::new(StorageKeys::ClaimHistory),
            staking_nonce: 0,
        };

        Contract::add_fts(
            approved_ft_token_ids,
            &mut this.approved_fts,
            &mut this.approved_ft_token_ids,
            ft_apy,
        );
        //this.approved_ft_token_ids.insert(&near_account());

        this
    }

    pub fn get_staking_history(
        self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<Stake> {
        let mut stake_history = vec![];
        if self.amount_staked.get(&account_id).is_none() {
            return stake_history;
        }
        let owner_stakes = self.amount_staked.get(&account_id).unwrap();
        let start = u128::from(from_index.unwrap_or(U128(0)));
        let end = min(
            start + (limit.unwrap_or(0) as u128),
            owner_stakes.len().try_into().unwrap(),
        );

        for i in start..end {
            stake_history.push(owner_stakes[i as usize].clone());
        }

        self.amount_staked.get(&account_id).unwrap()
    }

    pub fn get_claim_history(self, stake_id: StakeId) -> Option<ClaimHistory> {
        self.claim_history.get(&stake_id)
    }
}
