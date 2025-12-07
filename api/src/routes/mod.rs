use axum::Router;

pub mod auth;

pub fn app_routes() -> Router {
	Router::new()
		.nest("/auth", auth::auth_routes())
}	