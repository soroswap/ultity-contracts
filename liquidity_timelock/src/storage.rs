use crate::error::{CombinedLiquidityTimelockError, LiquidityTimelockError};
use soroban_sdk::{contracttype, Address, Env};

#[derive(Clone)]
#[contracttype]

enum DataKey {
    Initialized,
    Admin,
    RouterAddress,
    EndTime,
}

const DAY_IN_LEDGERS: u32 = 17280;
const INSTANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

pub fn extend_instance_ttl(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}

pub fn set_initialized(e: &Env) {
    e.storage().instance().set(&DataKey::Initialized, &true);
}

pub fn is_initialized(e: &Env) -> bool {
    e.storage().instance().has(&DataKey::Initialized)
}

pub fn set_admin(e: &Env, admin: Address) {
    e.storage().instance().set(&DataKey::Admin, &admin);
}
pub fn get_admin(e: &Env) -> Result<Address, CombinedLiquidityTimelockError> {
    e.storage()
        .instance()
        .get(&DataKey::Admin)
        .ok_or(LiquidityTimelockError::NotInitialized.into())
}

pub fn set_soroswap_router_address(e: &Env, address: Address) {
    e.storage()
        .instance()
        .set(&DataKey::RouterAddress, &address);
}

pub fn get_router_address(e: &Env) -> Result<Address, CombinedLiquidityTimelockError> {
    e.storage()
        .instance()
        .get(&DataKey::RouterAddress)
        .ok_or(LiquidityTimelockError::NotInitialized.into())
}

pub fn set_end_timestamp(e: &Env, timestamp: u64) {
    e.storage().instance().set(&DataKey::EndTime, &timestamp);
}

pub fn get_end_timestamp(e: &Env) -> Result<u64, CombinedLiquidityTimelockError> {
    e.storage()
        .instance()
        .get(&DataKey::EndTime)
        .ok_or(LiquidityTimelockError::NotInitialized.into())
}
