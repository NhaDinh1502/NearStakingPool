use crate::*;

#[near_bindgen]
impl Contract {
    /*
     * Deposit and Stake an Amount
     **/
    pub(crate) fn internal_deposit_and_stake(&mut self, account_id: AccountId, amount: u128) {
        let optional_account = self.accounts.get(&account_id);
        
        assert!(optional_account.is_some(), "ERR_ACCOUNT_NOT_FOUND");
        assert!(!self.paused, "ERR_CONTRACT_PAUSED");
        assert_eq!(self.ft_contract, env::predecessor_account_id(), "ERR_INVALID_FT_CONTRACT_ID");

        let mut account = optional_account.unwrap();

        if account.stake_balance == 0 {
            self.total_staker += 1;
        }

        let new_reward = self.internal_cal_account_reward(&account);

        // update account data
        account.pre_reward += new_reward;
        account.stake_balance += amount;
        account.last_block_balance_change = env::block_index();

        self.accounts.insert(&account_id, &account);


        // Update pool data
        let new_contract_reward = self.internal_cal_global_reward();
        self.total_stake_balance += amount;
        self.pre_reward += new_contract_reward;
        self.last_block_balance_change = env::block_index();
        
    }
    
    /*
     * Deposit and Stake an Amount
     **/
    pub(crate) fn internal_unstake(&mut self, account_id: AccountId, amount: u128) {
        let optional_account = self.accounts.get(&account_id);
        assert!(optional_account.is_some(), "ERR_ACCOUNT_NOT_FOUND");
        
        let mut account = optional_account.unwrap();

        assert!(amount <= account.stake_balance, "ERR_AMOUNT_MUST_LESS_THAN_STAKE_BALANCE");

        let new_reward = self.internal_cal_account_reward(&account);

        // update account data
        account.pre_reward += new_reward;
        account.stake_balance -= amount;
        account.last_block_balance_change = env::block_index();
        account.unstake_balance += amount;
        account.unstake_start_timestamp = env::block_timestamp();
        account.unstake_available_epoch = env::epoch_height();

        if account.stake_balance == 0 {
            self.total_staker -= 1;
        }

        self.accounts.insert(&account_id, &account);

        let new_contract_reward = self.internal_cal_global_reward();
        self.pre_reward += new_contract_reward;
        self.last_block_balance_change = env::block_index();
        self.total_stake_balance -= amount;
    }
    
    /*
     * Deposit and Stake an Amount
     **/
    pub(crate) fn internal_withdraw(&mut self, account_id: AccountId) -> Account {
        let optional_account = self.accounts.get(&account_id);
        assert!(optional_account.is_some(), "ERR_ACCOUNT_NOT_FOUND");
        
        let account = optional_account.unwrap();
        
        assert!(account.unstake_balance > 0, "ERR_UNSTAKE_BALANCE_EQUAL_ZERO");
        assert!(account.unstake_available_epoch <= env::epoch_height(), "ERR_DISABLED_WITHDRAW");

        let new_account = Account {
            account_id: account_id.clone(),
            stake_balance: account.stake_balance,
            pre_reward: account.pre_reward,
            last_block_balance_change: account.last_block_balance_change,
            unstake_balance: 0,
            unstake_start_timestamp: 0,
            unstake_available_epoch: 0,
        };

        self.accounts.insert(&account_id, &new_account);

        account
    }

    /*
     * Register Account
     **/
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
    
    /*
     * Calculate Account Reward 
     **/
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

    /*
     * Calculate All Reward In Pool
     **/
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
