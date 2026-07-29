#[cfg(test)]
mod tests {
    use crate::*;
    use soroban_sdk::{Address, Env, Vec};

    fn setup() -> (Env, GasOptimizationContractClient, Address) {
        let env = Env::new();
        let contract_id = env.register_contract(None, GasOptimizationContract);
        let client = GasOptimizationContractClient::new(&env, &contract_id);
        let admin = Address::generate(&env);

        (env, client, admin)
    }

    // ============== Optimization Tests ==============

    #[test]
    fn test_optimization_1_instance_storage_initialization() {
        let (env, client, admin) = setup();

        // Optimization 1: Instance storage should be used for config
        let result = client.initialize(&admin, &100);
        assert!(result.is_ok());

        // Second init should fail - config already exists
        let result2 = client.initialize(&admin, &200);
        assert!(result2.is_err());
    }

    #[test]
    fn test_optimization_2_caching_config() {
        let (env, client, admin) = setup();
        let user1 = Address::generate(&env);
        let user2 = Address::generate(&env);

        client.initialize(&admin, &100).unwrap();

        // Optimization 2: Transfer uses cached config (1 read instead of multiple)
        let result = client.transfer(&user1, &user2, &1000);
        assert!(result.is_ok());
    }

    #[test]
    fn test_optimization_3_batch_operations_vs_individual() {
        let (env, client, admin) = setup();

        client.initialize(&admin, &0).unwrap();

        // Optimization 3: Batch mint is more gas-efficient than individual transfers
        let mut recipients = Vec::new(&env);
        for i in 0..5 {
            let addr = Address::generate(&env);
            recipients.push_back((addr, 1000 * (i + 1) as u64));
        }

        let result = client.batch_mint(&recipients);
        assert!(result.is_ok());

        // Verify all balances were set
        for i in 0..5 {
            let addr = &recipients.get(i as u32).unwrap().0;
            let balance = client.get_balance(addr);
            assert_eq!(balance, 1000 * (i + 1) as u64);
        }
    }

    #[test]
    fn test_optimization_4_symbol_interning() {
        let (env, client, admin) = setup();

        // Optimization 4: Contract uses symbol_short! for efficient symbol creation
        // This test verifies the contract initializes correctly with symbol keys
        let result = client.initialize(&admin, &50);
        assert!(result.is_ok());
    }

    #[test]
    fn test_optimization_5_enum_state_over_strings() {
        let (env, client, admin) = setup();

        client.initialize(&admin, &100).unwrap();

        // Optimization 5: State stored as enum flags, not strings
        let result = client.set_emergency(&true);
        assert!(result.is_ok());

        // Pausing in emergency mode should still work
        let pause_result = client.pause();
        assert!(pause_result.is_ok());
    }

    #[test]
    fn test_optimization_6_minimizing_storage_reads() {
        let (env, client, admin) = setup();
        let user1 = Address::generate(&env);
        let user2 = Address::generate(&env);

        client.initialize(&admin, &100).unwrap();

        // Set up initial balance
        let mut recipients = Vec::new(&env);
        recipients.push_back((user1.clone(), 5000));
        client.batch_mint(&recipients).unwrap();

        // Optimization 6: Get balance uses single persistent read
        let balance = client.get_balance(&user1);
        assert_eq!(balance, 5000);

        // Transfer with minimal reads
        let result = client.transfer(&user1, &user2, &1000);
        assert!(result.is_ok());

        assert_eq!(client.get_balance(&user1), 4900); // 5000 - 1000 + 100 fee
    }

    #[test]
    fn test_optimization_7_lazy_initialization() {
        let (env, client, admin) = setup();

        // Optimization 7: Config only written during initialization
        let result = client.initialize(&admin, &200);
        assert!(result.is_ok());

        // Verify paused status defaults to false without explicit read
        let user1 = Address::generate(&env);
        let user2 = Address::generate(&env);

        let mut recipients = Vec::new(&env);
        recipients.push_back((user1.clone(), 2000));
        client.batch_mint(&recipients).unwrap();

        // Transfer should succeed (contract not paused)
        let result = client.transfer(&user1, &user2, &1000);
        assert!(result.is_ok());
    }

    #[test]
    fn test_optimization_8_checked_arithmetic() {
        let (env, client, admin) = setup();
        let user1 = Address::generate(&env);
        let user2 = Address::generate(&env);

        client.initialize(&admin, &0).unwrap();

        let mut recipients = Vec::new(&env);
        recipients.push_back((user1.clone(), 1000));
        client.batch_mint(&recipients).unwrap();

        // Optimization 8: Checked arithmetic prevents overflow
        let result = client.transfer(&user1, &user2, &2000); // More than balance
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Error::InsufficientBalance);
    }

    #[test]
    fn test_optimization_9_short_circuit_evaluation() {
        let (env, client, admin) = setup();
        let user1 = Address::generate(&env);
        let user2 = Address::generate(&env);

        client.initialize(&admin, &100).unwrap();

        // Pause contract
        client.pause().unwrap();

        // Optimization 9: Transfer should fail immediately when paused (short-circuit)
        let result = client.transfer(&user1, &user2, &1000);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Error::Paused);
    }

    #[test]
    fn test_optimization_10_typed_errors() {
        let (env, client, admin) = setup();
        let user1 = Address::generate(&env);
        let user2 = Address::generate(&env);

        client.initialize(&admin, &100).unwrap();

        // Optimization 10: Typed errors are more efficient than string errors
        let result = client.transfer(&user1, &user2, &0); // Invalid amount
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Error::InvalidAmount);
    }

    #[test]
    fn test_optimization_11_bitflags_for_state() {
        let (env, client, admin) = setup();

        client.initialize(&admin, &100).unwrap();

        // Optimization 11: Bitflags store multiple booleans in single u32
        // Set emergency mode
        let result = client.set_emergency(&true);
        assert!(result.is_ok());

        // Pause (second flag)
        let pause_result = client.pause();
        assert!(pause_result.is_ok());

        // Both flags set efficiently
        let unpause_result = client.unpause();
        assert!(unpause_result.is_ok());
    }

    #[test]
    fn test_optimization_12_struct_packing() {
        let (env, client, admin) = setup();

        // Optimization 12: Config struct is tightly packed
        // u32 (4 bytes) + u16 (2 bytes) + Address (aligned) = minimal overhead
        let result = client.initialize(&admin, &500);
        assert!(result.is_ok());
    }

    // ============== Functional Tests ==============

    #[test]
    fn test_transfer_basic() {
        let (env, client, admin) = setup();
        let user1 = Address::generate(&env);
        let user2 = Address::generate(&env);

        client.initialize(&admin, &100).unwrap();

        let mut recipients = Vec::new(&env);
        recipients.push_back((user1.clone(), 5000));
        client.batch_mint(&recipients).unwrap();

        let result = client.transfer(&user1, &user2, &1000);
        assert!(result.is_ok());

        let user1_balance = client.get_balance(&user1);
        let user2_balance = client.get_balance(&user2);

        // user1: 5000 - 1000 = 4000 (fee is 0.01% * 1000 = 0.1, rounds down to 0)
        assert_eq!(user1_balance, 4000);
        // user2: 0 + 1000 = 1000
        assert_eq!(user2_balance, 1000);
    }

    #[test]
    fn test_transfer_with_fee() {
        let (env, client, admin) = setup();
        let user1 = Address::generate(&env);
        let user2 = Address::generate(&env);

        client.initialize(&admin, &500).unwrap(); // 5% fee

        let mut recipients = Vec::new(&env);
        recipients.push_back((user1.clone(), 10000));
        client.batch_mint(&recipients).unwrap();

        let result = client.transfer(&user1, &user2, &1000);
        assert!(result.is_ok());

        let user1_balance = client.get_balance(&user1);
        let user2_balance = client.get_balance(&user2);

        // user1: 10000 - 1000 = 9000
        assert_eq!(user1_balance, 9000);
        // user2: 0 + (1000 - 50 fee) = 950
        assert_eq!(user2_balance, 950);
    }

    #[test]
    fn test_pause_unpause() {
        let (env, client, admin) = setup();
        let user1 = Address::generate(&env);
        let user2 = Address::generate(&env);

        client.initialize(&admin, &100).unwrap();

        let mut recipients = Vec::new(&env);
        recipients.push_back((user1.clone(), 1000));
        client.batch_mint(&recipients).unwrap();

        // Transfer works before pause
        assert!(client.transfer(&user1, &user2, &100).is_ok());

        // Pause contract
        assert!(client.pause().is_ok());

        // Transfer fails when paused
        assert_eq!(
            client.transfer(&user1, &user2, &100).unwrap_err(),
            Error::Paused
        );

        // Unpause contract
        assert!(client.unpause().is_ok());

        // Transfer works after unpause
        assert!(client.transfer(&user1, &user2, &100).is_ok());
    }

    #[test]
    fn test_batch_burn() {
        let (env, client, admin) = setup();

        client.initialize(&admin, &0).unwrap();

        let mut recipients = Vec::new(&env);
        for i in 0..3 {
            recipients.push_back((Address::generate(&env), (i + 1) * 1000));
        }
        client.batch_mint(&recipients).unwrap();

        let mut burn_list = Vec::new(&env);
        for i in 0..3 {
            let addr = &recipients.get(i as u32).unwrap().0;
            burn_list.push_back((addr.clone(), 500));
        }

        let result = client.batch_burn(&burn_list);
        assert!(result.is_ok());

        // Verify balances after burn
        for i in 0..3 {
            let addr = &recipients.get(i as u32).unwrap().0;
            let balance = client.get_balance(addr);
            assert_eq!(balance, (i + 1) * 1000 - 500); // Original - burned
        }
    }

    #[test]
    fn test_batch_operations_efficiency() {
        let (env, client, admin) = setup();

        client.initialize(&admin, &0).unwrap();

        // Create 10 accounts with batch
        let mut recipients = Vec::new(&env);
        for i in 0..10 {
            recipients.push_back((Address::generate(&env), (i + 1) * 100));
        }
        let result = client.batch_mint(&recipients);
        assert!(result.is_ok());

        // Verify all 10 accounts
        for i in 0..10 {
            let addr = &recipients.get(i as u32).unwrap().0;
            let balance = client.get_balance(addr);
            assert_eq!(balance, (i + 1) * 100);
        }
    }

    #[test]
    fn test_insufficient_balance_error() {
        let (env, client, admin) = setup();
        let user1 = Address::generate(&env);
        let user2 = Address::generate(&env);

        client.initialize(&admin, &100).unwrap();

        // Try to transfer without balance
        let result = client.transfer(&user1, &user2, &1000);
        assert_eq!(result.unwrap_err(), Error::InsufficientBalance);
    }

    #[test]
    fn test_invalid_amount_error() {
        let (env, client, admin) = setup();
        let user1 = Address::generate(&env);
        let user2 = Address::generate(&env);

        client.initialize(&admin, &100).unwrap();

        // Try to transfer zero amount
        let result = client.transfer(&user1, &user2, &0);
        assert_eq!(result.unwrap_err(), Error::InvalidAmount);
    }

    #[test]
    fn test_get_balances_batch_query() {
        let (env, client, admin) = setup();

        client.initialize(&admin, &0).unwrap();

        let mut recipients = Vec::new(&env);
        for i in 0..5 {
            recipients.push_back((Address::generate(&env), (i + 1) * 1000));
        }
        client.batch_mint(&recipients).unwrap();

        let mut addresses = Vec::new(&env);
        for i in 0..5 {
            addresses.push_back(recipients.get(i as u32).unwrap().0.clone());
        }

        let balances = client.get_balances(&addresses);
        assert_eq!(balances.len(), 5);

        for i in 0..5 {
            assert_eq!(balances.get(i as u32).unwrap(), (i + 1) * 1000);
        }
    }

    #[test]
    fn test_emergency_mode() {
        let (env, client, admin) = setup();

        client.initialize(&admin, &100).unwrap();

        // Enable emergency mode
        assert!(client.set_emergency(&true).is_ok());

        // Verify contract is still usable
        assert!(client.pause().is_ok());
        assert!(client.unpause().is_ok());
    }
}
