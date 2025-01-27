use crate::dto;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct TransactionDto {
    #[serde(rename = "sender", skip_serializing_if = "Option::is_none")]
    pub sender: Option<String>,
    #[serde(rename = "recipient", skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(rename = "publicKey", skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[serde(rename = "privateKey", skip_serializing_if = "Option::is_none")]
    pub private_key: Option<String>,
}

/// Transactions in blockchain response model
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct TransactionsInBlockChainResponse {
    pub transaction_count: usize,
    pub transactions: Vec<Transaction>,
}

/// Transaction API model
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct Transaction {
    #[serde(rename = "sender", skip_serializing_if = "Option::is_none")]
    pub sender: Option<String>,
    #[serde(rename = "recipient", skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(rename = "publicKey", skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[serde(rename = "signature", skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

/// Transaction request model for signing transactions
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct TransactionRequest {
    pub recipient: String,
    pub amount: f64,
}
