use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{near_bindgen, AccountId, BorshStorageKey, PanicOnDefault,env};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner_id: AccountId,
    approved_ft_token_ids: UnorderedSet<FT>,
    amount_staked: LookupMap<AccountId, Staking>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct FT {
    account_id: AccountId,
    symbol: String,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Staking{
    ft_symbol : String,
    amount  :u128,
    duration : u128,
    staked_at : u128
}

#[derive(BorshStorageKey, BorshSerialize)]
enum StorageKeys {
    ApproveFungibleTokens,
    AmountStaked,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId, approved_ft_token_ids: UnorderedSet<FT>) -> Self {
        let mut this = Self {
            owner_id: owner_id.into(),
            approved_ft_token_ids: UnorderedSet::new(StorageKeys::ApproveFungibleTokens),
            amount_staked: LookupMap::new(StorageKeys::AmountStaked),
        };

        this
    }

    pub fn stake (&mut self, ft_account_id : AccountId , amount :u128, duration: u128){
        let staker : AccountId = env::predecessor_account_id().try_into().unwrap();
    }
}
