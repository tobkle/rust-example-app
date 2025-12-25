use crate::errors::CustomError;
use axum::{
    http::StatusCode, 
    response::{Html, IntoResponse, Redirect, Response}, 
    Extension
};
use axum_extra::extract::Form;
use clorinde::{deadpool_postgres::Pool};
use serde::Deserialize;
use validator::Validate;

pub async fn loader(Extension(pool): Extension<Pool>) -> Result<Html<String>, CustomError> {
    let client = pool.get().await?;

    let users = clorinde::queries::users::get_users().bind(&client).all().await?;

    let html = web_pages::root::index(users);

    Ok(Html(html))
}

#[derive(Deserialize, Validate)]
pub struct SignUp {
    #[validate(email)] 
    email: String,
}

// 👇 handle form submission
pub async fn new_user_action(
    Extension(pool): Extension<Pool>,
    Form(form): Form<SignUp>,
) -> Result<Response, CustomError> {
    if form.validate().is_err() {
        return Ok((StatusCode::BAD_REQUEST, "Bad request").into_response());
    }

    let client = pool.get().await?;

    let email = form.email;

    let _ = clorinde::queries::users::create_user()
        .bind(&client, &email.as_str())
        .await?;

    // 303 redirect to users list
    Ok(Redirect::to("/").into_response())
}

// JSON response example
//
// #[derive(Serialize)]
// pub struct UserResponse {
//     id: i32,
//     email: String,
// }

// impl From<User> for UserResponse {
//     fn from(user: User) -> Self {
//         Self {
//             id: user.id,
//             email: user.email,
//         }
//     }
// }

// #[axum::debug_handler]
// pub async fn loader(
//     Extension(pool): Extension<Pool>,
// ) -> Result<Json<Vec<UserResponse>>, CustomError> {

//     let client = pool.get().await?;

//     let users = clorinde::queries::users::get_users()
//         .bind(&client)
//         .all()
//         .await?;

//     let user_responses: Vec<UserResponse> = users.into_iter()
//         .map(UserResponse::from)
//         .collect();

//     Ok(Json(user_responses))
// }