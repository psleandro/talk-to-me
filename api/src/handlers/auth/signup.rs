use axum::{
	extract::Json,
	http::StatusCode,
	response::IntoResponse
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

#[derive(Deserialize)]
pub struct SignUp {
	name: String,
	email: String,
	password: String,
}

pub async fn signup(Json(payload): Json<SignUp>) -> Result<impl IntoResponse, StatusCode> {
	let created_user = User {
		id: 1,
		name: payload.name,
		email: payload.email,
	};

	Ok((StatusCode::CREATED, Json(created_user)))
}