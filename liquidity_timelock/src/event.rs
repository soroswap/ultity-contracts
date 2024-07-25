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

// ADD LIQUIDITY EVENT
#[contracttype] 
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AddLiquidityEvent {
    pub token_a: Address,
    pub token_b: Address,
    pub pair: Address,
    pub amount_a: i128,
    pub amount_b: i128,
    pub liquidity: i128,
    pub to: Address
}

/// Publishes an `AddLiquidityEvent` to the event stream.
/// 
/// # Arguments
/// 
/// * `e` - An instance of the `Env` struct.
/// * `token_a` - The address of the first token in the liquidity pair.
/// * `token_b` - The address of the second token in the liquidity pair.
/// * `pair` - The address of the liquidity pair.
/// * `amount_a` - The amount of `token_a` to add to the liquidity.
/// * `amount_b` - The amount of `token_b` to add to the liquidity.
/// * `liquidity` - The amount of liquidity tokens minted.
/// * `to` - The address to receive the liquidity tokens.
pub(crate) fn add_liquidity(
    e: &Env,
    token_a: Address,
    token_b: Address,
    pair: Address,
    amount_a: i128,
    amount_b: i128,
    liquidity: i128,
    to: Address,
) {
    let event = AddLiquidityEvent {
        token_a,
        token_b,
        pair,
        amount_a,
        amount_b,
        liquidity,
        to,
    };

    e.events().publish(("LiquidityTimeLock", symbol_short!("add")), event);
}