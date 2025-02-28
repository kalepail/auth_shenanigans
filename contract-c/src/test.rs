#![cfg(test)]

use soroban_sdk::{testutils::Address as _, token::{StellarAssetClient, TokenClient}, Address, Env};

use crate::{Contract, ContractClient};

#[test]
fn test() {
    let env = Env::default();

    // env.mock_all_auths();
    env.mock_all_auths_allowing_non_root_auth();

    let contract_address = env.register(Contract, ());
    let contract_client = ContractClient::new(&env, &contract_address);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    let sac = env.register_stellar_asset_contract_v2(admin);
    let sac_address =  sac.address();
    let token_admin = StellarAssetClient::new(&env, &sac_address);
    // let token_client = TokenClient::new(&env, &sac_address);

    token_admin.mint(&user, &100);

    contract_client.call(&user, &sac_address);
}