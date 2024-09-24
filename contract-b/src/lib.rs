// #![no_std]

use soroban_sdk::{
    auth::{Context, ContractContext, CustomAccountInterface},
    contract, contracterror, contractimpl,
    crypto::Hash,
    vec, Env, Val, Vec,
};

mod test;

#[contract]
pub struct Contract;

#[contracterror]
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u32)]
pub enum Error {
    NotAllowed = 1,
}

#[contractimpl]
impl CustomAccountInterface for Contract {
    type Error = Error;
    type Signature = Val;

    #[allow(non_snake_case)]
    fn __check_auth(
        env: Env,
        _signature_payload: Hash<32>,
        _signature: Val,
        auth_contexts: Vec<Context>,
    ) -> Result<(), Error> {
        if let Context::Contract(ContractContext { contract: contract_b_address, .. }) = auth_contexts.get_unchecked(0) {
            println!("fired 1");
            contract_b_address.require_auth_for_args(vec![&env]);
            println!("fired 2");
        }

        Ok(())
    }
}
