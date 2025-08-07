#![no_std]
#![allow(dead_code)]

// Short symbols are max 9 characters
pub mod role {
    use soroban_sdk::{symbol_short, Symbol};

    pub const MINTER_ROLE: Symbol = symbol_short!("MINTER");
    pub const PAUSER_ROLE: Symbol = symbol_short!("PAUSER");
    pub const BURNER_ROLE: Symbol = symbol_short!("BURNER");
    pub const WHITELISTER_ROLE: Symbol = symbol_short!("WLISTER");
    pub const WHITELISTED_ROLE: Symbol = symbol_short!("WLISTED");
    pub const REDEMPTION_EXECUTOR_ROLE: Symbol = symbol_short!("REXECUTOR");
}
