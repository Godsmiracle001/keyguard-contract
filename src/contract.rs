use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct KeyguardContract;

#[contractimpl]
impl KeyguardContract {
    pub fn hello(_env: Env) {}
}