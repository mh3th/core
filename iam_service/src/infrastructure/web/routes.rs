use axum::response::Html;

pub async fn not_found() -> Html<&'static str> {
    Html("<h1>Not Found</h1>")
}

pub async fn root() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}