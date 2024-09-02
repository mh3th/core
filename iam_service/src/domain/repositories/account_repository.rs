use crate::domain::entities::account::Account;
use axum::async_trait;
use sqlx::Error;
use uuid::Uuid;

/// Трейт для создания аккаунта.
#[async_trait]
pub trait AccountCreateRepository {
    async fn create(&self, account: Account) -> Result<Account, Error>;
}

/// Трейт для чтения аккаунта по ID.
#[async_trait]
pub trait AccountReadRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Account>, Error>;
}

/// Трейт для поиска аккаунта по email.
#[async_trait]
pub trait AccountFindRepository {
    async fn find_by_email(&self, email: String) -> Result<Option<Account>, Error>;
}
