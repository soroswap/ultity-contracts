#![no_std]
use soroban_sdk::{auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation}, Symbol, contract, contractimpl, Address, Env, vec, Vec, Val, IntoVal};
use soroban_sdk::token::Client as TokenClient;

mod event;
mod storage;
mod error;
mod test;

soroban_sdk::contractimport!(
    file = "./soroswap_router.optimized.wasm"
);
pub type SoroswapRouterClient<'a> = Client<'a>;

use storage::{
    extend_instance_ttl, 
    set_initialized, 
    is_initialized, 
    set_admin,
    get_admin,
    set_router_address, 
    get_router_address,
    set_end_timestamp,
    get_end_timestamp, 
};
pub use error::ContractError;

pub fn check_nonnegative_amount(amount: i128) -> Result<(), ContractError> {
    if amount < 0 {
        Err(ContractError::NegativeNotAllowed)
    } else {
        Ok(())
    }
}

fn ensure_deadline(e: &Env, timestamp: u64) -> Result<(), ContractError> {
    let ledger_timestamp = e.ledger().timestamp();
    if ledger_timestamp >= timestamp {
        Err(ContractError::DeadlineExpired)
    } else {
        Ok(())
    }
}

fn check_initialized(e: &Env) -> Result<(), ContractError> {
    if is_initialized(e) {
        Ok(())
    } else {
        Err(ContractError::NotInitialized)
    }
}

fn check_time_bound(env: &Env, end_timestamp: u64) -> bool {
    let ledger_timestamp = env.ledger().timestamp();

    ledger_timestamp >= end_timestamp
}

pub trait AddLiquidityTimelockTrait {
    fn initialize(e: Env, admin: Address, router_address: Address, end_timestamp: u64) -> Result<(), ContractError>;

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
    ) -> Result<(i128, i128, i128), ContractError>;
    
    fn claim(e: Env, pair_address:Address) -> Result<(), ContractError>;

    fn get_admin(e: &Env) -> Result<Address, ContractError>;
}

#[contract]
struct AddLiquidityTimelock;

#[contractimpl]
impl AddLiquidityTimelockTrait for AddLiquidityTimelock {
    /// Initializes the contract and sets the phoenix multihop address
    fn initialize(
        e: Env,
        admin: Address,
        router_address: Address,
        end_timestamp: u64,
    ) -> Result<(), ContractError> {
        if is_initialized(&e) {
            return Err(ContractError::AlreadyInitialized);
        }

        if end_timestamp >= 10_u64.pow(10) {
            return Err(ContractError::WrongTimestamp);
        }

        set_admin(&e, admin.clone());
        set_end_timestamp(&e, end_timestamp.clone());
        set_router_address(&e, router_address.clone());
    
        set_initialized(&e);
        event::initialized(&e, true, end_timestamp);
        extend_instance_ttl(&e);
        Ok(())
    }

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
    ) -> Result<(i128, i128, i128), ContractError> {
        check_initialized(&e)?;
        check_nonnegative_amount(amount_a)?;
        check_nonnegative_amount(amount_b)?;
        extend_instance_ttl(&e);
        from.require_auth();
        ensure_deadline(&e, deadline)?;

        // Should transfer tokens from the user to the contract
        TokenClient::new(&e, &token_a).transfer(&from, &e.current_contract_address(), &amount_a);
        TokenClient::new(&e, &token_b).transfer(&from, &e.current_contract_address(), &amount_b);

        // Should execute add_liquidity on router with to as this contract address
        let soroswap_router_address = get_router_address(&e);
        let soroswap_router_client = SoroswapRouterClient::new(&e, &soroswap_router_address);

        let soroswap_pair_address = soroswap_router_client.router_pair_for(&token_a, &token_b);

        let mut transfer_args: Vec<Val> = vec![&e];
        transfer_args.push_back(e.current_contract_address().into_val(&e));
        transfer_args.push_back(soroswap_pair_address.into_val(&e));
        transfer_args.push_back(amount_a.into_val(&e));

        let mut transfer_args_2: Vec<Val> = vec![&e];
        transfer_args_2.push_back(e.current_contract_address().into_val(&e));
        transfer_args_2.push_back(soroswap_pair_address.into_val(&e));
        transfer_args_2.push_back(amount_b.into_val(&e));

        e.authorize_as_current_contract(vec![
            &e,
            InvokerContractAuthEntry::Contract( SubContractInvocation {
                context: ContractContext {
                    contract: token_a.clone(),
                    fn_name: Symbol::new(&e, "transfer"),
                    args: transfer_args.clone(),
                },
                sub_invocations: vec![&e]
            }),
            InvokerContractAuthEntry::Contract( SubContractInvocation {
                context: ContractContext {
                    contract: token_b.clone(),
                    fn_name: Symbol::new(&e, "transfer"),
                    args: transfer_args_2.clone(),
                },
                sub_invocations: vec![&e]
            })
        ]);

        let result = soroswap_router_client.add_liquidity(
            &token_a,
            &token_b,
            &amount_a,
            &amount_b,
            &amount_a_min,
            &amount_b_min,
            &e.current_contract_address(),
            &deadline,
        );

        Ok(result)
    }

    fn claim(e: Env, pair_address: Address ) -> Result<(), ContractError> {
        check_initialized(&e)?;
        let admin = get_admin(&e);
        admin.require_auth();
        let end_timestamp = get_end_timestamp(&e);

        if !check_time_bound(&e, end_timestamp) {
            return Err(ContractError::NeedToWait);
        }

        // Should get LP tokens balance and transfer them to the admin wallet
        let current_contract = &e.current_contract_address();
        let token_client = TokenClient::new(&e, &pair_address);

        let lp_balance = token_client.balance(&current_contract);
        token_client.transfer(&current_contract, &admin, &lp_balance);

        Ok(())
    }

    fn get_admin(e: &Env) -> Result<Address, ContractError> {
        check_initialized(&e)?;
        let admin = get_admin(&e);
        Ok(admin)
    }

}
