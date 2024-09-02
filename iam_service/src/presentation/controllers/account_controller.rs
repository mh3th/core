use axum::{
    extract::{Extension, Json},
    response::IntoResponse,
    http::StatusCode,
};
use crate::domain::use_cases::create_account::CreateAccount;
use crate::infrastructure::db::connection::DbConnection;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateAccountRequest {
    username: String,
    email: String,
    password: String,
}

pub async fn create_account_handler(
    Json(payload): Json<CreateAccountRequest>,
    Extension(db): Extension<DbConnection>,
) -> impl IntoResponse {
    let hashed_password = hash_password(payload.password);

    let use_case = CreateAccount { db: &db };

    match use_case
        .execute(payload.username, payload.email, hashed_password)
        .await
    {
        Ok(account) => (StatusCode::CREATED, Json(account)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create account").into_response(),
    }
}

fn hash_password(password: String) -> String {
    // Реализуйте функцию хэширования пароля, например, используя bcrypt
    password
}
