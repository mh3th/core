use askama::Template;

#[derive(Template)]
#[template(path = "pages/index/page.html")]
pub struct Index<'a> {
    pub title: &'a str,
}