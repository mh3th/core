use askama_axum::IntoResponse;

use crate::presentation::views::pages::contacts::Index;

pub async fn index_handler() -> impl IntoResponse {
    Index{title: "Home"}
}