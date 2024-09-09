use askama_axum::Template;

#[derive(Template)]
#[template(path = "pages/register/index.html")]
pub struct Index<'a> {
    pub title: &'a str,
    pub email_title: &'a str,
    pub email_pattern: &'a str,
    pub email_invalid_message: &'a str,
    pub password_title: &'a str,
    pub password_pattern: &'a str,
    pub password_invalid_message: &'a str,
    pub password_confirmation_title: &'a str,
    pub password_confirmation_error_message: &'a str,
    pub data_sitekey: &'a str,
}