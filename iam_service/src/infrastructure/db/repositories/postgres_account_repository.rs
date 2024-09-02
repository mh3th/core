use std::sync::Arc;

use crate::domain::entities::account::Account;
use crate::domain::repositories::account_repository::{
    AccountCreateRepository, AccountFindRepository, AccountReadRepository,
};
use crate::infrastructure::db::connections::Connections;
use axum::async_trait;
use sqlx::Error;
use uuid::Uuid;

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
            INSERT INTO accounts (id, username, email, hashed_password)
            VALUES ($1, $2, $3, $4)
            "#,
            account.id,
            account.username,
            account.email,
            account.hashed_password
        )
        .execute(&self.db.pg_pool)
        .await?;

        Ok(account)
    }
}

#[async_trait]
impl AccountReadRepository for PostgresAccountRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Account>, Error> {
        let account = sqlx::query_as!(
            Account,
            r#"
            SELECT id, username, email, hashed_password
            FROM accounts
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.db.pg_pool)
        .await?;

        Ok(account)
    }
}

#[async_trait]
impl AccountFindRepository for PostgresAccountRepository {
    async fn find_by_email(&self, email: String) -> Result<Option<Account>, Error> {
        let account = sqlx::query_as!(
            Account,
            r#"
            SELECT id, username, email, hashed_password
            FROM accounts
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(&self.db.pg_pool)
        .await?;

        Ok(account)
    }
}
