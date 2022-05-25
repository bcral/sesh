
use borsh::{BorshSerialize, BorshDeserialize};
use near_workspaces::AccountId;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
pub struct DepositArgs {
    account_id: AccountId
}

impl DepositArgs {

    pub fn new(account_id: AccountId) -> Self{
        Self {
            account_id
        }
    }

    pub fn try_to_vec(&self) -> Vec<u8> {
        self.account_id.try_to_vec().unwrap()
    }
}














// // macro allowing us to convert human readable units to workspace units.
// use near_units::parse_near;

// // macro allowing us to convert args into JSON bytes to be read by the contract.
// use serde_json::json;

// // Additional convenient imports that allows workspaces to function readily.
// // use workspaces::prelude::*;
// use core::pin::Pin;
// use core::future::Future;
// use near_workspaces::prelude::*;
// use near_workspaces::{Contract, Worker, Network, AccountId, Account};
// use anyhow::Result;
// use async_trait::async_trait;

// // pub trait TopLevelAccountDeployer {
// //     fn dev_deploy<'life0, 'async_trait>(&'life0 self, wasm: Vec<u8>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = anyhow::Result<Contract>> + ::core::marker::Send + 'async_trait>>
// // where
// //     'life0: 'async_trait,
// //     Self: 'async_trait;
// // }

// pub trait DevAccountDeployer {
//     fn dev_generate<'life0, 'async_trait>(
//         &'life0 self
//     ) -> Pin<Box<dyn Future<Output = (AccountId, SecretKey)> + Send + 'async_trait>>
//     where
//         'life0: 'async_trait,
//         Self: 'async_trait;
//     // fn dev_create_account<'life0, 'async_trait>(
//     //     &'life0 self
//     // ) -> Pin<Box<dyn Future<Output = Result<Account>> + Send + 'async_trait>>
//     // where
//     //     'life0: 'async_trait,
//     //     Self: 'async_trait;
//     fn dev_deploy<'life0, 'life1, 'async_trait>(
//         &'life0 self, 
//         wasm: &'life1 [u8]
//     ) -> Pin<Box<dyn Future<Output = Result<Contract>> + Send + 'async_trait>>
//     where
//         'life0: 'async_trait,
//         'life1: 'async_trait,
//         Self: 'async_trait;
// }

// #[async_trait]
// impl<T> DevAccountDeployer for T
// where
//     T: TopLevelAccountCreator + Send + Sync,
// {

//     async fn dev_generate(&self) -> (AccountId, SecretKey) {
//         let id = crate::rpc::tool::random_account_id();
//         let sk = SecretKey::from_seed(KeyType::ED25519, DEV_ACCOUNT_SEED);
//         (id, sk)
//     }

//     async fn dev_deploy(&self, wasm: &[u8]) -> anyhow::Result<Contract> {
//         let (id, sk) = self.dev_generate().await;
//         let contract = self.create_tla_and_deploy(id.clone(), sk, wasm).await?;
//         contract.into()
//     }
// }

// // trait Sandbox = Box<dyn Send + Sync + TopLevelAccountCreator + TopLevelAccountDeployer>;
// pub trait Sandbox
// where
//     Self: Send + Sync + DevAccountDeployer {

//     }

// pub async fn deploy(worker: &Worker<Box<dyn Sandbox>>, wasm_file: &str) -> anyhow::Result<Contract> {
//     let wasm = std::fs::read(wasm_file).expect("wasm file not found");
//     let contract = worker.dev_deploy(wasm).await?;
//     Ok(contract)
// } 
