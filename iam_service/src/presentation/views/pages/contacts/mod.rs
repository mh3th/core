use askama::Template;

#[derive(Template)]
#[template(path = "pages/contacts/index.html")]
pub struct Index<'a> {
    pub title: &'a str,
}