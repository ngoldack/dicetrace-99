use http_api_problem::*;
use rocket::{get, post, serde::json::Json};
use rs_models::user::User;

use crate::responses::OkResponse;

#[get("/<id>")]
pub async fn get_user(
    pool: &rocket::State<sqlx::MySqlPool>,
    id: i32,
) -> Result<Json<OkResponse<User>>, HttpApiProblem> {
    log::debug!("Getting user with id: {}", id);

    let result: Result<User, sqlx::Error> =
        sqlx::query_as!(User, "SELECT * FROM users WHERE id = ?", id)
            .fetch_one(pool.inner())
            .await;

    match result {
        Ok(v) => Ok(Json(OkResponse {
            code: StatusCode::OK.into(),
            size: 1,
            data: v,
        })),
        Err(e) => {
            Err(HttpApiProblem::from(StatusCode::INTERNAL_SERVER_ERROR)
                .value("error", &e.to_string()))
        }
    }
}

#[post("/", data = "<user>")]
pub async fn create_user(
    pool: &rocket::State<sqlx::MySqlPool>,
    mut user: Json<User>,
) -> Result<Json<OkResponse<User>>, HttpApiProblem> {
    log::debug!("Creating user: {:?}", user);

    user.id = Some(rs_id_gen::gen());

    let result: Result<_, sqlx::Error> = sqlx::query!(
        "INSERT INTO users (id, name, email, bgg_username) VALUES (?, ?, ?, ?)",
        user.id,
        user.name,
        user.email,
        user.bgg_username
    )
    .execute(pool.inner())
    .await;

    match result {
        Ok(_) => Ok(Json(OkResponse {
            code: StatusCode::OK.into(),
            size: 1,
            data: user.into_inner(),
        })),
        Err(e) => match e.as_database_error() {
            Some(db_err) if db_err.is_unique_violation() => {
                Err(HttpApiProblem::from(StatusCode::BAD_REQUEST)
                    .title("Bad Request")
                    .detail("The email address is already in use"))
            }
            _ => Err(HttpApiProblem::from(StatusCode::BAD_REQUEST).value("error", &e.to_string())),
        },
    }
}

#[get("/?<email>")]
pub async fn get_users(
    pool: &rocket::State<sqlx::MySqlPool>,
    email: Option<String>,
) -> Result<Json<OkResponse<Vec<User>>>, HttpApiProblem> {
    match email {
        Some(e) => {
            log::debug!("Getting user with email: {}", e);

            let result: Result<Vec<User>, sqlx::Error> =
                sqlx::query_as!(User, "SELECT * FROM users WHERE email = ?", e)
                    .fetch_all(pool.inner())
                    .await;

            match result {
                Ok(v) => Ok(Json(OkResponse {
                    code: StatusCode::OK.into(),
                    size: v.len(),
                    data: v,
                })),
                Err(e) => Err(HttpApiProblem::from(StatusCode::INTERNAL_SERVER_ERROR)
                    .value("error", &e.to_string())),
            }
        }
        None => Err(HttpApiProblem::from(StatusCode::BAD_REQUEST)
            .title("Bad Request")
            .detail("Query parameter 'email' is required")),
    }
}
