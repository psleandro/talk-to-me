use axum::{
	extract::Json,
	http::StatusCode,
	response::{IntoResponse, Response}
};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::extractors::ValidatedJson;

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
	error: String,
}

#[derive(Deserialize, Validate)]
pub struct SignUp {
	#[validate(length(min = 2, message = "must be at least 2 characters long"))]
	name: String,

	#[validate(email(message = "invalid format"))]
	email: String,

	#[validate(length(min = 8, message = "must be at least 8 characters long"))]
	password: String,
}


pub async fn signup(ValidatedJson(payload): ValidatedJson<SignUp>) -> Response {
	let created_user = User {
		id: 1,
		name: payload.name,
		email: payload.email,
	};

	(StatusCode::CREATED, Json(created_user)).into_response()
}