use crate::application::services::session_service::SessionService;
use axum::{extract::Json, http::StatusCode, response::IntoResponse, Extension};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize, Debug)]
pub struct CreateSessionRequest {
    pub username: String,
    pub password: String,
}

pub async fn create_session_handler(
    Extension(session_service): Extension<Arc<SessionService>>,
    Json(payload): Json<CreateSessionRequest>,
) -> impl IntoResponse {
    match session_service
        .create_session(payload.username, payload.password)
        .await
    {
        Ok(session) => (StatusCode::CREATED, Json(session)).into_response(),
        Err(err) => {
            tracing::error!("Failed to create session: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
        }
    }
}

pub async fn read_sessions_handler(
    Extension(session_service): Extension<Arc<SessionService>>,
) -> impl IntoResponse {
    match session_service.read_sessions().await {
        Ok(sessions) => (StatusCode::OK, Json(sessions)).into_response(),
        Err(err) => {
            tracing::error!("Failed to read sessions: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
        }
    }
}

pub async fn read_session_handler(
    Extension(session_service): Extension<Arc<SessionService>>,
    Json(payload): Json<CreateSessionRequest>,
) -> impl IntoResponse {
    match session_service
        .read_session(payload.username, payload.password)
        .await
    {
        Ok(session) => (StatusCode::CREATED, Json(session)).into_response(),
        Err(err) => {
            tracing::error!("Failed to read session: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
        }
    }
}

pub async fn update_session_email_handler(
    Extension(session_service): Extension<Arc<SessionService>>,
    Json(payload): Json<CreateSessionRequest>,
) -> impl IntoResponse {
    match session_service
        .update_session_email(payload.username, payload.password)
        .await
    {
        Ok(session) => (StatusCode::CREATED, Json(session)).into_response(),
        Err(err) => {
            tracing::error!("Failed to update session: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
        }
    }
}

pub async fn archive_session_handler(
    Extension(session_service): Extension<Arc<SessionService>>,
    Json(payload): Json<CreateSessionRequest>,
) -> impl IntoResponse {
    match session_service
        .archive_session(payload.username, payload.password)
        .await
    {
        Ok(session) => (StatusCode::CREATED, Json(session)).into_response(),
        Err(err) => {
            tracing::error!("Failed to archive session: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
        }
    }
}