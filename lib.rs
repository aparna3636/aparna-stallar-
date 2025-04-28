#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol, symbol_short};

#[contract]
pub struct VotingContract;

#[contractimpl]
impl VotingContract {
    // Function to cast a vote for a candidate
    pub fn vote(env: Env, candidate: Symbol) {
        let key = candidate.clone();
        let count: u32 = env.storage().instance().get(&key).unwrap_or(0);
        env.storage().instance().set(&key, &(count + 1));
    }

    // Function to get total votes for a candidate
    pub fn get_votes(env: Env, candidate: Symbol) -> u32 {
        env.storage().instance().get(&candidate).unwrap_or(0)
    }
}
