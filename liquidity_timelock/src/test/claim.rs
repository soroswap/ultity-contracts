extern crate chrono;

use crate::test::AddLiqudityTimelockTest;
use chrono::{Duration, Utc};

#[test]
fn claim_test() {
  let test = AddLiqudityTimelockTest::setup();

  // ADDING LIQUIDITY FIRST
  let now = Utc::now();
  // END TIMESTAMP SET TO CURRENT TIMESTAMP, IT SHOULD BE ABLE TO CLAIM
  let end_timestamp = now.timestamp() as u64;

  test.timelock_contract.initialize(
    &test.admin, 
    &test.router_contract.address,
    &end_timestamp
  );

  let amount_0: i128 = 1_000_000_000_000;
  let amount_1: i128 = 4_000_000_000_000;
  let expected_liquidity: i128 = 2_000_000_000_000;

  let (added_token_0, added_token_1, added_liquidity) = test.timelock_contract.add_liquidity(
    &test.token_0,
    &test.token_1,
    &amount_0,
    &amount_1,
    &0,
    &0,
    &test.admin,
    &end_timestamp
  );

  assert_eq!(added_token_0, amount_0);
  assert_eq!(added_token_1, amount_1);
  assert_eq!(added_liquidity, expected_liquidity);
  // END OF ADDING LIQUIDITY

  let result = test.timelock_contract.claim(
    &test.pair_address
  ); 

  assert_eq!(result,());
}

// #[test]
// fn claim_test_not_endtime() {
//   let test = AddLiqudityTimelockTest::setup();

//   // ADDING LIQUIDITY FIRST
//   let now = Utc::now();
//   // END TIMESTAMP SET TO CURRENT TIMESTAMP, IT SHOULD BE ABLE TO CLAIM
//   let end_timestamp = (now + Duration::hours(1)).timestamp() as u64;

//   test.timelock_contract.initialize(
//     &test.admin, 
//     &test.router_contract.address,
//     &end_timestamp
//   );

//   let amount_0: i128 = 1_000_000_000_000;
//   let amount_1: i128 = 4_000_000_000_000;
//   let expected_liquidity: i128 = 2_000_000_000_000;

//   let (added_token_0, added_token_1, added_liquidity) = test.timelock_contract.add_liquidity(
//     &test.token_0,
//     &test.token_1,
//     &amount_0,
//     &amount_1,
//     &test.admin,
//     &end_timestamp
//   );

//   assert_eq!(added_token_0, amount_0);
//   assert_eq!(added_token_1, amount_1);
//   assert_eq!(added_liquidity, expected_liquidity);
//   // END OF ADDING LIQUIDITY

//   let result = test.timelock_contract.claim(
//     &test.pair_address
//   ); 

//   assert_eq!(result,());
// }