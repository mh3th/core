use askama_axum::Template;

use crate::presentation::controllers::index_controller::Counter;

#[derive(Template)]
#[template(path = "pages/index/index.html")]
pub struct Index<'a> {
    pub title: &'a str,
    pub counter: Counter
}