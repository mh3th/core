use std::sync::Arc;

use crate::domain::entities::account::Account;
use crate::domain::repositories::account_repository::{
    AccountCreateRepository, 
    // AccountFindRepository, 
    // AccountReadRepository,
};
use crate::infrastructure::db::connections::Connections;
use axum::async_trait;
use sqlx::Error;
// use uuid::Uuid;

/// Репозиторий аккаунтов для PostgreSQL.
pub struct PostgresAccountRepository {
    pub db: Arc<Connections>,
}

impl PostgresAccountRepository {
    pub fn new(db: Arc<Connections>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl AccountCreateRepository for PostgresAccountRepository {
    async fn create(&self, account: Account) -> Result<Account, Error> {
        sqlx::query!(
            r#"
            INSERT INTO accounts (id, username, hashed_password)
            VALUES ($1, $2, $3)
            "#,
            account.id,
            account.username,
            account.hashed_password
        )
        .execute(&self.db.pg_pool)
        .await?;
        Ok(account)
    }
}