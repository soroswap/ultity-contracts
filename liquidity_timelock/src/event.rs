//! Definition of the Events used in the contract
use soroban_sdk::{contracttype, symbol_short, Env, Address};

// INITIALIZED
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitializedEvent {
    pub admin: Address,
    pub router_address: Address,
    pub end_timestamp: u64
}

pub(crate) fn initialized(
    e: &Env,
    admin: Address,
    router_address: Address,
    end_timestamp: u64) {
    
    let event: InitializedEvent = InitializedEvent {
        admin,
        router_address,
        end_timestamp,

    };
    e.events().publish(("LiquidityTimeLock", symbol_short!("init")), event);
}