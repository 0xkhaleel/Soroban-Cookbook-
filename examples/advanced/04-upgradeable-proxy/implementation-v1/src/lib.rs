//! Initial implementation contract for the upgradeable proxy example.

#![no_std]

use soroban_sdk::{contract, contractimpl};

#[contract]
pub struct ImplementationV1;

#[contractimpl]
impl ImplementationV1 {
    pub fn add(a: i128, b: i128) -> i128 {
        a.checked_add(b).unwrap_or_else(|| panic!("Overflow"))
    }

    pub fn sub(a: i128, b: i128) -> i128 {
        a.checked_sub(b).unwrap_or_else(|| panic!("Underflow"))
    }
}
