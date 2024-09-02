use std::sync::Arc;

use crate::domain::entities::account::Account;
use crate::domain::repositories::account_repository::AccountFindRepository;
use axum::async_trait;
use sqlx::Error;

/// Сценарий использования для поиска аккаунта по username.
#[async_trait]
pub trait FindAccountUseCase {
    async fn execute(&self, username: String) -> Result<Option<Account>, Error>;
}

/// Реализация FindAccountUseCase.
pub struct FindAccount<UCR> {
    repository: Arc<UCR>,
}

impl<UCR> FindAccount<UCR>
where
    UCR: AccountFindRepository + Sync + Send + 'static,
{
    pub fn new(repository: Arc<UCR>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<UCR> FindAccountUseCase for FindAccount<UCR>
where
    UCR: AccountFindRepository + Sync + Send,
{
    async fn execute(&self, username: String) -> Result<Option<Account>, Error> {
        self.repository.find_by_username(username).await
    }
}
