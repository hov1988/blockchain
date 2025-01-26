pub mod wallet_api;
pub mod blockchain_api;
pub mod models;

// Re-export the configure functions for ease of use in main.rs
pub use wallet_api::configure as configure_wallet_api;
pub use blockchain_api::configure as configure_blockchain_api;
