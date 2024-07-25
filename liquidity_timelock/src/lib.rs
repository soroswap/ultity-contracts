#![no_std]
use soroban_sdk::{auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation}, Symbol, contract, contractimpl, Address, Env, vec, Vec, Val, IntoVal};
use soroban_sdk::token::Client as TokenClient;
use soroswap_library::{SoroswapLibraryError as OtherSoroswapLibraryError};


mod event;
mod storage;
mod error;
mod test;

soroban_sdk::contractimport!(
    file = "./soroswap_contracts/soroswap_router.optimized.wasm"
);
pub type SoroswapRouterClient<'a> = Client<'a>;

// SoroswapFactory Contract
mod factory {
    soroban_sdk::contractimport!(file = "./soroswap_contracts/soroswap_factory.optimized.wasm");
    pub type SoroswapFactoryClient<'a> = Client<'a>;
}
use factory::SoroswapFactoryClient;

use storage::{
    extend_instance_ttl, 
    set_initialized, 
    is_initialized, 
    set_admin,
    get_admin,
    set_soroswap_router_address, 
    get_router_address,
    set_end_timestamp,
    get_end_timestamp, 
};
pub use error::{LiquidityTimelockError, CombinedLiquidityTimelockError};

pub fn check_nonnegative_amount(amount: i128) -> Result<(), CombinedLiquidityTimelockError> {
    if amount < 0 {
        Err(LiquidityTimelockError::NegativeNotAllowed.into())
    } else {
        Ok(())
    }
}

fn ensure_deadline(e: &Env, timestamp: u64) -> Result<(), CombinedLiquidityTimelockError> {
    let ledger_timestamp = e.ledger().timestamp();
    if ledger_timestamp >= timestamp {
        Err(LiquidityTimelockError::DeadlineExpired.into())
    } else {
        Ok(())
    }
}

fn check_initialized(e: &Env) -> Result<(), CombinedLiquidityTimelockError> {
    if is_initialized(e) {
        Ok(())
    } else {
        Err(LiquidityTimelockError::NotInitialized.into())
    }
}

fn check_timelock_bond(env: &Env) -> Result<(), CombinedLiquidityTimelockError> {
    let end_timestamp = get_end_timestamp(&env)?;
    let ledger_timestamp = env.ledger().timestamp();

    if ledger_timestamp <= end_timestamp { // we still need to wait
        return Err(LiquidityTimelockError::NeedToWait.into());
    }
    Ok(())
}


/*
    NOTE: We need to know exactely the amount of tokens that will be transferred to the Liquidity Pool
    This in order to generate the authorization for the transfer of the tokens.
    So we are copying here the function that we use in the Soroswap Router in order to know the actual amounts of tokens that will be used

*/

/// Given a pair of tokens, a desired and minimum amount of tokens to provide as liquidity, this function calculates
/// the correct amounts of tokens to add to the pool. If the pool doesn't exist, it creates one.
///
/// It considers the desired and minimum amounts for both tokens and calculates the optimal distribution to
/// satisfy these requirements while taking into account the current reserves in the pool.
///
/// # Arguments
/// * `e` - The contract environment (`Env`) in which the contract is executing.
/// * `token_a` - The address of the first token in the pair.
/// * `token_b` - The address of the second token in the pair.
/// * `amount_a_desired` - The desired amount of the first token to add.
/// * `amount_b_desired` - The desired amount of the second token to add.
/// * `amount_a_min` - The minimum required amount of the first token to add.
/// * `amount_b_min` - The minimum required amount of the second token to add.
///
/// # Returns
/// A tuple containing the calculated amounts of token A and B to be added to the pool.
fn add_liquidity_amounts(
    e: Env,
    factory: Address,
    token_a: Address,
    token_b: Address,
    amount_a_desired: i128,
    amount_b_desired: i128,
    amount_a_min: i128,
    amount_b_min: i128,
) -> Result<(i128, i128), CombinedLiquidityTimelockError> {
    // checks if the pair exists; otherwise, creates the pair
    let factory_client = SoroswapFactoryClient::new(&e, &factory);
    if !factory_client.pair_exists(&token_a, &token_b) {
        factory_client.create_pair(&token_a, &token_b);
    }

    let (reserve_a, reserve_b) = soroswap_library::get_reserves(
        e.clone(),
        factory.clone(),
        token_a.clone(),
        token_b.clone(),
    )?;

    // When there is no liquidity (first deposit)
    if reserve_a == 0 && reserve_b == 0 {
        Ok((amount_a_desired, amount_b_desired))
    } else {
        // We try first with the amount a desired:
        let amount_b_optimal = soroswap_library::quote(
            amount_a_desired.clone(),
            reserve_a.clone(),
            reserve_b.clone(),
        )?;

        if amount_b_optimal <= amount_b_desired {
            if amount_b_optimal < amount_b_min {
                return Err(LiquidityTimelockError::InsufficientBAmount.into());
            }
            Ok((amount_a_desired, amount_b_optimal))
        }
        // If not, we can try with the amount b desired
        else {
            let amount_a_optimal = soroswap_library::quote(amount_b_desired, reserve_b, reserve_a).map_err(OtherSoroswapLibraryError::from)?;

            // This should happen anyway. Because if we were not able to fulfill with our amount_b_desired for our amount_a_desired
            // It is to expect that the amount_a_optimal for that lower amount_b_desired to be lower than the amount_a_desired
            assert!(amount_a_optimal <= amount_a_desired);

            if amount_a_optimal < amount_a_min {
                return Err(LiquidityTimelockError::InsufficientAAmount.into());
            }
            Ok((amount_a_optimal, amount_b_desired))
        }
    }
}

pub trait AddLiquidityTimelockTrait {
    fn initialize(
        e: Env, 
        admin: Address, 
        router_address: Address, 
        end_timestamp: u64) -> Result<(), CombinedLiquidityTimelockError>;

    fn add_liquidity(
        e: Env,
        token_a: Address,
        token_b: Address,
        amount_a: i128,
        amount_b: i128,
        amount_a_min: i128,
        amount_b_min: i128,
        from: Address,
        deadline: u64,
    ) -> Result<(i128, i128, i128), CombinedLiquidityTimelockError>;
    
    fn claim(e: Env, pair_address:Address) -> Result<(), CombinedLiquidityTimelockError>;

    fn get_admin(e: &Env) -> Result<Address, CombinedLiquidityTimelockError>;
    
    fn get_release_time(e: &Env) -> Result<u64, CombinedLiquidityTimelockError>;
}


#[contract]
struct AddLiquidityTimelock;

#[contractimpl]
impl AddLiquidityTimelockTrait for AddLiquidityTimelock {
    
    fn initialize(
        e: Env,
        admin: Address,
        router_address: Address,
        end_timestamp: u64,
    ) -> Result<(), CombinedLiquidityTimelockError> {
        if is_initialized(&e) {
            return Err(LiquidityTimelockError::AlreadyInitialized.into());
        }

        if end_timestamp >= 10_u64.pow(10) {
            return Err(LiquidityTimelockError::WrongTimestamp.into());
        }

        set_admin(&e, admin.clone());
        set_end_timestamp(&e, end_timestamp.clone());
        set_soroswap_router_address(&e, router_address.clone());
        set_initialized(&e);

        event::initialized(&e, admin, router_address, end_timestamp);
        extend_instance_ttl(&e);
        Ok(())
    }

    /// # Arguments
    /// * `e` - The contract environment (`Env`) in which the contract is executing.
    /// * `token_a` - The address of the first token to add liquidity for.
    /// * `token_b` - The address of the second token to add liquidity for.
    /// * `amount_a_desired` - The desired amount of the first token to add.
    /// * `amount_b_desired` - The desired amount of the second token to add.
    /// * `amount_a_min` - The minimum required amount of the first token to add.
    /// * `amount_b_min` - The minimum required amount of the second token to add.
    /// * `from` - The address where the liquidity tokens will be taken from.
    /// * `deadline` - The deadline for executing the operation.
    fn add_liquidity(
        e: Env,
        token_a: Address,
        token_b: Address,
        amount_a_desired: i128,
        amount_b_desired: i128,
        amount_a_min: i128,
        amount_b_min: i128,
        from: Address,
        deadline: u64,
    ) -> Result<(i128, i128, i128), CombinedLiquidityTimelockError> {
        check_initialized(&e)?;
        check_nonnegative_amount(amount_a_desired)?;
        check_nonnegative_amount(amount_b_desired)?;
        check_nonnegative_amount(amount_a_min)?;
        check_nonnegative_amount(amount_b_min)?;
        extend_instance_ttl(&e);
        from.require_auth();
        ensure_deadline(&e, deadline)?;

        let soroswap_router_address = get_router_address(&e)?;
        let soroswap_router_client = SoroswapRouterClient::new(&e, &soroswap_router_address);

        let factory = soroswap_router_client.get_factory();

        let (amount_a, amount_b) = add_liquidity_amounts(
            e.clone(),
            factory.clone(),
            token_a.clone(),
            token_b.clone(),
            amount_a_desired,
            amount_b_desired,
            amount_a_min,
            amount_b_min,
        )?;

        // Should transfer tokens from the user to the contract
        let token_a_client = TokenClient::new(&e, &token_a);
        let token_b_client = TokenClient::new(&e, &token_b);
        token_a_client.transfer(&from, &e.current_contract_address(), &amount_a);
        token_b_client.transfer(&from, &e.current_contract_address(), &amount_b);

        let soroswap_pair_address = soroswap_router_client.router_pair_for(&token_a, &token_b);

        let mut transfer_args_a: Vec<Val> = vec![&e];
        transfer_args_a.push_back(e.current_contract_address().into_val(&e));
        transfer_args_a.push_back(soroswap_pair_address.into_val(&e));
        transfer_args_a.push_back(amount_a.into_val(&e));

        let mut transfer_args_b: Vec<Val> = vec![&e];
        transfer_args_b.push_back(e.current_contract_address().into_val(&e));
        transfer_args_b.push_back(soroswap_pair_address.into_val(&e));
        transfer_args_b.push_back(amount_b.into_val(&e));

        e.authorize_as_current_contract(vec![
            &e,
            InvokerContractAuthEntry::Contract( SubContractInvocation {
                context: ContractContext {
                    contract: token_a.clone(),
                    fn_name: Symbol::new(&e, "transfer"),
                    args: transfer_args_a.clone(),
                },
                sub_invocations: vec![&e]
            }),
            InvokerContractAuthEntry::Contract( SubContractInvocation {
                context: ContractContext {
                    contract: token_b.clone(),
                    fn_name: Symbol::new(&e, "transfer"),
                    args: transfer_args_b.clone(),
                },
                sub_invocations: vec![&e]
            })
        ]);

        // here amunt_a and amunt_b should get exactely the same value as the amount_a_desired and amount_b_desired
        let (amount_a, amount_b, liquidity) = soroswap_router_client.add_liquidity(
            &token_a,
            &token_b,
            &amount_a, // `amount_a_desired` - The desired amount of the first token to add.
            &amount_b, // `amount_b_desired` - The desired amount of the second token to add.
            &amount_a, // `amount_a_min` - The minimum required amount of the first token to add.
            &amount_b,  // `amount_b_min` - The minimum required amount of the second token to add.
            &e.current_contract_address(),
            &deadline,
        );

        // If for any reason the contract still has some token_a and token_b, it does return it to the user
        // Due to the calculation this should not be the case
        let token_a_balance = token_a_client.balance(&e.current_contract_address());
        let token_b_balance = token_a_client.balance(&e.current_contract_address());
        if token_a_balance > 0 {
            token_a_client.transfer(&e.current_contract_address(), &from, &token_a_balance);
        }
        if token_b_balance > 0 {
            token_b_client.transfer(&e.current_contract_address(), &from, &token_b_balance);
        }

        let pair: Address = soroswap_library::pair_for(
            e.clone(),
            factory,
            token_a.clone(),
            token_b.clone(),
        ).map_err(OtherSoroswapLibraryError::from)?;
        
        event::add_liquidity(
            &e,
            token_a,
            token_b,
            pair,
            amount_a,
            amount_b,
            liquidity,
            from);
            
        Ok((amount_a, amount_b, liquidity))
    }

    fn claim(e: Env, pair_address: Address ) -> Result<(), CombinedLiquidityTimelockError> {
        check_timelock_bond(&e)?; 

        let admin = get_admin(&e)?;
        admin.require_auth();

        // Should get LP tokens balance and transfer them to the admin wallet
        // So then the admin can decide how much liquidity to withdraw.
        let current_contract = &e.current_contract_address();
        let token_client = TokenClient::new(&e, &pair_address);

        let lp_balance = token_client.balance(&current_contract);
        token_client.transfer(&current_contract, &admin, &lp_balance);

        // emit claim event
        event::claim(&e, pair_address, lp_balance, admin);
        Ok(())
    }

    fn get_admin(e: &Env) -> Result<Address, CombinedLiquidityTimelockError> {
        let admin = get_admin(&e)?;
        Ok(admin)
    }

    fn get_release_time(e: &Env) -> Result<u64, CombinedLiquidityTimelockError> {
        Ok(get_end_timestamp(&e)?)
    }

}
