use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        if !request.headers().contains("Origin") {
            return;
        }

        println!("Request: {:?}", request.headers());

        let origin = request.headers().get_one("origin").unwrap();

        let env = std::env::var("ALLOWED_ORIGINS");
        let allowed_origins = match env.as_deref() {
            Ok(v) => v.split(',').map(|s| s.trim()).collect::<Vec<&str>>(),
            Err(_) => return,
        };

        if allowed_origins.contains(&origin) {
            println!("Allowing origin: {}", origin);
            response.set_header(Header::new("Access-Control-Allow-Origin", origin));
        }

        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PUT, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));

        println!("Response Headers {:?}", response.headers());
    }
}

/// Catches all OPTION requests in order to get the CORS related Fairing triggered.
#[options("/<_..>")]
#[allow(dead_code)]
#[allow(clippy::let_unit_value)]
pub fn all_options() {
    /* Intentionally left empty */
}
