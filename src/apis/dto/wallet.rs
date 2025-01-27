use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Wallet API model
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct WalletDto {
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "publicKey", skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[serde(rename = "privateKey", skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
}

/// Wallet response for API endpoints
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct WalletResponse {
    pub address: String,
    pub public_key: String,
    pub private_key: String,
}

/// Query amount response model
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct QueryAmountResponse {
    pub amount: f64,
}

impl WalletDto {
    pub fn new_from(address: &str, public_key: &str, private_key: &str) -> Self {
        WalletDto {
            address: Some(address.to_string()),
            public_key: Some(public_key.to_string()),
            private_key: Some(private_key.to_string()),
        }
    }
}
