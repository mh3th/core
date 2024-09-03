use axum::response::{Html, IntoResponse};

pub async fn not_found() -> impl IntoResponse {
    Html("<h1>Not Found</h1>")
}