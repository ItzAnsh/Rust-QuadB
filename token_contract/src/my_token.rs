// Declare dependencies and libraries
use ic_cdk_macros::*;
use ic_cdk::storage;
use ic_cdk::export::candid::{CandidType, Principal};

// Define token structure and storage
#[derive(Clone, CandidType)]
struct Token {
    owner: Principal,
    balance: u64,
}

impl Token {
    fn new(owner: Principal, initial_balance: u64) -> Self {
        Self {
            owner,
            balance: initial_balance,
        }
    }

    fn transfer(&mut self, to: Principal, amount: u64) -> Result<(), String> {
        if self.balance >= amount {
            self.balance -= amount;
            // Simulated transfer logic, adjust as per actual ICP APIs
            Ok(())
        } else {
            Err("Insufficient balance".to_string())
        }
    }

    fn get_balance(&self) -> u64 {
        self.balance
    }
}

// Declare storage for the token
#[init]
fn init() -> Token {
    Token::new(ic_cdk::caller(), 1000)  // Initialize token with 1000 tokens for the caller
}

// Define update functions (entry points for transactions)
#[update]
fn transfer(to: Principal, amount: u64) -> Result<(), String> {
    let mut token = storage::get_mut::<Token>();
    token.transfer(to, amount)
}
