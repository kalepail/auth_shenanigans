#![cfg(test)]

extern crate std;
use std::println;

use crate::{Contract as ContractA, ContractClient as ContractAClient};
use contract_b::{Contract as ContractB, ContractClient as ContractBClient, Error};
use soroban_sdk::{
    testutils::{Address as _, BytesN as _},
    vec,
    xdr::{
        InvokeContractArgs, ScVal, ScVec, SorobanAddressCredentials, SorobanAuthorizationEntry,
        SorobanAuthorizedFunction, SorobanAuthorizedInvocation, SorobanCredentials, VecM,
    },
    Address, BytesN, Env, IntoVal,
};

#[test]
fn test() {
    let env: Env = Env::default();
    let signature_expiration_ledger = env.ledger().sequence();

    let contract_a_address = env.register_contract(None, ContractA);
    let contract_a_client = ContractAClient::new(&env, &contract_a_address);

    let contract_b_address = env.register_contract(None, ContractB);
    let contract_b_client = ContractBClient::new(&env, &contract_b_address);

    let res = env.try_invoke_contract_check_auth::<Error>(
        &contract_b_address,
        &BytesN::random(&env),
        ScVal::Vec(Some(ScVec::default())).into_val(&env),
        &vec![&env],
    );

    println!("{:?}", res);

    // let root_invocation = SorobanAuthorizedInvocation {
    //     function: SorobanAuthorizedFunction::ContractFn(InvokeContractArgs {
    //         contract_address: contract_a_address.clone().try_into().unwrap(),
    //         function_name: "call".try_into().unwrap(),
    //         args: VecM::default(),
    //     }),
    //     sub_invocations: VecM::default(),
    // };

    // let root_auth = SorobanAuthorizationEntry {
    //     credentials: SorobanCredentials::Address(SorobanAddressCredentials {
    //         address: contract_b_address.clone().try_into().unwrap(),
    //         nonce: 0,
    //         signature_expiration_ledger,
    //         signature: ScVal::Vec(Some(ScVec::default())),
    //     }),
    //     root_invocation: root_invocation.clone(),
    // };

    // let __check_auth_invocation = SorobanAuthorizedInvocation {
    //     function: SorobanAuthorizedFunction::ContractFn(InvokeContractArgs {
    //         contract_address: contract_a_address.clone().try_into().unwrap(),
    //         function_name: "__check_auth".try_into().unwrap(),
    //         args: VecM::default(),
    //     }),
    //     sub_invocations: VecM::default(),
    // };

    // let __check_auth = SorobanAuthorizationEntry {
    //     credentials: SorobanCredentials::Address(SorobanAddressCredentials {
    //         address: contract_b_address.clone().try_into().unwrap(),
    //         nonce: 0,
    //         signature: ScVal::Vec(Some(ScVec::default())),
    //         signature_expiration_ledger,
    //     }),
    //     root_invocation: __check_auth_invocation.clone(),
    // };

    // contract_a_client
    // .set_auths(&[
    //     root_auth,
    //     // __check_auth
    // ])
    // .call(&contract_b_address);
}
