use http_api_problem::*;
use rocket::post;
use rocket::{get, serde::json::Json};
use rs_models::group::Group;
use rs_rocket_response::responses::OkResponse;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
struct GroupModel {
    id: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GroupUserModel {
    group_id: String,
    user_id: String,
}

#[get("/<id>")]
pub async fn get_group(
    pool: &rocket::State<sqlx::PgPool>,
    id: String,
) -> Result<Json<OkResponse<Group>>, HttpApiProblem> {
    log::debug!("Getting group with id: {}", id);

    let result_group = sqlx::query_as!(GroupModel, "SELECT id, name FROM groups WHERE id = $1", id)
        .fetch_one(pool.inner())
        .await;

    let result_group_users = sqlx::query_as!(
        GroupUserModel,
        "SELECT group_id, user_id FROM group_members WHERE group_id = $1",
        id,
    )
    .fetch_all(pool.inner())
    .await;

    let users = match result_group_users {
        Ok(v) => v.into_iter().map(|gum| gum.user_id).collect(),
        Err(e) => {
            return Err(HttpApiProblem::from(StatusCode::INTERNAL_SERVER_ERROR)
                .value("error", &e.to_string()))
        }
    };

    let result = match result_group {
        Ok(gm) => Group {
            id: Some(gm.id),
            name: gm.name,
            users,
        },
        Err(e) => {
            return Err(HttpApiProblem::from(StatusCode::INTERNAL_SERVER_ERROR)
                .value("error", &e.to_string()))
        }
    };

    Ok(Json(OkResponse {
        code: StatusCode::OK.into(),
        size: 1,
        data: result,
    }))
}

#[get("/")]
pub async fn get_groups(
    pool: &rocket::State<sqlx::PgPool>,
) -> Result<Json<OkResponse<Vec<Group>>>, HttpApiProblem> {
    log::debug!("Getting all groups");

    let result_group = sqlx::query_as!(GroupModel, "SELECT id, name FROM groups")
        .fetch_all(pool.inner())
        .await;

    let result_group_users = sqlx::query_as!(
        GroupUserModel,
        "SELECT group_id, user_id FROM group_members",
    )
    .fetch_all(pool.inner())
    .await;

    let groups = match result_group {
        Ok(v) => {
            let mut groups: Vec<Group> = v
                .into_iter()
                .map(|gm| Group {
                    id: Some(gm.id),
                    name: gm.name,
                    users: vec![],
                })
                .collect();

            match result_group_users {
                Ok(user_models) => {
                    for group_user in user_models {
                        let group_id = group_user.group_id.clone(); // Clone the group ID
                        if let Some(group) =
                            groups.iter_mut().find(|g| g.id == Some(group_id.clone()))
                        {
                            group.users.push(group_user.user_id);
                        }
                    }
                }
                Err(e) => {
                    return Err(HttpApiProblem::from(StatusCode::INTERNAL_SERVER_ERROR)
                        .value("error", &e.to_string()))
                }
            }

            groups
        }
        Err(e) => {
            return Err(HttpApiProblem::from(StatusCode::INTERNAL_SERVER_ERROR)
                .value("error", &e.to_string()))
        }
    };

    Ok(Json(OkResponse {
        code: StatusCode::OK.into(),
        size: groups.len(),
        data: groups,
    }))
}

#[post("/", data = "<group>")]
pub async fn create_group(
    pool: &rocket::State<sqlx::PgPool>,
    mut group: Json<Group>,
) -> Result<Json<OkResponse<Group>>, HttpApiProblem> {
    log::debug!("Creating group: {:?}", group);

    group.id = Some(rs_id_gen::gen());

    let result: Result<_, sqlx::Error> = sqlx::query!(
        "INSERT INTO groups (id, name) VALUES ($1, $2)",
        group.id,
        group.name,
    )
    .execute(pool.inner())
    .await;

    match result {
        Ok(_) => {
            // Insert group members
            let group_id = group.id.clone().unwrap(); // Clone the group ID
            let user_ids: Vec<&str> = group.users.iter().map(|user_id| user_id.as_str()).collect();

            let insert_members_result = sqlx::query!(
                "INSERT INTO group_members (group_id, user_id) SELECT $1, unnest($2::text[])",
                group_id,
                &user_ids as &[&str],
            )
            .execute(pool.inner())
            .await;

            if let Err(e) = insert_members_result {
                return Err(HttpApiProblem::from(StatusCode::INTERNAL_SERVER_ERROR)
                    .value("error", &e.to_string()));
            }

            Ok(Json(OkResponse {
                code: StatusCode::OK.into(),
                size: 1,
                data: group.into_inner(),
            }))
        }
        Err(e) => match e.as_database_error() {
            Some(db_err) if db_err.is_unique_violation() => {
                Err(HttpApiProblem::from(StatusCode::BAD_REQUEST)
                    .title("Bad Request")
                    .detail("Group already exists"))
            }
            _ => Err(HttpApiProblem::from(StatusCode::INTERNAL_SERVER_ERROR)
                .value("error", &e.to_string())),
        },
    }
}
