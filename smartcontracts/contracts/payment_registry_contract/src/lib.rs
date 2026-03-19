#![no_std]

use soroban_sdk::{contract, contractimpl, Env};

mod logic;
mod storage;
mod types;

#[contract]
pub struct PaymentRegistryContract;

#[contractimpl]
impl PaymentRegistryContract {
    pub fn version(_env: Env) -> u32 {
        1
    }
}
