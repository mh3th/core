use askama_axum::IntoResponse;

use crate::presentation::views::pages::register::Index;
pub static EMAIL_PATTERN: &str =
    r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,10})";
pub static PASSWORD_PATTERN: &str = r"^(?=.*[^\w\s]).{5,200}$";
pub async fn indexx_handler() -> impl IntoResponse {
    Index{
        title: "Registration",
        email_title: "Your email",
        email_pattern: EMAIL_PATTERN,
        email_invalid_message: "Invalid email address",
        password_title: "Your password",
        password_pattern: PASSWORD_PATTERN,
        password_invalid_message: "Must contain at least one special character and be between 5 and 200 characters long",
        password_confirmation_title: "Confirm password",
        password_confirmation_error_message: "Passwords do not match",
        data_sitekey: "6LeIxAcTAAAAAJcZVRqyHh71UMIEGNQ_MXjiZKhI",
    }
}
