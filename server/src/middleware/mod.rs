use axum::{
    http::{self, Request, StatusCode},
    middleware::Next,
    response::Response,
};

#[derive(Clone)]
pub struct CurrentUser {
    pub id: uuid::Uuid,
}

fn authorize_current_user(auth_header: &str) -> Option<CurrentUser> {
    let bearer = auth_header.split_at(7).1;
    let user_id = if let Ok(user_id) = uuid::Uuid::parse_str(bearer) {
        user_id
    } else {
        return None;
    };

    let current_user = CurrentUser { id: user_id };
    Some(current_user)
}

pub async fn auth<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    if let Some(current_user) = authorize_current_user(auth_header) {
        // insert the current user into a request extension so the handler can
        // extract it
        req.extensions_mut().insert(current_user);
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}
