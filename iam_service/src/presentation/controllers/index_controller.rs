use std::fmt::Display;
use askama_axum::IntoResponse;
use serde::{Deserialize, Serialize};
use tower_sessions::Session;

use crate::presentation::views::pages::index::Index;
const COUNTER_KEY: &str = "counter";
#[derive(Default, Deserialize, Serialize)]
pub struct Counter(usize);

impl Display for Counter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
    
}

pub async fn index_handler(session: Session) -> impl IntoResponse {
    let counter: Counter = session.get(COUNTER_KEY).await.unwrap().unwrap_or_default();
    session.insert(COUNTER_KEY, &counter.0 + 1).await.unwrap();
    Index {
        title: "Home",
        counter,
    }
}
