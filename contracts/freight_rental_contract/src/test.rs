#![cfg(test)]

use super::*;
use soroban_sdk::{Env, String};

#[test]
fn test_happy_path_create_and_verify() {
    let env = Env::default();
    let contract_id = env.register_contract(None, FreightContract);
    let client = FreightContractClient::new(&env, &contract_id);

    let id = client.create_agreement(
        &String::from_str(&env, "ABC Corp"),
        &String::from_str(&env, "Electronics"),
        &String::from_str(&env, "Manila-Cebu"),
        &1000,
        &String::from_str(&env, "2026-06-01"),
    );

    assert!(client.verify_agreement(&id));
}

#[test]
fn test_edge_case_invalid_id() {
    let env = Env::default();
    let contract_id = env.register_contract(None, FreightContract);
    let client = FreightContractClient::new(&env, &contract_id);

    // ID does not exist
    assert!(!client.verify_agreement(&999));
}

#[test]
fn test_state_verification() {
    let env = Env::default();
    let contract_id = env.register_contract(None, FreightContract);
    let client = FreightContractClient::new(&env, &contract_id);

    let id = client.create_agreement(
        &String::from_str(&env, "XYZ Corp"),
        &String::from_str(&env, "Food"),
        &String::from_str(&env, "Davao-Manila"),
        &2000,
        &String::from_str(&env, "2026-06-02"),
    );

    let agreement = client.get_agreement(&id);

    assert_eq!(agreement.client, String::from_str(&env, "XYZ Corp"));
}

#[test]
fn test_mark_paid() {
    let env = Env::default();
    let contract_id = env.register_contract(None, FreightContract);
    let client = FreightContractClient::new(&env, &contract_id);

    let id = client.create_agreement(
        &String::from_str(&env, "Client A"),
        &String::from_str(&env, "Goods"),
        &String::from_str(&env, "A-B"),
        &500,
        &String::from_str(&env, "2026-06-03"),
    );

    client.mark_paid(&id);
    let agreement = client.get_agreement(&id);

    assert!(agreement.paid);
}

#[test]
fn test_multiple_agreements() {
    let env = Env::default();
    let contract_id = env.register_contract(None, FreightContract);
    let client = FreightContractClient::new(&env, &contract_id);

    let id1 = client.create_agreement(
        &String::from_str(&env, "A"),
        &String::from_str(&env, "Cargo1"),
        &String::from_str(&env, "Route1"),
        &100,
        &String::from_str(&env, "2026-06-01"),
    );

    let id2 = client.create_agreement(
        &String::from_str(&env, "B"),
        &String::from_str(&env, "Cargo2"),
        &String::from_str(&env, "Route2"),
        &200,
        &String::from_str(&env, "2026-06-02"),
    );

    assert!(id2 > id1);
}