use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Wallet API model
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct Wallet {
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "publicKey", skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[serde(rename = "privateKey", skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
}

impl Wallet {
    pub fn new_from(address: &str, public_key: &str, private_key: &str) -> Self {
        Wallet {
            address: Some(address.to_string()),
            public_key: Some(public_key.to_string()),
            private_key: Some(private_key.to_string()),
        }
    }
}
