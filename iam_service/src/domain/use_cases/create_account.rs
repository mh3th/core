use std::sync::Arc;

use crate::domain::entities::account::Account;
use crate::domain::repositories::account_repository::AccountCreateRepository;
use axum::async_trait;
use sqlx::Error;

/// Сценарий использования для создания аккаунта.
#[async_trait]
pub trait CreateAccountUseCase {
    async fn execute(
        &self,
        username: String,
        email: String,
        hashed_password: String,
    ) -> Result<Account, Error>;
}

/// Реализация CreateAccountUseCase.
pub struct CreateAccount<UCR> {
    repository: Arc<UCR>,
}

impl<UCR> CreateAccount<UCR>
where
    UCR: AccountCreateRepository + Sync + Send + 'static,
{
    pub fn new(repository: Arc<UCR>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<UCR> CreateAccountUseCase for CreateAccount<UCR>
where
    UCR: AccountCreateRepository + Sync + Send + 'static,
{
    async fn execute(
        &self,
        username: String,
        email: String,
        hashed_password: String,
    ) -> Result<Account, Error> {
        let account = Account::new(username, email, hashed_password);
        self.repository.create(account).await
    }
}
