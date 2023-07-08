use rocket::{catchers, launch, routes};

mod db;
mod error_handler;
mod responses;
mod user_handler;

use error_handler::{bad_request, internal_error, not_found};
use log;
use tracing_log::LogTracer;

#[launch]
async fn rocket() -> _ {
    LogTracer::init().expect("Failed to set logger");

    log::info!("Starting server");

    dotenv::dotenv().expect("Failed to load .env file");
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
