#![cfg(test)]
extern crate std;
pub mod soroswap_setup;

use soroban_sdk::{
    Env, 
    vec,
    Vec,
    BytesN, 
    Address, 
    String,
    testutils::{
        Address as _,
        Ledger,
    },
};
use crate::{AddLiquidityTimelock, AddLiquidityTimelockClient};
use soroswap_setup::{SoroswapTest, router};
use router::SoroswapRouterClient;

// SoroswapAggregatorProxy Contract
fn create_add_liquidity_timelock<'a>(e: &Env) -> AddLiquidityTimelockClient<'a> {
    AddLiquidityTimelockClient::new(e, &e.register_contract(None, AddLiquidityTimelock {}))
}

pub struct AddLiqudityTimelockTest<'a> {
    env: Env,
    timelock_contract: AddLiquidityTimelockClient<'a>,
    router_contract: SoroswapRouterClient<'a>,
    admin: Address,
    token_0: Address,
    token_1: Address,
    user: Address,
    pair_address: Address,
}

impl<'a> AddLiqudityTimelockTest<'a> {
    fn setup() -> Self {
        let test = SoroswapTest::soroswap_setup();
        
        let timelock_contract = create_add_liquidity_timelock(&test.env);
        let router_contract = test.router_contract;

        AddLiqudityTimelockTest {
            env: test.env,
            timelock_contract,
            router_contract,
            admin: test.admin,
            token_0: test.token_0.address,
            token_1: test.token_1.address,
            user: test.user,
            pair_address: test.pair_address,
        }
    }
}

pub mod initialize;
pub mod add_liquidity;
pub mod claim;
// pub mod get_protocols;