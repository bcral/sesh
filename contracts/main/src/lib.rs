use near_sdk::{
    near_bindgen, env, log, require, PanicOnDefault, AccountId, Balance,
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::{LookupMap}, 
    // serde::{Serialize, Deserialize},
    Promise, Timestamp, PromiseError,
};


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner: AccountId,
    balances: LookupMap<AccountId, Balance>,
    months: LookupMap<AccountId, u128>,
    sub_dates: LookupMap<AccountId, Timestamp>,
    sub_index: u128,
    last_sub_withdrawal: u128,
    yocto_per_credit: u128,
    yocto_per_month: u128,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn init(ypc: u128, ypm: u128) -> Self {
        Self {
            owner: env::predecessor_account_id(),
            balances: LookupMap::new(b"b"),
            months: LookupMap::new(b"m"),
            sub_dates: LookupMap::new(b"s"),
            sub_index: 0,
            last_sub_withdrawal: 0,
            // set the number of yoctos per credit.  ex: 1000000 would make each API call cost 1,000,000 yocto
            yocto_per_credit: ypc,
            // set the number of yoctos per month.  ex: 10000000000 would set the cost of 1 month of service to 10,000,000,000 yocto
            yocto_per_month: ypm
        }
    }

    // Deposits all passed NEAR to the pay-per-call system, generating credits at the rate set in init
    // Credits = Passed NEAR / yocto_per_credit
    #[payable]
    pub fn deposit_ppc(&mut self) {
        let deposit = env::attached_deposit();
        require!(deposit > 0);
        let caller = env::predecessor_account_id();
        match self.balances.get(&caller) {
            Some(deposits) => self.balances.insert(&caller, &deposits.checked_add(deposit).unwrap()),
            _ => self.balances.insert(&caller, &deposit)
        };
    }

    // Deposits all passed NEAR to the 1 month subscription
    // Credits = Passed NEAR / yocto_per_month
    #[payable]
    pub fn deposit_monthly(&mut self) {
        // store monthly rate in memory
        let ypm: u128 = self.yocto_per_month;
        require!(env::attached_deposit() >= ypm);

        // store calling account in memory
        let caller: AccountId = env::predecessor_account_id();
        
        let overpay: u128 = env::attached_deposit() % ypm;
        let periods: u128 = env::attached_deposit() / ypm;
        // if attached near is not a modulo of yocto_per_month, return the remainder
        if overpay > 0 {
            Promise::new(caller.clone()).transfer(overpay);
        }

        self.sub_dates.insert(&caller, &env::block_timestamp());
        
        match self.months.get(&caller) {
            Some(months) => self.months.insert(&caller, &months.checked_add(periods).unwrap()),
            _ => self.balances.insert(&caller, &periods)
        };

        // increase the subscription index by adding all newly funded months
        // *note* 
        //      With the subscription model, all subscription fees cannot be withdrawn after they are sent. This means
        //      funds are available to the owner to withdraw as soon as they are paid.
        self.sub_index += periods;
    }

    // For withdrawing NEAR based on # of credits
    pub fn withdraw(&mut self, mut amount: u128) {
        let current_amnt: u128 = self.balances.get(&env::predecessor_account_id()).unwrap();
        // If withdrawal amount is more than credits owned, withdraw all credits.
        if &amount > &current_amnt {
            amount = current_amnt;
        }
        let amount_near: u128 = &amount * &self.yocto_per_credit;
        self.withdraw_near(amount_near);
    }

    // For withdrawing based on NEAR value, not # of credits
    pub fn withdraw_near(&mut self, mut amount_near: u128) {

        // Get credits
        let current_near: u128 = &self.balances.get(&env::predecessor_account_id()).unwrap() * &self.yocto_per_credit;
        // If withdrawal amount is more than near value of credits, withdraw all near available.
        if &amount_near > &current_near {
            amount_near = current_near;
        }
        require!(env::account_balance() >= amount_near, "This contract does not have the required funds.");
        match amount_near.checked_sub(current_near) {
            // update user's balance appropriately
            Some(checked_balance) => self.balances.insert(
                &env::predecessor_account_id(),
                &(checked_balance.checked_mul(self.yocto_per_credit).unwrap())
            ),
            _ => self.balances.insert(&env::predecessor_account_id(), &0)
        };
        // transfer near
        Promise::new(env::predecessor_account_id()).transfer(amount_near);
    }

    pub fn owner_withdraw(&mut self) -> Promise {
        require!(env::predecessor_account_id() == self.owner, "Only the owner can withdraw the owner's funds.");

        // transfer balance of owner to the owner
        let sub_total: u128 = (self.sub_index - self.last_sub_withdrawal) * self.yocto_per_month;
        let total_withdraw: u128 = self.balances.get(&self.owner).unwrap() + sub_total;
            
        // transfer total withdrawable amount to sender
        let trx: Promise = Promise::new(self.owner.clone()).transfer(total_withdraw);

        return trx.then(
            Self::ext(env::current_account_id())
            .owner_withdraw_callback()
        )
    }

    #[private]
    pub fn owner_withdraw_callback(&mut self, #[callback_result] call_result: Result<String, PromiseError>) {
        if call_result.is_err() {
            log!("There was an error contacting Hello NEAR");
            return
        }
        // reset owner's balance to 0
        self.balances.insert(&self.owner, &0);
        // set last withdrawal index to current index
        self.last_sub_withdrawal = self.sub_index;
    }

    // Only used for debiting balance when funds are used
    pub fn update_balance(&mut self, user: AccountId, amount: u128) {
        require!(amount > 0, "No need to update without changes to make");
        // if subscription, whatever

        // if not subscription, check credits
        let current_amnt: &u128 = &self.balances.get(&user).unwrap();
        let owner_amnt: &u128 = &self.balances.get(&self.owner).unwrap();
        // Check for underflow - Prevent underflow panic by setting balance to 0
        match current_amnt.checked_sub(amount) {
            // update user's balance appropriately
            Some(checked_balance) => self.balances.insert(&user, &(checked_balance)),
            _ => self.balances.insert(&user, &0)
        };
        // Check for overflow?  Not sure it's needed...
        self.balances.insert(&self.owner, &(owner_amnt + amount));
    }

    // Retrieve the balance of credits belonging to account_id
    pub fn get_balance(&self, account_id: AccountId) -> u128 {
        self.balances.get(&account_id).unwrap_or(0).into()
    }

    // Retrieve the cost of each call credit
    pub fn get_credit_cost(&self) -> u128 {
        self.yocto_per_credit
    }

    // Retrieve the cost of one month subscription
    pub fn get_monthly_cost(&self) -> u128 {
        self.yocto_per_month
    }
}

