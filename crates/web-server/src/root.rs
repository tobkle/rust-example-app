use crate::errors::CustomError;
use axum::{Extension, Json};
use clorinde::{deadpool_postgres::Pool, queries::users::User};
use serde::Serialize;

#[derive(Serialize)]
pub struct UserResponse {
    id: i32,
    email: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
        }
    }
}

#[axum::debug_handler]
pub async fn loader(
    Extension(pool): Extension<Pool>,
) -> Result<Json<Vec<UserResponse>>, CustomError> {
    let client = pool.get().await?;

    let users = clorinde::queries::users::get_users()
        .bind(&client)
        .all()
        .await?;

    let user_responses: Vec<UserResponse> = users.into_iter()
        .map(UserResponse::from)
        .collect();

    Ok(Json(user_responses))
}