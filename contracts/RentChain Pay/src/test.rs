#![cfg(test)]

use super::*;
use soroban_sdk::{Env, Address, symbol_short};

fn setup() -> (Env, Address, Address) {
    let env = Env::default();
    let tenant = Address::generate(&env);
    let landlord = Address::generate(&env);
    (env, tenant, landlord)
}

#[test]
fn test_happy_path_payment_and_confirmation() {
    let (env, tenant, landlord) = setup();
    let contract_id = env.register_contract(None, RentChainPay);
    let client = RentChainPayClient::new(&env, &contract_id);

    client.pay_rent(&symbol_short!("R1"), &tenant, &landlord, &1000);
    client.confirm_payment(&symbol_short!("R1"), &landlord);
}

#[test]
fn test_edge_case_unauthorized_landlord() {
    let (env, tenant, landlord) = setup();
    let fake_landlord = Address::generate(&env);

    let contract_id = env.register_contract(None, RentChainPay);
    let client = RentChainPayClient::new(&env, &contract_id);

    client.pay_rent(&symbol_short!("R2"), &tenant, &landlord, &1000);

    // should panic if wrong landlord confirms
    let result = std::panic::catch_unwind(|| {
        client.confirm_payment(&symbol_short!("R2"), &fake_landlord);
    });

    assert!(result.is_err());
}

#[test]
fn test_state_verification_confirmed() {
    let (env, tenant, landlord) = setup();
    let contract_id = env.register_contract(None, RentChainPay);
    let client = RentChainPayClient::new(&env, &contract_id);

    client.pay_rent(&symbol_short!("R3"), &tenant, &landlord, &1000);
    client.confirm_payment(&symbol_short!("R3"), &landlord);

    let status = client.get_status(&symbol_short!("R3"));
    assert!(status);
}

#[test]
fn test_initial_state_unconfirmed() {
    let (env, tenant, landlord) = setup();
    let contract_id = env.register_contract(None, RentChainPay);
    let client = RentChainPayClient::new(&env, &contract_id);

    client.pay_rent(&symbol_short!("R4"), &tenant, &landlord, &1000);

    let status = client.get_status(&symbol_short!("R4"));
    assert!(!status);
}

#[test]
fn test_payment_record_exists() {
    let (env, tenant, landlord) = setup();
    let contract_id = env.register_contract(None, RentChainPay);
    let client = RentChainPayClient::new(&env, &contract_id);

    client.pay_rent(&symbol_short!("R5"), &tenant, &landlord, &1000);

    let status = client.get_status(&symbol_short!("R5"));
    assert_eq!(status, false);
}