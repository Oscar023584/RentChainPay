#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol
};

#[contract]
pub struct RentChainPay;

#[contracttype]
#[derive(Clone)]
pub struct RentRecord {
    tenant: Address,
    landlord: Address,
    amount: i128,
    confirmed: bool,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Payment(Symbol),
}

const PAID: Symbol = symbol_short!("PAID");
const CONFIRM: Symbol = symbol_short!("CONFIRM");

#[contractimpl]
impl RentChainPay {

    // Tenant initiates rent payment
    pub fn pay_rent(env: Env, id: Symbol, tenant: Address, landlord: Address, amount: i128) {
        tenant.require_auth();

        let record = RentRecord {
            tenant: tenant.clone(),
            landlord: landlord.clone(),
            amount,
            confirmed: false,
        };

        env.storage().instance().set(&DataKey::Payment(id.clone()), &record);

        env.events().publish((PAID, id), amount);
    }

    // Landlord confirms payment
    pub fn confirm_payment(env: Env, id: Symbol, landlord: Address) {
        landlord.require_auth();

        let mut record: RentRecord = env
            .storage()
            .instance()
            .get(&DataKey::Payment(id.clone()))
            .expect("Payment not found");

        if record.landlord != landlord {
            panic!("Unauthorized landlord");
        }

        record.confirmed = true;

        env.storage().instance().set(&DataKey::Payment(id.clone()), &record);

        env.events().publish((CONFIRM, id), record.amount);
    }

    // Check payment status
    pub fn get_status(env: Env, id: Symbol) -> bool {
        let record: RentRecord = env
            .storage()
            .instance()
            .get(&DataKey::Payment(id))
            .expect("Not found");

        record.confirmed
    }
}