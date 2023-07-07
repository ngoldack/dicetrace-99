use http_api_problem::*;
use rocket::{get, post, serde::json::Json};
use rs_models::user::User;

use crate::responses::SuccessResponse;

#[get("/<id>")]
pub async fn get_user(
    pool: &rocket::State<sqlx::MySqlPool>,
    id: i32,
) -> Result<Json<SuccessResponse<User>>, HttpApiProblem> {
    let result: Result<User, sqlx::Error> =
        sqlx::query_as!(User, "SELECT * FROM users WHERE id = ?", id)
            .fetch_one(pool.inner())
            .await;

    match result {
        Ok(v) => {
            return Ok(Json(SuccessResponse {
                code: StatusCode::OK.into(),
                size: 1,
                data: v,
            }));
        }
        Err(e) => {
            return Err(HttpApiProblem::from(StatusCode::INTERNAL_SERVER_ERROR)
                .value("error", &e.to_string()));
        }
    }
}

#[post("/", data = "<user>")]
pub async fn create_user(
    pool: &rocket::State<sqlx::MySqlPool>,
    mut user: Json<User>,
) -> Result<Json<SuccessResponse<User>>, HttpApiProblem> {
    user.id = Some(id_gen::gen());

    let result: Result<_, sqlx::Error> = sqlx::query!(
        "INSERT INTO users (id, name, email) VALUES (?, ?, ?)",
        user.id,
        user.name,
        user.email,
    )
    .execute(pool.inner())
    .await;

    match result {
        Ok(_) => Ok(Json(SuccessResponse {
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

#[get("/")]
pub async fn get_users(
    pool: &rocket::State<sqlx::MySqlPool>,
) -> Result<Json<SuccessResponse<Vec<User>>>, HttpApiProblem> {
    let result: Result<Vec<User>, sqlx::Error> = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(pool.inner())
        .await;

    match result {
        Ok(v) => {
            return Ok(Json(SuccessResponse {
                code: StatusCode::OK.into(),
                size: v.len(),
                data: v,
            }));
        }
        Err(e) => {
            return Err(HttpApiProblem::from(StatusCode::INTERNAL_SERVER_ERROR)
                .value("error", &e.to_string()));
        }
    }
}
