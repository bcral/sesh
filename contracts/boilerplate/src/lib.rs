use near_sdk::{
    near_bindgen, env, log, require, PanicOnDefault, AccountId, Balance,
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::{LookupMap}, 
    serde::{Serialize, Deserialize},
    json_types::{U128}
};


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct BoilerplateContract {
    owner: AccountId,
    balance: LookupMap<AccountId, Balance>
}

#[near_bindgen]
impl BoilerplateContract {
    #[init]
    pub fn init() -> Self {
        Self {
            owner: env::predecessor_account_id(),
            balance: LookupMap::new(b"b")
        }
    }

    #[payable]
    pub fn deposit(&mut self) {
        let deposit = env::attached_deposit();
        require!(deposit > 0);
        let caller = env::predecessor_account_id();
        match self.balance.get(&caller) {
            Some(deposits) => self.balance.insert(&caller, &deposits.checked_add(deposit).unwrap()),
            _ => self.balance.insert(&caller, &deposit)
        };
    }

    pub fn get_balance(&self, account_id: AccountId) -> u128 {
        self.balance.get(&account_id).unwrap_or(0).into()
    }

}


#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder, };
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, Balance};

    use super::*;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }


    #[test]
    fn test_deposit() {
        let deployer : AccountId = "test.near".parse().unwrap();
        // set Test VM context
        let mut context = get_context(deployer.clone());
        testing_env!(context.build());
        // deploy contract
        let mut contract = BoilerplateContract::init();

        // update VM to register attached near and call deposit on contract
        let deposit: u128 = 50;
        context.attached_deposit(deposit);
        testing_env!(context.build());
        contract.deposit();

        // view deposits from contract
        let deposits = contract.get_balance(deployer);

        // assert storage update
        println!("Deposits: {:?}", deposits);
        assert_eq!(deposit, deposits, "Deposits did not register");   
    }

   
}


