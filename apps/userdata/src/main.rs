use rocket::{catchers, launch, routes};

mod db;
mod error_handler;
mod responses;
mod user_handler;

use error_handler::{bad_request, internal_error, not_found};

#[launch]
async fn rocket() -> _ {
    dotenv::dotenv().ok();
    rocket::build()
        .manage::<sqlx::MySqlPool>(db::connect().await)
        .register("/", catchers![not_found, internal_error, bad_request])
        .mount(
            "/api/v1/user",
            routes![
                user_handler::get_users,
                user_handler::get_user,
                user_handler::create_user
            ],
        )
}
