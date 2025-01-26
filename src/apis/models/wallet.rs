use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Wallet {
    pub address: String,
    pub public_key: String,
    pub private_key: String,
}
