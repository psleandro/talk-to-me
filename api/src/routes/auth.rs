use axum::Router;
use axum::routing::post;
use crate::handlers::auth::signup;

pub fn auth_routes() -> Router {
    Router::new()
        .route("/signup", post(signup))
}