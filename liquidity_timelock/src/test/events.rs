use soroban_sdk::{symbol_short, testutils::Events, vec, Address, IntoVal, String, Vec};

use crate::test::{
    AddLiqudityTimelockTest
};

use crate::event::{
    InitializedEvent,
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
