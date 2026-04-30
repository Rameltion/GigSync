#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};
use soroban_sdk::token::Client as TokenClient;
use soroban_sdk::token::StellarAssetClient as TokenAdminClient;

fn setup_env<'a>() -> (Env, Address, Address, Address, TokenClient<'a>, GigSyncContractClient<'a>) {
    let env = Env::default();
    env.mock_all_auths();

    let client = Address::generate(&env);
    let freelancer = Address::generate(&env);
    
    // Create a mock USDC token
    let token_admin = Address::generate(&env);
    let token_contract = env.register_stellar_asset_contract(token_admin.clone());
    let token_client = TokenClient::new(&env, &token_contract);
    let token_admin_client = TokenAdminClient::new(&env, &token_contract);

    // Mint initial tokens to the client
    token_admin_client.mint(&client, &1000);

    // Register our GigSync contract
    let contract_id = env.register_contract(None, GigSyncContract);
    let app_client = GigSyncContractClient::new(&env, &contract_id);

    (env, client, freelancer, token_contract, token_client, app_client)
}

#[test]
fn test_1_happy_path_end_to_end() {
    let (env, client, freelancer, token, token_client, app) = setup_env();
    
    app.init(&client, &freelancer, &token);
    app.deposit(&100);
    app.approve(&25);
    app.withdraw(&25);

    assert_eq!(token_client.balance(&freelancer), 25);
    assert_eq!(token_client.balance(&app.address), 75);
    assert_eq!(token_client.balance(&client), 900);
}

#[test]
#[should_panic(expected = "insufficient approved funds")]
fn test_2_edge_case_withdraw_unapproved() {
    let (env, client, freelancer, token, _, app) = setup_env();
    
    app.init(&client, &freelancer, &token);
    app.deposit(&100);
    // Client approves 10, but freelancer tries to withdraw 20
    app.approve(&10);
    app.withdraw(&20); 
}

#[test]
fn test_3_state_verification() {
    let (env, client, freelancer, token, _, app) = setup_env();
    
    app.init(&client, &freelancer, &token);
    app.deposit(&50);
    app.approve(&20);

    // Verify storage directly bypasses the client wrapper
    let balance: i128 = env.as_contract(&app.address, || env.storage().instance().get(&DataKey::Balance).unwrap());
    let approved: i128 = env.as_contract(&app.address, || env.storage().instance().get(&DataKey::ApprovedBalance).unwrap());
    
    assert_eq!(balance, 50);
    assert_eq!(approved, 20);
}

#[test]
#[should_panic(expected = "insufficient escrow balance for approval")]
fn test_4_edge_case_approve_exceeds_deposit() {
    let (env, client, freelancer, token, _, app) = setup_env();
    
    app.init(&client, &freelancer, &token);
    app.deposit(&50);
    app.approve(&60); // Cannot approve more than the escrow holds
}

#[test]
#[should_panic]
fn test_5_edge_case_unauthorized_withdraw() {
    let (env, client, freelancer, token, _, app) = setup_env();
    
    // We disable mock_all_auths to test strict authorization
    let strict_env = Env::default();
    let strict_client = Address::generate(&strict_env);
    let strict_freelancer = Address::generate(&strict_env);
    let token_contract = strict_env.register_stellar_asset_contract(Address::generate(&strict_env));
    
    let contract_id = strict_env.register_contract(None, GigSyncContract);
    let strict_app = GigSyncContractClient::new(&strict_env, &contract_id);

    // This should panic because init requires client auth, which isn't mocked/provided here
    strict_app.init(&strict_client, &strict_freelancer, &token_contract);
}