//! Integration tests for the Soroban Cookbook examples.
//!
//! This file is intentionally minimal; cross-contract benchmarks live
//! in `cross_contract_benchmarks.rs`.

use soroban_sdk::{Env, Symbol};

#[test]
fn env_works() {
    let env = Env::default();
    let sym = Symbol::new(&env, "ok");
    assert_eq!(sym, Symbol::new(&env, "ok"));
}
