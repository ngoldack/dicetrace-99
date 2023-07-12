use rocket::{catchers, launch, routes};
use rs_rocket_catcher::catcher::{bad_request, internal_error, not_found};

mod db;
mod group_handler;

#[launch]
async fn rocket() -> _ {
    dotenv::dotenv().expect("Failed to load .env file");

    log::info!("Starting server");

    let pool = db::connect().await;

    println!("cargo:rerun-if-changed=migrations");

    rocket::build()
        .manage::<sqlx::PgPool>(pool)
        .attach(rs_rocket_cors::cors::CORS)
        .register("/", catchers![not_found, internal_error, bad_request])
        .mount(
            "/api/v1/group",
            routes![
                group_handler::get_group,
                group_handler::get_groups,
                group_handler::create_group
            ],
        )
        .mount("/", routes![rs_rocket_cors::cors::all_options])
}
