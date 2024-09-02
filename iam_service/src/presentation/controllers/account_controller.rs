use crate::application::services::account_service::AccountService;
use axum::{extract::Json, http::StatusCode, response::IntoResponse, Extension};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize, Debug)]
pub struct CreateAccountRequest {
    pub username: String,
    pub password: String,
}

pub async fn create_account_handler(
    Extension(account_service): Extension<Arc<AccountService>>,
    Json(payload): Json<CreateAccountRequest>,
) -> impl IntoResponse {
    match account_service
        .create_account(payload.username, payload.password)
        .await
    {
        Ok(account) => (StatusCode::CREATED, Json(account)).into_response(),
        Err(err) => {
            // Логирование ошибки и возвращение ответа
            tracing::error!("Failed to create account: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
        }
    }
}
