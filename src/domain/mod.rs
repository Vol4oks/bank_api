use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type AccountId = u32;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: AccountId,
    pub balance: i64,
}

impl Account {
    pub fn new(id: u32, initial_balance: i64) -> Result<Self, DomainError> {
        if initial_balance < 0 {
            return Err(DomainError::InvalodAmount(
                "Initial balance must be non-negative".into(),
            ));
        }
        Ok(Self {
            id,
            balance: initial_balance,
        })
    }

    pub fn deposit(&mut self, amount: Amount) {
        self.balance += amount.0
    }

    pub fn withdraw(&mut self, amount: Amount) -> Result<(), DomainError> {
        if self.balance < amount.0 {
            return Err(DomainError::InsufficientFunds);
        }
        self.balance -= amount.0;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Amount(pub i64);

impl Amount {
    pub fn new(value: i64) -> Result<Self, DomainError> {
        if value < 0 {
            return Err(DomainError::InvalodAmount(
                "Amount must be non-negative".into(),
            ));
        }
        Ok(Self(value))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transfer {
    pub from: AccountId,
    pub to: AccountId,
    pub amount: Amount,
}

impl Transfer {
    pub fn new(from: AccountId, to: AccountId, amount: i64) -> Result<Self, DomainError> {
        if from == to {
            return Err(DomainError::InvalodAmount(
                "Transfer must be between different accounts".into(),
            ));
        }
        Ok(Self {
            from,
            to,
            amount: Amount::new(amount)?,
        })
    }
}

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("invalid amount: {0}")]
    InvalodAmount(String),
    #[error("insufficient funds")]
    InsufficientFunds,
    #[error("account not found")]
    AccountNotFound,
}
