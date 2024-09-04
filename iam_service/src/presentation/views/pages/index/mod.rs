use askama::Template;

#[derive(Template)]
#[template(path = "pages/index/index.html")]
pub struct Index<'a> {
    pub title: &'a str,
}