use near_sdk::Timestamp;

use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate="near_sdk::serde")]
pub struct Account {
    pub account_id: AccountId,
    pub stake_balance: Balance,
    pub pre_reward: Balance,
    pub last_block_balance_change: BlockHeight,
    pub unstake_balance: Balance,
    pub unstake_start_timestamp: Timestamp,
    pub unstake_available_epoch: EpochHeight,
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AccountJson {
    pub account_id: AccountId,
    pub stake_balance: U128,
    pub unstake_balance: U128,
    pub reward: U128,
    pub can_withdraw: bool,
    pub unstake_start_timestamp: Timestamp,
    pub unstake_available_epoch: EpochHeight,
    pub current_epoch: EpochHeight,
}

impl AccountJson {
    pub fn from(new_reward: Balance, account: Account) -> Self {
        AccountJson {
            account_id: account.account_id,
            stake_balance: U128(account.stake_balance),
            unstake_balance: U128(account.unstake_balance),
            reward: U128(account.pre_reward + new_reward),
            can_withdraw: account.unstake_available_epoch <= env::epoch_height(),
            unstake_start_timestamp: account.unstake_start_timestamp,
            unstake_available_epoch: account.unstake_available_epoch,
            current_epoch: env::epoch_height(),
        }
    } 
}

#[near_bindgen]
impl Contract {
    pub fn get_account_info(&self, account_id: AccountId) -> AccountJson {
        let account = self.accounts.get(&account_id).unwrap();
        let new_reward = self.internal_cal_account_reward(&account);
        AccountJson::from(new_reward, account)
    }

    pub fn get_account_reward(&self, account_id: AccountId) -> Balance {
        let account = self.accounts.get(&account_id).unwrap();
        let new_reward = self.internal_cal_account_reward(&account);

        account.pre_reward + new_reward
    }
}
