//! Definition of the Events used in the contract
use soroban_sdk::{contracttype, symbol_short, Env};

// INITIALIZED
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitializedEvent {
    pub state: bool,
    pub end_timestamp: u64
}

pub(crate) fn initialized(e: &Env, state: bool, end_timestamp: u64) {
    
    let event: InitializedEvent = InitializedEvent {
        state: state,
        end_timestamp,
    };
    e.events().publish(("AddLiquidityTimeLock", symbol_short!("init")), event);
}