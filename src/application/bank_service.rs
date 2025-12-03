use std::sync::Arc;

use crate::{
    data::account_repository::AccountRepository,
    domain::{Account, AccountId, Amount, DomainError, Transfer},
};

#[derive(Clone)]
pub struct BankService<R: AccountRepository + 'static> {
    repo: Arc<R>,
}

impl<R> BankService<R>
where
    R: AccountRepository + 'static,
{
    pub fn new(repo: Arc<R>) -> Self {
        BankService { repo }
    }

    pub async fn create_account(
        &self,
        id: AccountId,
        initial_balance: i64,
    ) -> Result<(), DomainError> {
        let account = Account::new(id, initial_balance)?;
        self.repo.create(account).await
    }

    pub async fn get_account(&self, id: AccountId) -> Result<Account, DomainError> {
        match self.repo.get(id).await? {
            Some(account) => Ok(account),
            None => Err(DomainError::AccountNotFound),
        }
    }

    pub async fn deposit(&self, id: AccountId, amount: i64) -> Result<Account, DomainError> {
        let mut account = self.get_account(id).await?;
        let amount = Amount::new(amount)?;
        account.deposit(amount);
        self.repo.upsert(account.clone()).await?;
        Ok(account)
    }

    pub async fn withdraw(&self, id: AccountId, amount: i64) -> Result<Account, DomainError> {
        let mut account = self.get_account(id).await?;
        let amount = Amount::new(amount)?;
        account.withdraw(amount)?;
        self.repo.upsert(account.clone()).await?;
        Ok(account)
    }

    pub async fn transfer(&self, from: AccountId, to: AccountId, amount: i64) -> Result<(), DomainError> {
        let transfer = Transfer::new(from, to, amount)?;

        let mut from_account = self.get_account(transfer.from).await?;
        let mut to_account = self.get_account(transfer.to).await?;

        from_account.withdraw(transfer.amount)?;
        to_account.deposit(transfer.amount);

        self.repo.upsert(from_account).await?;
        self.repo.upsert(to_account).await?;        

        Ok(())
    }
}
