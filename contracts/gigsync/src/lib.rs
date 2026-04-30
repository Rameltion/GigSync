
#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, token};

// Define storage keys for contract state
#[contracttype]
pub enum DataKey {
    Client,
    Freelancer,
    Token,
    Balance,
    ApprovedBalance,
}

#[contract]
pub struct GigSyncContract;

#[contractimpl]
impl GigSyncContract {
    /// Initializes the escrow with the client, freelancer, and the token (e.g., USDC) to be used.
    pub fn init(env: Env, client: Address, freelancer: Address, token: Address) {
        client.require_auth();
        env.storage().instance().set(&DataKey::Client, &client);
        env.storage().instance().set(&DataKey::Freelancer, &freelancer);
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::Balance, &0_i128);
        env.storage().instance().set(&DataKey::ApprovedBalance, &0_i128);
    }

    /// Client deposits funds into the smart contract escrow.
    pub fn deposit(env: Env, amount: i128) {
        let client: Address = env.storage().instance().get(&DataKey::Client).unwrap();
        client.require_auth();
        
        let token: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        let token_client = token::Client::new(&env, &token);
        
        // Transfer funds from client to the contract
        token_client.transfer(&client, &env.current_contract_address(), &amount);
        
        // Update stored balance
        let mut balance: i128 = env.storage().instance().get(&DataKey::Balance).unwrap();
        balance += amount;
        env.storage().instance().set(&DataKey::Balance, &balance);
    }

    /// Client approves a specific micro-milestone amount for the freelancer to withdraw.
    pub fn approve(env: Env, amount: i128) {
        let client: Address = env.storage().instance().get(&DataKey::Client).unwrap();
        client.require_auth(); // Only the client can approve milestones
        
        let mut approved: i128 = env.storage().instance().get(&DataKey::ApprovedBalance).unwrap();
        let balance: i128 = env.storage().instance().get(&DataKey::Balance).unwrap();
        
        // Ensure approval doesn't exceed total deposited funds
        if approved + amount > balance {
            panic!("insufficient escrow balance for approval");
        }
        
        approved += amount;
        env.storage().instance().set(&DataKey::ApprovedBalance, &approved);
    }

    /// Freelancer withdraws their approved funds to their own wallet.
    pub fn withdraw(env: Env, amount: i128) {
        let freelancer: Address = env.storage().instance().get(&DataKey::Freelancer).unwrap();
        freelancer.require_auth(); // Only the freelancer can withdraw
        
        let mut approved: i128 = env.storage().instance().get(&DataKey::ApprovedBalance).unwrap();
        
        // Ensure freelancer isn't withdrawing more than approved
        if amount > approved {
            panic!("insufficient approved funds");
        }
        
        // Deduct from internal accounting
        approved -= amount;
        env.storage().instance().set(&DataKey::ApprovedBalance, &approved);

        let mut balance: i128 = env.storage().instance().get(&DataKey::Balance).unwrap();
        balance -= amount;
        env.storage().instance().set(&DataKey::Balance, &balance);

        // Execute the on-chain transfer from contract to freelancer
        let token: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&env.current_contract_address(), &freelancer, &amount);
    }
}

mod test;