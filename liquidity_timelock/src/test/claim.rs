use soroban_sdk::{Address, testutils::Address as _};

use crate::error::ContractError;
use crate::test::AddLiqudityTimelockTest;

#[test]
fn claim_test() {
  let test = AddLiqudityTimelockTest::setup();
  let deadline = 1746885472;
  test.timelock_contract.initialize(
    &test.admin, 
    &test.router_contract.address,
    &deadline
  );
  let result = test.timelock_contract.claim(
    &test.pair_address
  ); 

  assert_eq!(result,());
}

#[test]
fn claim_test_with_expired_deadline() {
  let test = AddLiqudityTimelockTest::setup();
  let deadline = 1746885472;
  test.timelock_contract.initialize(
    &test.admin, 
    &test.router_contract.address,
    &deadline
  );
  let result = test.timelock_contract.claim(
    &test.pair_address
  ); 
  assert_eq!(result,());
}

#[test]
fn claim_test_invalid_pair() {
  let test = AddLiqudityTimelockTest::setup();
  let deadline = 1746885472;
  test.timelock_contract.initialize(
    &test.admin, 
    &test.router_contract.address,
    &deadline
  );
  let result = test.timelock_contract.claim(
    &test.token_0
  ); 
}

#[test]
fn claim_test_invalid_admin() {
  let test = AddLiqudityTimelockTest::setup();
  let deadline = 1746885472;
  test.timelock_contract.initialize(
    &test.user, 
    &test.router_contract.address,
    &deadline
  );
  let result = test.timelock_contract.claim(
    &test.pair_address
  ); 
}