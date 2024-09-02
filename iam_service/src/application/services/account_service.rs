use std::sync::Arc;

use crate::domain::entities::account::Account;
use crate::domain::use_cases::{
    create_account::CreateAccountUseCase,
    read_account::ReadAccountUseCase,
    find_account::FindAccountUseCase,
};
use sqlx::Error;
use uuid::Uuid;

/// Сервис для работы с аккаунтами, объединяющий различные сценарии использования.
pub struct AccountService<UCC, UCR, UCF> {
    create_use_case: Arc<UCC>,
    read_use_case: Arc<UCR>,
    find_use_case: Arc<UCF>,
}

impl<UCC, UCR, UCF> AccountService<UCC, UCR, UCF>
where
    UCC: CreateAccountUseCase + Sync + Send + 'static,
    UCR: ReadAccountUseCase + Sync + Send + 'static,
    UCF: FindAccountUseCase + Sync + Send + 'static,
{
    pub fn new(create_use_case: Arc<UCC>, read_use_case: Arc<UCR>, find_use_case: Arc<UCF>) -> Self {
        Self {
            create_use_case,
            read_use_case,
            find_use_case,
        }
    }

    /// Создает новый аккаунт.
    pub async fn create_account(
        &self,
        username: String,
        email: String,
        hashed_password: String,
    ) -> Result<Account, Error> {
        self.create_use_case
            .execute(username, email, hashed_password)
            .await
    }

    /// Читает аккаунт по ID.
    pub async fn read_account(&self, id: Uuid) -> Result<Option<Account>, Error> {
        self.read_use_case.execute(id).await
    }

    /// Ищет аккаунт по email.
    pub async fn find_account(&self, email: String) -> Result<Option<Account>, Error> {
        self.find_use_case.execute(email).await
    }
}
