use soroban_sdk::{Address, testutils::Address as _};

use crate::error::ContractError;
use crate::test::{AddLiqudityTimelockTest};

#[test]
fn add_liquidity_test() {
    let test = AddLiqudityTimelockTest::setup();
    let deadline = 1746885472;
    test.timelock_contract.initialize(
      &test.admin, 
      &test.router_contract.address,
      &deadline
    );
    let result = test.timelock_contract.add_liquidity(
      &test.token_0,
      &test.token_1,
      &10_000_000,
      &10_000_000,
      &test.admin,
      &deadline
    );
}

#[test]
fn add_negative_liquidity_test() {
  let test = AddLiqudityTimelockTest::setup();
  let deadline = 1746885472;
  test.timelock_contract.initialize(
    &test.admin, 
    &test.router_contract.address,
    &deadline
  );
  let result = test.timelock_contract.add_liquidity(
    &test.token_0,
    &test.token_1,
    &(-10_000_000),
    &(-10_000_000),
    &test.admin,
    &deadline
  );
}

#[test]
fn add_liquidity_with_expired_deadline_test() {
  let test = AddLiqudityTimelockTest::setup();
  let deadline = 1746885472;
  test.timelock_contract.initialize(
    &test.admin, 
    &test.router_contract.address,
    &deadline
  );
  let result = test.timelock_contract.add_liquidity(
    &test.token_0,
    &test.token_1,
    &10_000_000,
    &10_000_000,
    &test.admin,
    &(deadline - 1)
  );
}

#[test]
fn add_liquidity_with_uninitialized_contract_test() {
  let test = AddLiqudityTimelockTest::setup();
  let deadline = 1746885472;
  let result = test.timelock_contract.add_liquidity(
    &test.token_0,
    &test.token_1,
    &10_000_000,
    &10_000_000,
    &test.admin,
    &deadline
  );
}
