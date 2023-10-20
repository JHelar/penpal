pub mod authorize_current_user;
use axum::{
    http::{self, Request, StatusCode},
    middleware::Next,
    response::Response,
};

pub async fn auth<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let cookie_header = req
        .headers()
        .get(http::header::COOKIE)
        .and_then(|header| header.to_str().ok());

    let cookie_str = if let Some(cookie_str) = cookie_header {
        cookie_str
    } else {
        return Err(StatusCode::IM_A_TEAPOT);
    };

    if let Some(current_user) = authorize_current_user::authorize_current_user(cookie_str) {
        println!("auth: current_user {:?}", current_user);
        req.extensions_mut().insert(current_user);
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
