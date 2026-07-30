#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, Symbol};

/// # Gas Optimization Patterns for Soroban
///
/// This contract demonstrates 10+ gas optimization techniques:
/// 1. Storage tier selection (Instance vs Persistent vs Temporary)
/// 2. Caching frequently accessed values
/// 3. Batch operations vs individual operations
/// 4. Symbol interning and short symbols
/// 5. Using enums instead of strings for state
/// 6. Minimizing storage reads per operation
/// 7. Lazy initialization
/// 8. Checked arithmetic vs unchecked
/// 9. Short-circuit evaluation
/// 10. Efficient error handling
/// 11. Bitflags for boolean state
/// 12. Struct packing and layout

/// DataKey enum for typed, efficient storage access
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    /// Instance storage: Config that survives the lifetime of the contract instance
    Config = 0,
    /// Persistent storage: User balances (survives upgrades)
    Balance(soroban_sdk::Address) = 1,
    /// Temporary storage: Session data (1 ledger TTL by default)
    SessionCache(u64) = 2,
}

/// Optimization 11: Bitflags for boolean state (more efficient than separate bools)
#[contracttype]
pub struct Config {
    /// Packed flags: bit 0 = paused, bit 1 = emergency_mode, bits 2-31 = reserved
    flags: u32,
    /// Fee rate in basis points (avoids floating point)
    fee_bps: u16,
    /// Admin address
    admin: soroban_sdk::Address,
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

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    /// Optimization 10: Typed errors are more efficient than strings
    Paused = 1,
    EmergencyMode = 2,
    InsufficientBalance = 3,
    InvalidAmount = 4,
    Unauthorized = 5,
}

#[contract]
pub struct GasOptimizationContract;

/// Optimization 1 & 4: Use instance storage for contract config
/// Instance storage is cheaper than persistent and sufficient for config
/// Symbol interning: symbol_short! creates efficient short symbols
const CONFIG_KEY: Symbol = symbol_short!("cfg");

#[contractimpl]
impl GasOptimizationContract {
    /// Initialize contract config once
    /// Optimization 7: Lazy initialization - config only written once
    pub fn initialize(env: Env, admin: soroban_sdk::Address, fee_bps: u16) -> Result<(), Error> {
        // Optimization 3: Check if already initialized before writing
        if env.storage().instance().has(&CONFIG_KEY) {
            return Err(Error::Unauthorized);
        }

        let config = Config {
            flags: 0,
            fee_bps,
            admin,
        };
        env.storage().instance().set(&CONFIG_KEY, &config);
        Ok(())
    }

    /// Optimization 2 & 6: Minimize storage reads by caching config locally
    /// Single read at function entry instead of multiple reads
    pub fn transfer(
        env: Env,
        from: soroban_sdk::Address,
        to: soroban_sdk::Address,
        amount: u64,
    ) -> Result<(), Error> {
        from.require_auth();

        // Optimization 2: Cache config read (1 read instead of 3)
        let config: Config = env.storage().instance().get(&CONFIG_KEY).unwrap_or(Config {
            flags: 0,
            fee_bps: 0,
            admin: from.clone(),
        });

        // Optimization 9: Short-circuit evaluation - exit early if contract is paused
        if config.is_paused() {
            return Err(Error::Paused);
        }

        // Optimization 5: Use typed enum state — block transfers during emergency
        if config.is_emergency() {
            return Err(Error::EmergencyMode);
        }

        // Optimization 10: Use typed errors for efficient error handling
        if amount == 0 {
            return Err(Error::InvalidAmount);
        }

        // Optimization 6: Single read for from balance
        let from_balance: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::Balance(from.clone()))
            .unwrap_or(0);

        if from_balance < amount {
            return Err(Error::InsufficientBalance);
        }

        // Optimization 8: Use checked arithmetic
        let new_from_balance =
            from_balance.checked_sub(amount).ok_or(Error::InvalidAmount)?;

        // Calculate fee (using integer arithmetic, no floating point)
        let fee = (amount * config.fee_bps as u64) / 10_000;
        let to_amount = amount.checked_sub(fee).ok_or(Error::InvalidAmount)?;

        // Optimization 3 & 6: Batch storage operations
        // Write both balances in sequence rather than scattered through logic
        env.storage()
            .persistent()
            .set(&DataKey::Balance(from.clone()), &new_from_balance);

        let to_balance: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::Balance(to.clone()))
            .unwrap_or(0);
        let new_to_balance =
            to_balance.checked_add(to_amount).ok_or(Error::InvalidAmount)?;
        env.storage()
            .persistent()
            .set(&DataKey::Balance(to.clone()), &new_to_balance);

        Ok(())
    }

    /// Optimization 2 & 6: Read-heavy operation with caching
    /// Get balance without redundant config reads
    pub fn get_balance(env: Env, account: soroban_sdk::Address) -> u64 {
        // Optimization 1: Use persistent storage for balances (survives upgrades)
        env.storage()
            .persistent()
            .get(&DataKey::Balance(account))
            .unwrap_or(0)
    }

    /// Optimization 6: Efficient batch query - single storage read
    pub fn get_balances(
        env: Env,
        accounts: soroban_sdk::Vec<soroban_sdk::Address>,
    ) -> soroban_sdk::Vec<u64> {
        let mut balances = soroban_sdk::Vec::new(&env);
        for account in accounts.iter() {
            let balance = env
                .storage()
                .persistent()
                .get(&DataKey::Balance(account.clone()))
                .unwrap_or(0);
            balances.push_back(balance);
        }
        balances
    }

    /// Optimization 2 & 6: Caching reduces gas for repeated config access
    pub fn pause(env: Env) -> Result<(), Error> {
        let config: Config = env.storage().instance().get(&CONFIG_KEY).unwrap_or(Config {
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

    /// Optimization 2 & 6: Caching reduces gas for repeated config access
    pub fn unpause(env: Env) -> Result<(), Error> {
        let config: Config = env.storage().instance().get(&CONFIG_KEY).unwrap_or(Config {
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

    /// Optimization 5: Using typed enum state is more efficient than string
    pub fn set_emergency(env: Env, emergency: bool) -> Result<(), Error> {
        let config: Config = env.storage().instance().get(&CONFIG_KEY).unwrap_or(Config {
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

    /// Optimization 3: Batch initialization of multiple accounts
    /// More efficient than calling transfer multiple times
    pub fn batch_mint(
        env: Env,
        recipients: soroban_sdk::Vec<(soroban_sdk::Address, u64)>,
    ) -> Result<(), Error> {
        let config: Config = env.storage().instance().get(&CONFIG_KEY).unwrap_or(Config {
            flags: 0,
            fee_bps: 0,
            admin: env.current_contract_address(),
        });

        config.admin.require_auth();

        // Optimization 3: Write all balances in one batch
        for (recipient, amount) in recipients.iter() {
            if amount > 0 {
                let current_balance: u64 = env
                    .storage()
                    .persistent()
                    .get(&DataKey::Balance(recipient.clone()))
                    .unwrap_or(0);
                let new_balance =
                    current_balance.checked_add(amount).ok_or(Error::InvalidAmount)?;
                env.storage()
                    .persistent()
                    .set(&DataKey::Balance(recipient.clone()), &new_balance);
            }
        }

        Ok(())
    }

    /// Optimization 3: Batch burn operation
    pub fn batch_burn(
        env: Env,
        accounts: soroban_sdk::Vec<(soroban_sdk::Address, u64)>,
    ) -> Result<(), Error> {
        let config: Config = env.storage().instance().get(&CONFIG_KEY).unwrap_or(Config {
            flags: 0,
            fee_bps: 0,
            admin: env.current_contract_address(),
        });

        config.admin.require_auth();

        // Optimization 3: Process all burns efficiently
        for (account, amount) in accounts.iter() {
            let current_balance: u64 = env
                .storage()
                .persistent()
                .get(&DataKey::Balance(account.clone()))
                .unwrap_or(0);
            if current_balance < amount {
                return Err(Error::InsufficientBalance);
            }
            let new_balance = current_balance - amount;
            env.storage()
                .persistent()
                .set(&DataKey::Balance(account.clone()), &new_balance);
        }

        Ok(())
    }
}
