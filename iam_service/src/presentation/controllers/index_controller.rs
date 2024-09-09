use askama_axum::IntoResponse;
use axum::response::Redirect;

use crate::presentation::views::pages::index::Index;

pub async fn index_handler() -> impl IntoResponse {
// redirect to the /registration page
    Redirect::to("/register")
    // Index{title: "Home"}
}