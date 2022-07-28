use std::u128;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, setup_alloc, AccountId, Balance, BlockHeight, BorshStorageKey, EpochHeight,
    PanicOnDefault, Promise,
};

mod account;
mod config;
mod internal;
mod utils;

use account::*;
use config::*;
use internal::*;
use utils::*;

setup_alloc!();

#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
#[near_bindgen]
pub struct Contract {
    pub owner_id: AccountId,
    pub ft_contract: AccountId,
    pub config: Config,
    pub total_stake_balance: Balance,
    pub total_paid_reward_balance: Balance,
    pub total_staker: u128,
    pub pre_reward: Balance,
    pub last_block_balance_change: BlockHeight,
    pub accounts: LookupMap<AccountId, Account>,
    pub paused: bool,
    pub pause_in_block: BlockHeight,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId, ft_contract_id: AccountId, config: Config) -> Self {
        Contract {
            owner_id,
            ft_contract: ft_contract_id,
            config,
            total_stake_balance: 0,
            total_paid_reward_balance: 0,
            total_staker: 0,
            pre_reward: 0,
            last_block_balance_change: env::block_index(),
            accounts: LookupMap::new(StorageKey::Accounts),
            paused: false,
            pause_in_block: 0,
        }
    }

    #[init]
    pub fn new_default(ft_contract_id: AccountId) -> Self {
        Self::new(
            env::predecessor_account_id(),
            ft_contract_id,
            Config::default(),
        )
    }
}

/*
 Storage Deposit
*/
#[near_bindgen]
impl Contract {
    pub fn register_and_deposit_storage(&mut self) {
        assert_at_least_one_yocto();
        let account = env::predecessor_account_id();
        let account_stake = self.accounts.get(&account);

        if account_stake.is_some() {
            refund_deposit(0);
        } else {
            let before_usage = env::storage_usage();
            self.internal_register_acccount(account);
            let after_usage = env::storage_usage();

            refund_deposit(after_usage - before_usage);
        }
    }
    
    pub fn storage_balance_of(&self, account_id: AccountId) -> bool {
        let account = self.accounts.get(&account_id);
        account.is_some()
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }
}
