extern crate chrono;

use crate::test::{AddLiqudityTimelockTest, SoroswapPairClient};
use crate::error::CombinedLiquidityTimelockError; 


use soroban_sdk::{
  testutils::{
      Ledger},
  };


#[test]
fn claim_test_neet_to_wait() {
  let test = AddLiqudityTimelockTest::setup();

  let ledger_timestamp = 100;
  let end_timestamp = 1000;
  assert!(end_timestamp > ledger_timestamp);
  test.env.ledger().with_mut(|li| {
      li.timestamp = ledger_timestamp;
  });

  test.timelock_contract.initialize(
    &test.admin, 
    &test.router_contract.address,
    &end_timestamp
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
    &end_timestamp
  );

  assert_eq!(added_token_0, amount_0);
  assert_eq!(added_token_1, amount_1);
  assert_eq!(added_liquidity, expected_liquidity);
  // // END OF ADDING LIQUIDITY

  let pair_address = test.soroswap_factory_contract.get_pair(&test.token_0.address, &test.token_1.address);


  // If the current time is still less than end_timestamp, we will have an error
  let result = test.timelock_contract.try_claim(
    &pair_address
  );

  assert_eq!(result, Err(Ok(CombinedLiquidityTimelockError::TimelockNeedToWait)));
}


#[test]
fn claim_test() {
  let test = AddLiqudityTimelockTest::setup();

  let ledger_timestamp = 100;
  let end_timestamp = 1000;
  assert!(end_timestamp > ledger_timestamp);

  test.env.ledger().with_mut(|li| {
      li.timestamp = ledger_timestamp;
  });

  test.timelock_contract.initialize(
    &test.admin, 
    &test.router_contract.address,
    &end_timestamp
  );

  let amount_0: i128 = 1_000_000_000_000;
  let amount_1: i128 = 4_000_000_000_000;
  let expected_liquidity: i128 = 2_000_000_000_000;


  let initial_user_balance_0 = test.token_0.balance(&test.user);
  let initial_user_balance_1 = test.token_1.balance(&test.user);
  let initial_admin_balance_0 = test.token_0.balance(&test.admin);
  let initial_admin_balance_1 = test.token_1.balance(&test.admin);

  // but the contract DOES holds the LP token
  let pair_address = test.soroswap_factory_contract.get_pair(&test.token_0.address, &test.token_1.address);
  let pair_client = SoroswapPairClient::new(&test.env, &pair_address);

  let initial_user_lp_token_balance = pair_client.balance(&test.user);
  let initial_admin_lp_token_balance = pair_client.balance(&test.admin);

  let (added_token_0, added_token_1, added_liquidity) = test.timelock_contract.add_liquidity(
    &test.token_0.address,
    &test.token_1.address,
    &amount_0,
    &amount_1,
    &0,
    &0,
    &test.user,
    &end_timestamp
  );

  assert_eq!(added_token_0, amount_0);
  assert_eq!(added_token_1, amount_1);
  assert_eq!(added_liquidity, expected_liquidity);

  assert_eq!(test.token_0.balance(&test.user), initial_user_balance_0.checked_sub(amount_0).unwrap());
  assert_eq!(test.token_1.balance(&test.user), initial_user_balance_1.checked_sub(amount_1).unwrap());
  assert_eq!(test.token_0.balance(&test.admin), initial_admin_balance_0);
  assert_eq!(test.token_1.balance(&test.admin), initial_admin_balance_1);

  // the contract does not hold any token...
  assert_eq!(test.token_0.balance(&test.timelock_contract.address), 0);
  assert_eq!(test.token_1.balance(&test.timelock_contract.address), 0);

  // however the cpontract DOES hold the LP token
  
  assert_eq!(pair_client.balance(&test.timelock_contract.address), expected_liquidity);

  // and the user does NOT hold any NEW amount of LP token
  assert_eq!(pair_client.balance(&test.user), initial_user_lp_token_balance);



  let new_ledget_timestamp = end_timestamp + 1;

  test.env.ledger().with_mut(|li| {
    li.timestamp = new_ledget_timestamp;
  });

  // NOW WE WILL CLAIM
  test.timelock_contract.claim(
    &pair_address
  );

  // token balances do not change
  assert_eq!(test.token_0.balance(&test.user), initial_user_balance_0.checked_sub(amount_0).unwrap());
  assert_eq!(test.token_1.balance(&test.user), initial_user_balance_1.checked_sub(amount_1).unwrap());
  assert_eq!(test.token_0.balance(&test.timelock_contract.address), 0);
  assert_eq!(test.token_1.balance(&test.timelock_contract.address), 0);

  // LP tokens are now moved to the ADMIN
  assert_eq!(pair_client.balance(&test.timelock_contract.address), 0);
  assert_eq!(pair_client.balance(&test.user), initial_user_lp_token_balance);
  assert_eq!(pair_client.balance(&test.admin), initial_admin_lp_token_balance+expected_liquidity);

  // admin now can withdraw the liquidity
  let (withdrawn_token_0, withdrawn_token_1) = test.router_contract.remove_liquidity(
    &test.token_0.address,
    &test.token_1.address,
    &expected_liquidity,
    &0,
    &0,
    &test.admin,
    &(new_ledget_timestamp+1)
  );

  // withdrawn amounts are the same when deposited
  assert_eq!(withdrawn_token_0, amount_0);
  assert_eq!(withdrawn_token_1, amount_1);
  
  // admin now has more tokens
  assert_eq!(test.token_0.balance(&test.admin), initial_admin_balance_0.checked_add(amount_0).unwrap());
  assert_eq!(test.token_1.balance(&test.admin), initial_admin_balance_1.checked_add(amount_1).unwrap());

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