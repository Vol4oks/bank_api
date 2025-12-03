use serde::{Deserialize, Serialize};

use crate::domain::{Account, AccountId};

#[derive(Debug, Deserialize)]
pub struct CreateAccountRquest {
    pub id: AccountId,
    pub initial: i64,
}

#[derive(Debug, Deserialize)]
pub struct AmountRequest {
    pub amount: i64,
}

#[derive(Debug, Deserialize)]
pub struct TransferRequest {
    pub from: AccountId,
    pub to: AccountId,
    pub amount: i64,
}

#[derive(Debug, Serialize)]
pub struct AccountResponse {
    pub id: AccountId,
    pub balance: i64,
}

impl From<Account> for AccountResponse {
    fn from(acc: Account) -> Self {
        Self {
            id: acc.id,
            balance: acc.balance,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub error: String,
}

impl ApiError {
    pub fn new(msg: impl Into<String>) -> Self {
        Self { error: msg.into() }
    }
}
