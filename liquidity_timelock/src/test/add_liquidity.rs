extern crate chrono;
use crate::test::AddLiqudityTimelockTest;
use crate::error::CombinedLiquidityTimelockError;

use chrono::{Duration, Utc};


use soroban_sdk::{
  Address,
  testutils::{
      Address as _, 
      MockAuth,
      MockAuthInvoke,
      Ledger},
  vec,
  IntoVal};



#[test]
fn test_add_liquidity_not_yet_initialized() {
    let test = AddLiqudityTimelockTest::setup();
    let result = test.timelock_contract.try_add_liquidity(
        &test.token_0.address,  // token_a: Address,
        &test.token_1.address,  // token_b: Address,
        &10000,                 // amount_a_desired: i128,
        &10000,                 // amount_b_desired: i128,
        &0,                     // amount_a_min: i128,
        &0,                     // amount_b_min: i128,
        &test.user,             // to: Address,
        &0,                     // deadline: u64,
    );
    assert_eq!(result, Err(Ok(CombinedLiquidityTimelockError::TimelockNotInitialized)));
}


#[test]
fn test_add_liquidity_amount_a_desired_negative() {
    let test = AddLiqudityTimelockTest::setup();
    
    let ledger_timestamp = 100;
    let release_time = 1000;
    assert!(release_time > ledger_timestamp);
    test.env.ledger().with_mut(|li| {
        li.timestamp = ledger_timestamp;
    });

    test.timelock_contract.initialize(
      &test.admin, 
      &test.router_contract.address,
      &release_time
    );
    
    let result = test.timelock_contract.try_add_liquidity(
        &test.token_0.address,  // token_a: Address,
        &test.token_1.address,  // token_b: Address,
        &-1,                    // amount_a_desired: i128,
        &10000,                 // amount_b_desired: i128,
        &0,                     // amount_a_min: i128,
        &0,                     // amount_b_min: i128,
        &test.user,             // to: Address,
        &0,                     // deadline: u64,
    );
    assert_eq!(result, Err(Ok(CombinedLiquidityTimelockError::TimelockNegativeNotAllowed)));
}

#[test]
fn test_add_liquidity_amount_b_desired_negative() {
    let test = AddLiqudityTimelockTest::setup();
    
    let ledger_timestamp = 100;
    let release_time = 1000;
    assert!(release_time > ledger_timestamp);
    test.env.ledger().with_mut(|li| {
        li.timestamp = ledger_timestamp;
    });

    test.timelock_contract.initialize(
      &test.admin, 
      &test.router_contract.address,
      &release_time
    );
    
    let result = test.timelock_contract.try_add_liquidity(
        &test.token_0.address,  // token_a: Address,
        &test.token_1.address,  // token_b: Address,
        &10000,                 // amount_a_desired: i128,
        &-1,                    // amount_b_desired: i128,
        &0,                     // amount_a_min: i128,
        &0,                     // amount_b_min: i128,
        &test.user,             // to: Address,
        &0,                     // deadline: u64,
    );
    assert_eq!(result, Err(Ok(CombinedLiquidityTimelockError::TimelockNegativeNotAllowed)));
}

#[test]
fn test_add_liquidity_amount_a_min_negative() {
    let test = AddLiqudityTimelockTest::setup();
    
    let ledger_timestamp = 100;
    let release_time = 1000;
    assert!(release_time > ledger_timestamp);
    test.env.ledger().with_mut(|li| {
        li.timestamp = ledger_timestamp;
    });

    test.timelock_contract.initialize(
      &test.admin, 
      &test.router_contract.address,
      &release_time
    );
    
    let result = test.timelock_contract.try_add_liquidity(
        &test.token_0.address,  // token_a: Address,
        &test.token_1.address,  // token_b: Address,
        &10000,                 // amount_a_desired: i128,
        &10000,                 // amount_b_desired: i128,
        &-1,                    // amount_a_min: i128,
        &0,                     // amount_b_min: i128,
        &test.user,             // to: Address,
        &0,                     // deadline: u64,
    );
    assert_eq!(result, Err(Ok(CombinedLiquidityTimelockError::TimelockNegativeNotAllowed)));
}

#[test]
fn test_add_liquidity_amount_b_min_negative() {
    let test = AddLiqudityTimelockTest::setup();
    
    let ledger_timestamp = 100;
    let release_time = 1000;
    assert!(release_time > ledger_timestamp);
    test.env.ledger().with_mut(|li| {
        li.timestamp = ledger_timestamp;
    });

    test.timelock_contract.initialize(
      &test.admin, 
      &test.router_contract.address,
      &release_time
    );
    
    let result = test.timelock_contract.try_add_liquidity(
        &test.token_0.address,  // token_a: Address,
        &test.token_1.address,  // token_b: Address,
        &10000,                 // amount_a_desired: i128,
        &10000,                 // amount_b_desired: i128,
        &0,                     // amount_a_min: i128,
        &-1,                    // amount_b_min: i128,
        &test.user,             // to: Address,
        &0,                     // deadline: u64,
    );
    assert_eq!(result, Err(Ok(CombinedLiquidityTimelockError::TimelockNegativeNotAllowed)));
}


#[test]
fn test_add_liquidity_deadline_expired() {
    let test = AddLiqudityTimelockTest::setup();
    

    let ledger_timestamp = 100;
    let release_time = 1000;
    assert!(release_time > ledger_timestamp);
    test.env.ledger().with_mut(|li| {
        li.timestamp = ledger_timestamp;
    });

    test.timelock_contract.initialize(
      &test.admin, 
      &test.router_contract.address,
      &release_time
    );
    

    let alice = Address::generate(&test.env);
    let bob = Address::generate(&test.env);
    // alice is not equal to bob
    assert_ne!(alice, bob);

    let ledger_timestamp = 100;
    let deadline_for_adding_liquidity = 90;

    assert!(deadline_for_adding_liquidity < ledger_timestamp);

    test.env.ledger().with_mut(|li| {
        li.timestamp = ledger_timestamp;
    });

    // /*
    //     Here we test the case when the deadline has passed
    //  */
    let result = test.timelock_contract.try_add_liquidity(
        &test.token_0.address, //     token_a: Address,
        &test.token_1.address, //     token_b: Address,
        &0, //     amount_a_desired: i128,
        &0, //     amount_b_desired: i128,
        &0, //     amount_a_min: i128,
        &0 , //     amount_b_min: i128,
        &bob, //     to: Address,
        &deadline_for_adding_liquidity, //     deadline: u64,
    );
    assert_eq!(result, Err(Ok(CombinedLiquidityTimelockError::TimelockDeadlineExpired)));
}


// insufficient ammount (a and b)

// Pub function that will be used in other tests:
pub fn add_liquidity(
  test: &AddLiqudityTimelockTest, 
  amount_0: &i128,
  amount_1: &i128) -> (i128, i128, i128){

  let ledger_timestamp = 100;
  let desired_deadline = 1000;
  assert!(desired_deadline > ledger_timestamp);
  test.env.ledger().with_mut(|li| {
      li.timestamp = ledger_timestamp;
  });


      test.env.budget().reset_unlimited();
      test.timelock_contract.add_liquidity(
          &test.token_0.address, //     token_a: Address,
          &test.token_1.address, //     token_b: Address,
          &amount_0, //     amount_a_desired: i128,
          &amount_1, //     amount_b_desired: i128,
          &0, //     amount_a_min: i128,
          &0 , //     amount_b_min: i128,
          &test.user, //     to: Address,
          &desired_deadline//     deadline: u64,
      )

}

#[test]
fn insufficient_b_amount() {
    let test = AddLiqudityTimelockTest::setup();
    
    let ledger_timestamp = 100;
    let release_time = 1000;
    assert!(release_time > ledger_timestamp);
    test.env.ledger().with_mut(|li| {
        li.timestamp = ledger_timestamp;
    });

    test.timelock_contract.initialize(
      &test.admin, 
      &test.router_contract.address,
      &release_time
    );

    
    let ledger_timestamp = 100;
    let desired_deadline = 1000;

    assert!(desired_deadline > ledger_timestamp);

    test.env.ledger().with_mut(|li| {
        li.timestamp = ledger_timestamp;
    });

    let amount_0: i128 = 1_000_000_000_000;
    let amount_1: i128 = 4_000_000_000_000;
    
    add_liquidity(&test, &amount_0, &amount_1);
    
    // We can provide liquidity again and should not panic
    let result = test.timelock_contract.try_add_liquidity(
        &test.token_0.address,    // token_a: Address,
        &test.token_1.address,    // token_b: Address,
        &amount_0,                // amount_a_desired: i128,
        &amount_1,                // amount_b_desired: i128,
        &(amount_0),              // amount_a_min: i128,
        &(amount_1 + 1),          // amount_b_min: i128,
        &test.user,               // to: Address,
        &desired_deadline,        // deadline: u64,
    );
    assert_eq!(result, Err(Ok(CombinedLiquidityTimelockError::TimelockInsufficientBAmount)));
}

#[test]
fn insufficient_a_amount() {
    let test = AddLiqudityTimelockTest::setup();
    
    let ledger_timestamp = 100;
    let release_time = 1000;
    assert!(release_time > ledger_timestamp);
    test.env.ledger().with_mut(|li| {
        li.timestamp = ledger_timestamp;
    });

    test.timelock_contract.initialize(
      &test.admin, 
      &test.router_contract.address,
      &release_time
    );

    
    let ledger_timestamp = 100;
    let desired_deadline = 1000;

    assert!(desired_deadline > ledger_timestamp);

    test.env.ledger().with_mut(|li| {
        li.timestamp = ledger_timestamp;
    });

    let amount_0: i128 = 1_000_000_000_000;
    let amount_1: i128 = 4_000_000_000_000;
    
    add_liquidity(&test, &amount_0, &amount_1);
    
    // We can provide liquidity again and should not panic
    let result = test.timelock_contract.try_add_liquidity(
        &test.token_0.address,    // token_a: Address,
        &test.token_1.address,    // token_b: Address,
        &(amount_0 + 1),          // amount_a_desired: i128,
        &amount_1,                // amount_b_desired: i128,
        &(amount_0 + 1),          // amount_a_min: i128,
        &amount_1,                // amount_b_min: i128,
        &test.user,               // to: Address,
        &desired_deadline,        // deadline: u64,
    );
    assert_eq!(result, Err(Ok(CombinedLiquidityTimelockError::TimelockInsufficientAAmount)));
}


#[test]
fn add_liquidity_test() {
    let test = AddLiqudityTimelockTest::setup();

    let ledger_timestamp = 100;
    let release_time = 1000;
    assert!(release_time > ledger_timestamp);
    test.env.ledger().with_mut(|li| {
        li.timestamp = ledger_timestamp;
    });

    test.timelock_contract.initialize(
      &test.admin, 
      &test.router_contract.address,
      &release_time
    );

    let amount_0: i128 = 1_000_000_000_000;
    let amount_1: i128 = 4_000_000_000_000;
    let expected_liquidity: i128 = 2_000_000_000_000;

    let (added_token_0, added_token_1, added_liquidity) = test.timelock_contract.add_liquidity(
      &test.token_0.address,
      &test.token_1.address,
      &amount_0,
      &amount_1,
      &0,
      &0,
      &test.admin,
      &release_time
    );

    assert_eq!(added_token_0, amount_0);
    assert_eq!(added_token_1, amount_1);
    assert_eq!(added_liquidity, expected_liquidity);
}