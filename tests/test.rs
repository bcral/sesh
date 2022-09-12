// integration test module imports
pub mod common; // must import module before performing granular imports
use common::{serializer::DepositArgs};

// macro allowing us to convert human readable units to workspace units.
use near_units::parse_near;

// macro allowing us to convert args into JSON bytes to be read by the contract.
use serde_json::json;

// Additional convenient imports that allows workspaces to function readily.
// use workspaces::prelude::*;
use near_workspaces::prelude::*;
use near_workspaces::AccountId;


const CONTRACT: &str = "./res/contract.wasm";

#[tokio::test]
async fn test() -> anyhow::Result<()> {
    
    // deploy contract to chain with wasm file
    let worker = near_workspaces::sandbox().await?;
    let wasm = std::fs::read(CONTRACT).expect("wasm file not found");
    let contract = worker.dev_deploy(&wasm).await?;
    
    // *** CONTRACT INITIALIZATION EXAMPLE ***
    // initialize contract calling the `new()` function
    contract.call(&worker, "init")
        // .gas(near_units::parse_gas!("300 T") as u64)
        .transact()
        .await?;

    // *** PAYABLE FUNCTION WITH STATE CHANGE FUNCTION ***
    let deposit:u128 = 500;
    contract.call(&worker, "deposit")
        .deposit(deposit)
        .transact()
        .await?;

    // *** VIEW FUNCTION EXAMPLE ***
    // check storage variable `balance`
    let args_obj = DepositArgs::new(contract.id().clone());
    let args = args_obj.try_to_json();
    let balance_res: u128 = contract
        .call(&worker, "get_balance")
        .args_json(args)?
        .transact()
        .await?
        .json()?;
    println!("BALANCE: {:?}", balance_res);
    assert_eq!(deposit, balance_res, "Balance did not update in contract");

    Ok(())
}