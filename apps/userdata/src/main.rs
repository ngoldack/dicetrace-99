use rocket::{catchers, launch, routes};

mod db;
mod error_handler;
mod responses;
mod user_handler;

use error_handler::{bad_request, internal_error, not_found};

#[launch]
async fn rocket() -> _ {
    dotenv::dotenv().expect("Failed to load .env file");

    log::info!("Starting server");

    let pool = db::connect().await;

    println!("cargo:rerun-if-changed=migrations");

    rocket::build()
        .manage::<sqlx::MySqlPool>(pool)
        .attach(rs_rocket_cors::cors::CORS)
        .register("/", catchers![not_found, internal_error, bad_request])
        .mount(
            "/api/v1/user",
            routes![
                user_handler::get_users,
                user_handler::get_user,
                user_handler::create_user
            ],
        )
        .mount("/", routes![rs_rocket_cors::cors::all_options])
}
