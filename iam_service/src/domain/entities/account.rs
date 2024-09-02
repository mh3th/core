use serde::Serialize;
use uuid::Uuid;

/// Сущность Account, представляющая аккаунт пользователя.
#[derive(Debug, Serialize, Clone)]
pub struct Account {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub hashed_password: String,
}

impl Account {
    pub fn new(username: String, email: String, hashed_password: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            username,
            email,
            hashed_password,
        }
    }
}
