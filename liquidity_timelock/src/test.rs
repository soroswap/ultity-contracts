#![cfg(test)]
extern crate std;
pub mod soroswap_setup;

use soroban_sdk::{
    Env, 
    Address, 
};
use crate::{AddLiquidityTimelock, AddLiquidityTimelockClient};
use soroswap_setup::{SoroswapTest, router, factory, TokenClient};
use router::SoroswapRouterClient;
use factory::SoroswapFactoryClient;

pub use soroswap_setup::SoroswapPairClient;



// SoroswapAggregatorProxy Contract
fn create_add_liquidity_timelock<'a>(e: &Env) -> AddLiquidityTimelockClient<'a> {
    AddLiquidityTimelockClient::new(e, &e.register_contract(None, AddLiquidityTimelock {}))
}

pub struct AddLiqudityTimelockTest<'a> {
    env: Env,
    timelock_contract: AddLiquidityTimelockClient<'a>,
    router_contract: SoroswapRouterClient<'a>,
    soroswap_factory_contract: SoroswapFactoryClient<'a>,
    admin: Address,
    token_0: TokenClient<'a>,
    token_1: TokenClient<'a>,
    pair_address: Address,
    user: Address,
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
            soroswap_factory_contract: test.factory_contract,
            admin: test.admin,
            token_0: test.token_0,
            token_1: test.token_1,
            pair_address: test.pair_address,
            user: test.user,
        }
    }
}

pub mod initialize;
pub mod add_liquidity;
pub mod claim;
pub mod events;
// pub mod get_protocols;