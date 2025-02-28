#![no_std]

use soroban_sdk::{contract, contractimpl, token, Address, Env};

mod test;

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn call(env: Env, source: Address, sac: Address) {
        let token_client = token::Client::new(&env, &sac);
        token_client.transfer(&source, &source, &100);
    }
}