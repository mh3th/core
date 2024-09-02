use std::sync::Arc;

use crate::domain::entities::account::Account;
use crate::domain::repositories::account_repository::AccountReadRepository;
use axum::async_trait;
use sqlx::Error;
use uuid::Uuid;

/// Сценарий использования для чтения аккаунта по ID.
#[async_trait]
pub trait ReadAccountUseCase {
    async fn execute(&self, id: Uuid) -> Result<Option<Account>, Error>;
}

/// Реализация ReadAccountUseCase.
pub struct ReadAccount<UCR> {
    repository: Arc<UCR>,
}

impl<UCR> ReadAccount<UCR>
where
    UCR: AccountReadRepository + Sync + Send + 'static,
{
    pub fn new(repository: Arc<UCR>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<UCR> ReadAccountUseCase for ReadAccount<UCR>
where
    UCR: AccountReadRepository + Sync + Send + 'static,
{
    async fn execute(&self, id: Uuid) -> Result<Option<Account>, Error> {
        self.repository.find_by_id(id).await
    }
}
