use serde::Deserialize;
use serde::Serialize;

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Option<String>,
    pub name: String,
    pub bgg_username: String,
    pub email: String,
}
