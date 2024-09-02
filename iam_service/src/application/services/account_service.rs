use std::sync::Arc;

use crate::domain::entities::account::Account;
use crate::domain::use_cases::{
    create_account::CreateAccountUseCase, find_account::FindAccountUseCase,
    read_account::ReadAccountUseCase,
};
use sqlx::Error;
use uuid::Uuid;

/// Сервис для работы с аккаунтами, объединяющий различные сценарии использования.
pub struct AccountService {
    create_account_use_case: Arc<dyn CreateAccountUseCase + Sync + Send>,
    read_account_use_case: Arc<dyn ReadAccountUseCase + Sync + Send>,
    find_account_use_case: Arc<dyn FindAccountUseCase + Sync + Send>,
}

impl AccountService {
    pub fn new(
        create_account_use_case: Arc<dyn CreateAccountUseCase + Sync + Send>,
        read_account_use_case: Arc<dyn ReadAccountUseCase + Sync + Send>,
        find_account_use_case: Arc<dyn FindAccountUseCase + Sync + Send>,
    ) -> Self {
        Self {
            create_account_use_case,
            read_account_use_case,
            find_account_use_case,
        }
    }

    /// Создает новый аккаунт.
    pub async fn create_account(
        &self,
        username: String,
        email: String,
        hashed_password: String,
    ) -> Result<Account, Error> {
        self.create_account_use_case
            .execute(username, email, hashed_password)
            .await
    }

    /// Читает аккаунт по ID.
    pub async fn read_account(&self, id: Uuid) -> Result<Option<Account>, Error> {
        self.read_account_use_case.execute(id).await
    }

    /// Ищет аккаунт по email.
    pub async fn find_account(&self, email: String) -> Result<Option<Account>, Error> {
        self.find_account_use_case.execute(email).await
    }
}
