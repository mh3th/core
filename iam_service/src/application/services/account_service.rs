use std::sync::Arc;

use crate::domain::entities::account::Account;
use crate::domain::use_cases::{
    create_account::CreateAccountUseCase, 
    // find_account::FindAccountUseCase,
    // read_account::ReadAccountUseCase,
};
use sqlx::Error;
// use uuid::Uuid;

/// Сервис для работы с аккаунтами, объединяющий различные сценарии использования.
pub struct AccountService <T: CreateAccountUseCase + Sync + Send> {
   create_account_use_case: Arc<T>,
}

impl<T: CreateAccountUseCase + Sync + Send> AccountService<T> {
    pub fn new(
        create_account_use_case: Arc<T>,
    ) -> Self {
        Self {
            create_account_use_case,
            // read_account_use_case,
            // find_account_use_case,
        }
    }

    /// Создает новый аккаунт.
    pub async fn create_account(
        &self,
        username: String,
        hashed_password: String,
    ) -> Result<Account, Error> {
        self.create_account_use_case
            .execute(username, hashed_password)
            .await
    }
}
