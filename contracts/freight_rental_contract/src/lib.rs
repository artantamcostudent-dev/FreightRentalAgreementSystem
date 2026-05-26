#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Env, Symbol, String, Vec, Map
};

// Storage key for agreements
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Agreement(u64),   // stores each agreement by ID
    Counter           // keeps track of total agreements
}

// Agreement structure
#[contracttype]
#[derive(Clone)]
pub struct Agreement {
    pub id: u64,
    pub client: String,
    pub cargo: String,
    pub route: String,
    pub fee: u64,
    pub date: String,
    pub timestamp: u64,
    pub paid: bool,
}

// Contract definition
#[contract]
pub struct FreightContract;

#[contractimpl]
impl FreightContract {

    // 📄 Create a new freight rental agreement
    pub fn create_agreement(
        env: Env,
        client: String,
        cargo: String,
        route: String,
        fee: u64,
        date: String
    ) -> u64 {

        // Get current counter or default to 0
        let mut count: u64 = env.storage().instance()
            .get(&DataKey::Counter)
            .unwrap_or(0);

        count += 1;

        // Create agreement
        let agreement = Agreement {
            id: count,
            client,
            cargo,
            route,
            fee,
            date,
            timestamp: env.ledger().timestamp(),
            paid: false,
        };

        // Store agreement permanently
        env.storage().instance().set(&DataKey::Agreement(count), &agreement);

        // Update counter
        env.storage().instance().set(&DataKey::Counter, &count);

        count
    }

    // 🔍 View agreement details
    pub fn get_agreement(env: Env, id: u64) -> Agreement {
        env.storage().instance()
            .get(&DataKey::Agreement(id))
            .unwrap()
    }

    // 💳 Mark agreement as paid
    pub fn mark_paid(env: Env, id: u64) {
        let mut agreement: Agreement = env.storage().instance()
            .get(&DataKey::Agreement(id))
            .unwrap();

        agreement.paid = true;

        env.storage().instance().set(&DataKey::Agreement(id), &agreement);
    }

    // ✅ Verify agreement exists (basic integrity check)
    pub fn verify_agreement(env: Env, id: u64) -> bool {
        env.storage().instance().has(&DataKey::Agreement(id))
    }
}