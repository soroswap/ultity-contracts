//! Definition of the Events used in the contract
use soroban_sdk::{contracttype, symbol_short, Env, Address, Vec, String};

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

// SWAP EVENT
#[contracttype] 
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SwapEvent {
    pub amount_in: i128,
    pub path: Vec<Address>,
    pub to: Address
}

pub(crate) fn swap(
    e: &Env,
    amount_in: i128,
    path: Vec<Address>,
    to: Address
) {
    let event = SwapEvent {
        amount_in,
        path,
        to,
    };

    e.events().publish(("AddLiquidityTimeLock", symbol_short!("swap")), event);
}