pub mod letter;
pub mod sending_info;
pub mod user;
use axum::http;

pub async fn hello_world() -> Result<(http::StatusCode, String), http::StatusCode> {
    Ok((http::StatusCode::OK, "Hello world!".to_string()))
}
