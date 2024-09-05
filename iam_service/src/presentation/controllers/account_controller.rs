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
            tracing::error!("Failed to create account: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
        }
    }
}

pub async fn read_accounts_handler(
    Extension(account_service): Extension<Arc<AccountService>>,
) -> impl IntoResponse {
    match account_service.read_accounts().await {
        Ok(accounts) => (StatusCode::OK, Json(accounts)).into_response(),
        Err(err) => {
            tracing::error!("Failed to read accounts: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
        }
    }
}

pub async fn read_account_handler(
    Extension(account_service): Extension<Arc<AccountService>>,
    Json(payload): Json<CreateAccountRequest>,
) -> impl IntoResponse {
    match account_service
        .read_account(payload.username, payload.password)
        .await
    {
        Ok(account) => (StatusCode::CREATED, Json(account)).into_response(),
        Err(err) => {
            tracing::error!("Failed to read account: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
        }
    }
}

pub async fn update_account_email_handler(
    Extension(account_service): Extension<Arc<AccountService>>,
    Json(payload): Json<CreateAccountRequest>,
) -> impl IntoResponse {
    match account_service
        .update_account_email(payload.username, payload.password)
        .await
    {
        Ok(account) => (StatusCode::CREATED, Json(account)).into_response(),
        Err(err) => {
            tracing::error!("Failed to update account email: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
        }
    }
}

pub async fn update_account_password_handler(
    Extension(account_service): Extension<Arc<AccountService>>,
    Json(payload): Json<CreateAccountRequest>,
) -> impl IntoResponse {
    match account_service
        .update_account_password(payload.username, payload.password)
        .await
    {
        Ok(account) => (StatusCode::CREATED, Json(account)).into_response(),
        Err(err) => {
            tracing::error!("Failed to update account password: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
        }
    }
}

pub async fn confirm_account_handler(
    Extension(account_service): Extension<Arc<AccountService>>,
    Json(payload): Json<CreateAccountRequest>,
) -> impl IntoResponse {
    match account_service
        .confirm_account(payload.username, payload.password)
        .await
    {
        Ok(account) => (StatusCode::CREATED, Json(account)).into_response(),
        Err(err) => {
            tracing::error!("Failed to confirm account: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
        }
    }
}

pub async fn archive_account_handler(
    Extension(account_service): Extension<Arc<AccountService>>,
    Json(payload): Json<CreateAccountRequest>,
) -> impl IntoResponse {
    match account_service
        .archive_account(payload.username, payload.password)
        .await
    {
        Ok(account) => (StatusCode::CREATED, Json(account)).into_response(),
        Err(err) => {
            tracing::error!("Failed to archive account: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
        }
    }
}

pub async fn reset_account_password_handler(
    Extension(account_service): Extension<Arc<AccountService>>,
    Json(payload): Json<CreateAccountRequest>,
) -> impl IntoResponse {
    match account_service
        .reset_account_password(payload.username, payload.password)
        .await
    {
        Ok(account) => (StatusCode::CREATED, Json(account)).into_response(),
        Err(err) => {
            tracing::error!("Failed to reset account password: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
        }
    }
}