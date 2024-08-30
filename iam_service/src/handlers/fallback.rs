use axum::response::Html;

pub async fn handler() -> Html<&'static str> {
    Html("<h1>Not Found</h1>")
}