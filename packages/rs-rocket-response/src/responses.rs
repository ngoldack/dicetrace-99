use rocket::serde::Serialize;

#[derive(Debug, Serialize)]
pub struct OkResponse<T> {
    pub data: T,
    pub code: u16,
    pub size: usize,
}
