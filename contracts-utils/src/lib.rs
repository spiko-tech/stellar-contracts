pub mod role {
    use soroban_sdk::{symbol_short, Symbol};

    pub const MINTER_ROLE: Symbol = symbol_short!("0");
    pub const PAUSER_ROLE: Symbol = symbol_short!("1");
    pub const BURNER_ROLE: Symbol = symbol_short!("2");
    pub const WHITELISTER_ROLE: Symbol = symbol_short!("3");
    pub const WHITELISTED_ROLE: Symbol = symbol_short!("4");
    pub const REDEMPTION_EXECUTOR_ROLE: Symbol = symbol_short!("5");
}
