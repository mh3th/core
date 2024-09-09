use crate::domain::entities::account::Account;
use axum::async_trait;
use sqlx::Error;

/// Трейт для создания аккаунта.
#[async_trait]
pub trait AccountCreateRepository {
    async fn create(&self, account: Account) -> Result<Account, Error>;
}
