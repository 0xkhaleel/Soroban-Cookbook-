#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, symbol_short, Address, Env, Symbol, Vec,
};

/// # Gas Optimization Patterns for Soroban
///
/// This contract demonstrates 12 gas optimization techniques:
/// 1. Storage tier selection (Instance vs Persistent vs Temporary)
/// 2. Caching frequently accessed values
/// 3. Batch operations vs individual operations
/// 4. Symbol interning and short symbols
/// 5. Using enums instead of strings for state
/// 6. Minimizing storage reads per operation
/// 7. Lazy initialization
/// 8. Checked arithmetic vs unchecked
/// 9. Short-circuit evaluation
/// 10. Efficient error handling with typed errors
/// 11. Bitflags for boolean state packing
/// 12. Struct packing and layout optimization

/// DataKey enum for typed, efficient storage access.
///
/// Optimization 1: No explicit discriminants on tuple variants —
/// `#[contracttype]` does not support mixing explicit integer discriminants
/// with tuple variants.
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    /// Instance storage: contract-wide config
    Config,
    /// Persistent storage: per-user balance (survives upgrades)
    Balance(Address),
    /// Temporary storage: session cache keyed by session id
    SessionCache(u64),
}

/// Optimization 11: Bitflags for boolean state (packs multiple booleans into
/// a single `u32`, which is one Soroban-compatible storage word).
#[contracttype]
#[derive(Clone)]
pub struct Config {
    /// Packed flags: bit 0 = paused, bit 1 = emergency_mode, bits 2–31 reserved
    pub flags: u32,
    /// Fee rate in basis points using `u32` (Soroban does not support `u16` in
    /// contract types — use `u32` and enforce the range in application logic).
    pub fee_bps: u32,
    /// Administrator address
    pub admin: Address,
}

impl Config {
    fn is_paused(&self) -> bool {
        (self.flags & 0x01) != 0
    }

    fn set_paused(&mut self, paused: bool) {
        if paused {
            self.flags |= 0x01;
        } else {
            self.flags &= !0x01;
        }
    }

    fn is_emergency(&self) -> bool {
        (self.flags & 0x02) != 0
    }

    fn set_emergency(&mut self, emergency: bool) {
        if emergency {
            self.flags |= 0x02;
        } else {
            self.flags &= !0x02;
        }
    }
}

/// Optimization 10: Typed errors via `#[contracterror]` are more efficient
/// than string panics and let callers pattern-match on specific failure modes.
///
/// Note: the type cannot be named `Error` because that conflicts with a
/// reserved name inside the Soroban SDK macros.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    Paused = 1,
    EmergencyMode = 2,
    InsufficientBalance = 3,
    InvalidAmount = 4,
    Unauthorized = 5,
}

#[contract]
pub struct GasOptimizationContract;

/// Optimization 4: `symbol_short!` creates a compile-time `Symbol` constant
/// that is cheaper to compare and store than a heap-allocated string.
const CONFIG_KEY: Symbol = symbol_short!("cfg");

#[contractimpl]
impl GasOptimizationContract {
    /// Initialize contract config once.
    ///
    /// Optimization 7: Lazy initialization — config is written exactly once;
    /// subsequent calls are rejected, so callers only pay the write cost once.
    pub fn initialize(
        env: Env,
        admin: Address,
        fee_bps: u32,
    ) -> Result<(), ContractError> {
        // Guard against re-initialization before writing anything.
        if env.storage().instance().has(&CONFIG_KEY) {
            return Err(ContractError::Unauthorized);
        }

        let config = Config {
            flags: 0,
            fee_bps,
            admin,
        };
        env.storage().instance().set(&CONFIG_KEY, &config);
        Ok(())
    }

    /// Transfer tokens from `from` to `to`.
    ///
    /// Optimization 2 & 6: Read config once and cache it locally rather than
    /// issuing multiple individual storage reads throughout the function.
    ///
    /// Optimization 9: Short-circuit on `paused` / `emergency` before touching
    /// balance storage, so we pay zero balance-read gas on blocked calls.
    pub fn transfer(
        env: Env,
        from: Address,
        to: Address,
        amount: i128,
    ) -> Result<(), ContractError> {
        from.require_auth();

        // Optimization 2: single config read cached in a local variable.
        let config: Config =
            env.storage().instance().get(&CONFIG_KEY).unwrap_or(Config {
                flags: 0,
                fee_bps: 0,
                admin: from.clone(),
            });

        // Optimization 9: exit immediately when paused (no balance I/O).
        if config.is_paused() {
            return Err(ContractError::Paused);
        }

        // Optimization 5: typed enum state — block transfers during emergency.
        if config.is_emergency() {
            return Err(ContractError::EmergencyMode);
        }

        if amount <= 0 {
            return Err(ContractError::InvalidAmount);
        }

        // Optimization 6: single read for the sender balance.
        let from_balance: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Balance(from.clone()))
            .unwrap_or(0);

        if from_balance < amount {
            return Err(ContractError::InsufficientBalance);
        }

        // Optimization 8: checked arithmetic to catch overflow without panic.
        let new_from_balance = from_balance
            .checked_sub(amount)
            .ok_or(ContractError::InvalidAmount)?;

        // Fee calculated with integer arithmetic — no floating point needed.
        let fee = (amount * config.fee_bps as i128) / 10_000;
        let to_amount = amount
            .checked_sub(fee)
            .ok_or(ContractError::InvalidAmount)?;

        // Optimization 3 & 6: batch both balance writes together.
        env.storage()
            .persistent()
            .set(&DataKey::Balance(from.clone()), &new_from_balance);

        let to_balance: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Balance(to.clone()))
            .unwrap_or(0);
        let new_to_balance = to_balance
            .checked_add(to_amount)
            .ok_or(ContractError::InvalidAmount)?;
        env.storage()
            .persistent()
            .set(&DataKey::Balance(to.clone()), &new_to_balance);

        Ok(())
    }

    /// Return the balance of `account`.
    ///
    /// Optimization 1: balances live in persistent storage so they survive
    /// contract upgrades without a migration step.
    pub fn get_balance(env: Env, account: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Balance(account))
            .unwrap_or(0)
    }

    /// Return balances for multiple accounts in a single call.
    ///
    /// Optimization 6: batching queries reduces per-call overhead versus
    /// issuing N individual `get_balance` cross-contract calls.
    pub fn get_balances(env: Env, accounts: Vec<Address>) -> Vec<i128> {
        let mut balances = Vec::new(&env);
        for account in accounts.iter() {
            let balance: i128 = env
                .storage()
                .persistent()
                .get(&DataKey::Balance(account.clone()))
                .unwrap_or(0);
            balances.push_back(balance);
        }
        balances
    }

    /// Pause the contract (admin only).
    pub fn pause(env: Env) -> Result<(), ContractError> {
        let config: Config =
            env.storage().instance().get(&CONFIG_KEY).unwrap_or(Config {
                flags: 0,
                fee_bps: 0,
                admin: env.current_contract_address(),
            });

        config.admin.require_auth();

        let mut new_config = config;
        new_config.set_paused(true);
        env.storage().instance().set(&CONFIG_KEY, &new_config);
        Ok(())
    }

    /// Unpause the contract (admin only).
    pub fn unpause(env: Env) -> Result<(), ContractError> {
        let config: Config =
            env.storage().instance().get(&CONFIG_KEY).unwrap_or(Config {
                flags: 0,
                fee_bps: 0,
                admin: env.current_contract_address(),
            });

        config.admin.require_auth();

        let mut new_config = config;
        new_config.set_paused(false);
        env.storage().instance().set(&CONFIG_KEY, &new_config);
        Ok(())
    }

    /// Enable or disable emergency mode (admin only).
    ///
    /// Optimization 5: state is stored as a bitflag rather than a string,
    /// making reads and comparisons significantly cheaper.
    pub fn set_emergency(env: Env, emergency: bool) -> Result<(), ContractError> {
        let config: Config =
            env.storage().instance().get(&CONFIG_KEY).unwrap_or(Config {
                flags: 0,
                fee_bps: 0,
                admin: env.current_contract_address(),
            });

        config.admin.require_auth();

        let mut new_config = config;
        new_config.set_emergency(emergency);
        env.storage().instance().set(&CONFIG_KEY, &new_config);
        Ok(())
    }

    /// Mint tokens to multiple recipients in a single call.
    ///
    /// Optimization 3: batching writes reduces per-call invocation overhead
    /// compared with N individual mint calls.
    pub fn batch_mint(
        env: Env,
        recipients: Vec<(Address, i128)>,
    ) -> Result<(), ContractError> {
        let config: Config =
            env.storage().instance().get(&CONFIG_KEY).unwrap_or(Config {
                flags: 0,
                fee_bps: 0,
                admin: env.current_contract_address(),
            });

        config.admin.require_auth();

        for (recipient, amount) in recipients.iter() {
            if amount > 0 {
                let current: i128 = env
                    .storage()
                    .persistent()
                    .get(&DataKey::Balance(recipient.clone()))
                    .unwrap_or(0);
                let new_balance = current
                    .checked_add(amount)
                    .ok_or(ContractError::InvalidAmount)?;
                env.storage()
                    .persistent()
                    .set(&DataKey::Balance(recipient.clone()), &new_balance);
            }
        }
        Ok(())
    }

    /// Burn tokens from multiple accounts in a single call.
    ///
    /// Optimization 3: same batching benefit as `batch_mint`.
    pub fn batch_burn(
        env: Env,
        accounts: Vec<(Address, i128)>,
    ) -> Result<(), ContractError> {
        let config: Config =
            env.storage().instance().get(&CONFIG_KEY).unwrap_or(Config {
                flags: 0,
                fee_bps: 0,
                admin: env.current_contract_address(),
            });

        config.admin.require_auth();

        for (account, amount) in accounts.iter() {
            let current: i128 = env
                .storage()
                .persistent()
                .get(&DataKey::Balance(account.clone()))
                .unwrap_or(0);
            if current < amount {
                return Err(ContractError::InsufficientBalance);
            }
            env.storage()
                .persistent()
                .set(&DataKey::Balance(account.clone()), &(current - amount));
        }
        Ok(())
    }
}
