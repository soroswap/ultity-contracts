use soroban_sdk::{symbol_short, testutils::Events, vec, Address, IntoVal, String, Vec};

use crate::test::{
    AddLiqudityTimelockTest
};
use crate::test::add_liquidity::add_liquidity;

use crate::event::{
    InitializedEvent,
    AddLiquidityEvent
    //
};

#[test]
fn initialized_event() {
    let test = AddLiqudityTimelockTest::setup();
    let desired_release_time = 1746885472;

    test.timelock_contract.initialize(
        &test.admin, 
        &test.router_contract.address,
        &desired_release_time
    );
    
    let initialized_event = test.env.events().all().last().unwrap();

    let expected_initialized_event: InitializedEvent = InitializedEvent {
        admin: test.admin.clone(),
        router_address: test.router_contract.address.clone(),
        end_timestamp: desired_release_time,
    };

    assert_eq!(
        vec![&test.env, initialized_event.clone()],
        vec![
            &test.env,
            (
                test.timelock_contract.address.clone(),
                ("LiquidityTimeLock", symbol_short!("init")).into_val(&test.env),
                (expected_initialized_event).into_val(&test.env)
            ),
        ]
    );

    let false_initialized_event:InitializedEvent = InitializedEvent {
        admin: test.admin.clone(),
        router_address: test.router_contract.address.clone(),
        end_timestamp: (desired_release_time+1),
    };
    assert_ne!(
        vec![&test.env, initialized_event.clone()],
        vec![
            &test.env,
            (
                test.timelock_contract.address.clone(),
                ("LiquidityTimeLock", symbol_short!("init")).into_val(&test.env),
                (false_initialized_event).into_val(&test.env)
            ),
        ]
    );

    // Wront symbol_short
    assert_ne!(
        vec![&test.env, initialized_event.clone()],
        vec![
            &test.env,
            (
                test.timelock_contract.address.clone(),
                ("LiquidityTimeLock", symbol_short!("iniit")).into_val(&test.env),
                (expected_initialized_event).into_val(&test.env)
            ),
        ]
    );

    // Wront string
    assert_ne!(
        vec![&test.env, initialized_event.clone()],
        vec![
            &test.env,
            (
                test.timelock_contract.address,
                ("LiquidityTimeLockk", symbol_short!("init")).into_val(&test.env),
                (expected_initialized_event).into_val(&test.env)
            ),
        ]
    );
}


#[test]
fn add_liquidity_event() {
    let test = AddLiqudityTimelockTest::setup();
    let desired_release_time = 1746885472;

    test.timelock_contract.initialize(
        &test.admin, 
        &test.router_contract.address,
        &desired_release_time
    );

    let amount_0: i128 = 1_000_000_000_000;
    let amount_1: i128 = 4_000_000_000_000;

    let (deposited_amount_0, 
        deposited_amount_1, 
        received_liquidity) =add_liquidity(&test, &amount_0, &amount_1);

    let add_liquidity_event = test.env.events().all().last().unwrap();

    let expected_add_liquidity_event: AddLiquidityEvent = AddLiquidityEvent {
        token_a: test.token_0.address.clone(),
        token_b: test.token_1.address.clone(),
        pair: test.pair_address.clone(),
        amount_a: deposited_amount_0.clone(),
        amount_b: deposited_amount_1.clone(),
        liquidity: received_liquidity,
        to: test.user.clone(),
    };

    assert_eq!(
        vec![&test.env, add_liquidity_event.clone()],
        vec![
            &test.env,
            (
                test.timelock_contract.address.clone(),
                ("LiquidityTimeLock", symbol_short!("add")).into_val(&test.env),
                (expected_add_liquidity_event).into_val(&test.env)
            ),
        ]
    );

    let false_add_liquidity_event: AddLiquidityEvent = AddLiquidityEvent {
        token_a: test.token_0.address.clone(),
        token_b: test.token_1.address.clone(),
        pair: test.pair_address.clone(),
        amount_a: deposited_amount_0.clone(),
        amount_b: deposited_amount_1.clone(),
        liquidity: 0, // False value
        to: test.user.clone(),
    };

    assert_ne!(
        vec![&test.env, add_liquidity_event.clone()],
        vec![
            &test.env,
            (
                test.timelock_contract.address.clone(),
                ("LiquidityTimeLock", symbol_short!("add")).into_val(&test.env),
                (false_add_liquidity_event).into_val(&test.env)
            ),
        ]
    );

    // Wrong symbol_short
    assert_ne!(
        vec![&test.env, add_liquidity_event.clone()],
        vec![
            &test.env,
            (
                test.timelock_contract.address.clone(),
                ("LiquidityTimeLock", symbol_short!("addd")).into_val(&test.env),
                (expected_add_liquidity_event).into_val(&test.env)
            ),
        ]
    );

    // Wrong string
    assert_ne!(
        vec![&test.env, add_liquidity_event.clone()],
        vec![
            &test.env,
            (
                test.timelock_contract.address,
                ("LiquidityTimeLockk", symbol_short!("add")).into_val(&test.env),
                (expected_add_liquidity_event).into_val(&test.env)
            ),
        ]
    );
}