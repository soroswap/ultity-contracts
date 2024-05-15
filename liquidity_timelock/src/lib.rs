#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Vec, String};
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
    has_router_address,
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

pub trait AddLiquidityTimelockTrait {
    fn initialize(e: Env, admin: Address, router_address: Address, end_timestamp: u64) -> Result<(), ContractError>;

    fn add_liquidity(
        e: Env,
        token_a: Address,
        token_b: Address,
        amount_a: i128,
        amount_b: i128,
        from: Address,
        deadline: u64,
    ) -> Result<(i128, i128, i128), ContractError>;
    
    fn claim(e: Env) -> Result<(), ContractError>;

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
        from: Address,
        deadline: u64,
    ) -> Result<(i128, i128, i128), ContractError> {
        check_initialized(&e)?;
        check_nonnegative_amount(amount_a)?;
        check_nonnegative_amount(amount_b)?;
        extend_instance_ttl(&e);
        from.require_auth();
        ensure_deadline(&e, deadline)?;

        let current_contract = &e.current_contract_address();

        // Should transfer tokens from the user to the contract
        TokenClient::new(&e, &token_a).transfer(&from, &current_contract, &amount_a);
        TokenClient::new(&e, &token_b).transfer(&from, &current_contract, &amount_b);

        // Should execute add_liquidity on router with to as this contract address
        let soroswap_router_address = get_router_address(&e);
        let soroswap_router_client = SoroswapRouterClient::new(&e, &soroswap_router_address);

        let result = soroswap_router_client.add_liquidity(
            &token_a,
            &token_b,
            &amount_a,
            &amount_b,
            &0,
            &0,
            &current_contract,
            &deadline,
        );

        Ok(result)
    }

    fn claim(e: Env) -> Result<(), ContractError> {
        check_initialized(&e)?;
        let admin = get_admin(&e);
        admin.require_auth();
        let ledger_timestamp = e.ledger().timestamp();
        let end_timestamp = get_end_timestamp(&e);
        
        // Should get LP tokens balance and transfer them to the admin wallet
        let current_contract = &e.current_contract_address();
        
        if  ledger_timestamp >= end_timestamp {
            // Should get lp_token address / pair address 
            //THIS IS A MOCKUP (TESTNET SOROSWAP XLM/USDC PAIR)
            let address_string = String::from_str(&e, "CBDBQPQU3JPSW5RYOG7U3GDUOYNZ2I2G4VZGNLAUX27CNQR65M6XBCQN");
            let lp_token_address = Address::from_string(&address_string);
            let lp_balance = TokenClient::new(&e, &lp_token_address).balance(&current_contract);

            TokenClient::new(&e, &lp_token_address).transfer(&current_contract, &admin, &lp_balance);
        }    
        Ok(())
    }

    fn get_admin(e: &Env) -> Result<Address, ContractError> {
        check_initialized(&e)?;
        let admin = get_admin(&e);
        Ok(admin)
    }

}
