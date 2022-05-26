# Near Boilerplate

## Overview

This repo is a boilerplate meant to standardize file structure in a NEAR development environment as well as integration with near-workspaces. 
With this template, developers can easily add multiple custom contracts to a single workspace, import external contracts (as `.wasm` files) into a workspace, and perform simulation tests on 
a local Sandbox instance.

## Layout

This section explains the file system layout of this template. For the purpose of standardization and faster onboarding to a project, this layout is only meant to be extended, not manipulated
- ### Contracts

    - Your NEAR contracts will reside in this directory. Each contract must have it's own folder with a `Cargo.toml` and `lib.rs` file to define it as it's own contract. After adding a contract, you must add the contract's root folder path to the `[workspaces]` section in the `Cargo.toml` in the root project folder. This allows the root project to recognize the contract folder as its own contract within the environment for building and testing

    Because each contract within the `/contracts` directory is isolated, they cannot import other contracts within the root contracts directory, and thus cannot perform cross-contract calls in their unit tests. To perform testing of cross contract calls read the simulation tests section.
- ### Simulation Tests

    - Simulation tests reside in the `/tests` directory and are executed with the near-workspaces rust crate (see [https://doc.rust-lang.org/cargo/reference/workspaces.html]). Each file in the tests directory is its own simulation test. Helper functions are files that reside in the `/tests/common` directory. To create a helper function, import it into `/test/common/mod.rs` as a public module. Then the specified module can be used in the required simulation test file

    - DOES NEAR WORKSPACES REQUIRE A MANUAL SANDBOX INSTALL??

- ### Scripts

    - The `./scripts` directory should be used to house files whose goal is to execute NEAR commands. Ideally, we will have a framework that allows these scripts to be easily ran in a common language

- ### Res

    - This `/res` directory is generated by the `./build` command (see Bash Scripts section). This folder allows simple storage of the `.wasm` contract builds as opposed to the `./target` directory

## Near Workspaces
- see `[https://github.com/near/workspaces-rs]

- UPDATE AFTER LEARNING MORE DETAILS

## Bash Scripts

- The scripts reside in the root project directory, and are used to combine cargo, near, and general bash scripting to speed up development. 
(Note: These commands will only run on a linux environment, and you may need to run `sudo chmod +rx {FILE_NAME}` to allow execution and read permissions.)

- `./build` 
    - This function should be used to build contracts because it will recognize all contracts in your workspace, and copy the build output (.wasm files) to the `/res` directory. This will make it easy to find 
    your contract's .wasm builds as opposed to searching through the `/target` folders

- `./sim`
    - This function will first build any changes made to the contracts, and execute the simulation tests

- `./clean_sb`
    - This function provides shorter command for cleaning the Sandbox cache


 
## Global Config

This section explains how to add contracts, simulation tests, and aliased cargo commands.

### Cargo.toml
- `[dev-dependencies]`
 - These dependencies will only be utilized in the `test` folder. Contract dependencies reside in the respective contract's Cargo.toml.

- `[workspaces]`
    - see [https://doc.rust-lang.org/cargo/reference/workspaces.html]
    - A workspace is a collection of one or more packages that share common dependency resolution (with a shared `Cargo.lock`), output directory, and various settings such as profiles
    - To add a contract append the path to the contract's root folder (contains `Cargo.toml`) to the array parameter in the `[workspaces]` section

- `[test]`
    - see  [https://doc.rust-lang.org/cargo/reference/cargo-targets.html]

### config.toml
- see [https://doc.rust-lang.org/cargo/reference/config.html]

- `[alias]`
    - Some default aliased commands already set up to speed up testing. Feel free to extend to your liking

## TODO (Possible added functionality
- add Typechain like bindings for contracts
- add scripting languange for `/scripts` folder (TS/Python)
