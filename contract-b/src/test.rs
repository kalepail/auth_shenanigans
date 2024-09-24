#![cfg(test)]

extern crate std;
use std::println;

use crate::{Contract as ContractB, ContractClient as ContractBClient};
use soroban_sdk::{
    contractfile,
    testutils::{Address as _, BytesN as _},
    vec,
    xdr::{
        InvokeContractArgs, ScVal, ScVec, SorobanAddressCredentials, SorobanAuthorizationEntry,
        SorobanAuthorizedFunction, SorobanAuthorizedInvocation, SorobanCredentials, VecM,
    },
    Address, BytesN, Env, IntoVal,
};

// use contract_a::{Contract as ContractA, ContractClient as ContractAClient};
use contract_a::{Client as ContractAClient, WASM as ContractAWASM};
mod contract_a {
    use soroban_sdk::auth::Context;
    soroban_sdk::contractimport!(file = "../target/wasm32-unknown-unknown/release/contract_a.wasm");
}

#[test]
fn test() {
    let env: Env = Env::default();
    let signature_expiration_ledger = env.ledger().sequence();

    // let contract_a_address = env.register_contract(None, ContractA);
    let contract_a_address = env.register_contract_wasm(None, ContractAWASM);
    let contract_a_client = ContractAClient::new(&env, &contract_a_address);

    let contract_b_address = env.register_contract(None, ContractB);
    // let contract_b_client = ContractBClient::new(&env, &contract_b_address);

    let root_invocation = SorobanAuthorizedInvocation {
        function: SorobanAuthorizedFunction::ContractFn(InvokeContractArgs {
            contract_address: contract_a_address.clone().try_into().unwrap(),
            function_name: "call".try_into().unwrap(),
            args: VecM::default(),
        }),
        sub_invocations: VecM::default(),
    };

    let root_auth = SorobanAuthorizationEntry {
        credentials: SorobanCredentials::Address(SorobanAddressCredentials {
            address: contract_b_address.clone().try_into().unwrap(),
            nonce: 0,
            signature_expiration_ledger,
            signature: ScVal::Vec(Some(ScVec::default())),
        }),
        root_invocation: root_invocation.clone(),
    };

    contract_a_client
        .set_auths(&[root_auth])
        .call(&contract_b_address);
}
