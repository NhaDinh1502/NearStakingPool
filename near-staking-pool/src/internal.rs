use crate::*;

#[near_bindgen]
impl Contract {
    pub(crate) fn internal_register_acccount(&mut self, account_id: AccountId) {
        let account = Account {
            account_id: account_id.clone(),
            stake_balance: 0,
            pre_reward: 0,
            last_block_balance_change: env::block_index(),
            unstake_balance: 0,
            unstake_start_timestamp: 0,
            unstake_available_epoch: 0,
        };
        self.accounts.insert(&account_id, &account);
    } 
    
    pub(crate) fn internal_cal_account_reward(&self, account: &Account) -> Balance {
        let lasted_block = if self.paused {
            self.pause_in_block
        } else {
            env::block_index()
        };

        let diff_block = lasted_block - account.last_block_balance_change;
        let reward: Balance = (account.stake_balance * self.config.reward_numerator as u128 * diff_block as u128) / self.config.reward_denumerator as u128;

        reward
    }

    pub(crate) fn internal_cal_global_reward(&self) -> Balance {
        let lasted_block = if self.paused {
            self.pause_in_block
        } else {
            env::block_index()
        };

        let diff_block = lasted_block - self.last_block_balance_change;
        let reward: Balance = (self.total_stake_balance * self.config.reward_numerator as u128 * diff_block as u128) / self.config.reward_denumerator as u128;

        reward
    }
}
