#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec, Address};

#[contracttype]
#[derive(Clone, Debug)]
pub struct BucketItem {
    id: u64,
    dream: String,
    author: Address,
    timestamp: u64,
    is_done: bool,
}

const BUCKET_DATA: Symbol = symbol_short!("BKTDATA");

#[contract]
pub struct BucketListContract;

#[contractimpl]
impl BucketListContract {

    // Get all bucket list items
    pub fn get_dreams(env: Env) -> Vec<BucketItem> {
        env.storage().instance().get(&BUCKET_DATA).unwrap_or(Vec::new(&env))
    }

    // Add a new dream to your bucket list
    pub fn add_dream(env: Env, author: Address, dream: String) -> String {
        author.require_auth();

        let mut items: Vec<BucketItem> = env.storage().instance().get(&BUCKET_DATA).unwrap_or(Vec::new(&env));

        let item = BucketItem {
            id: env.prng().gen::<u64>(),
            dream,
            author,
            timestamp: env.ledger().timestamp(),
            is_done: false,
        };

        items.push_back(item);
        env.storage().instance().set(&BUCKET_DATA, &items);

        String::from_str(&env, "Dream added to your bucket list!")
    }

    // Mark a dream as completed
    pub fn mark_done(env: Env, caller: Address, id: u64) -> String {
        caller.require_auth();

        let mut items: Vec<BucketItem> = env.storage().instance().get(&BUCKET_DATA).unwrap_or(Vec::new(&env));

        for i in 0..items.len() {
            let item = items.get(i).unwrap();
            if item.id == id {
                if item.author != caller {
                    return String::from_str(&env, "You can only mark your own dreams as done");
                }
                let updated = BucketItem {
                    id: item.id,
                    dream: item.dream,
                    author: item.author,
                    timestamp: item.timestamp,
                    is_done: true,
                };
                items.set(i, updated);
                env.storage().instance().set(&BUCKET_DATA, &items);
                return String::from_str(&env, "Dream marked as completed!");
            }
        }

        String::from_str(&env, "Dream not found")
    }

    // Delete a dream (owner only)
    pub fn delete_dream(env: Env, caller: Address, id: u64) -> String {
        caller.require_auth();

        let mut items: Vec<BucketItem> = env.storage().instance().get(&BUCKET_DATA).unwrap_or(Vec::new(&env));

        for i in 0..items.len() {
            let item = items.get(i).unwrap();
            if item.id == id {
                if item.author != caller {
                    return String::from_str(&env, "You can only delete your own dreams");
                }
                items.remove(i);
                env.storage().instance().set(&BUCKET_DATA, &items);
                return String::from_str(&env, "Dream removed from bucket list");
            }
        }

        String::from_str(&env, "Dream not found")
    }
}

mod test;