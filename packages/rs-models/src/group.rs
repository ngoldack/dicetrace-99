use serde::Deserialize;
use serde::Serialize;

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct Group {
    pub id: Option<String>,
    pub name: String,
    pub description: String,
    pub users: Vec<String>,
}
