use http_api_problem::{HttpApiProblem, StatusCode};
use rocket::{catch, Request};

#[catch(500)]
pub fn internal_error() -> HttpApiProblem {
    HttpApiProblem::from(StatusCode::INTERNAL_SERVER_ERROR)
        .title("Internal Server Error")
        .detail("Something went wrong")
}

#[catch(400)]
pub fn bad_request() -> HttpApiProblem {
    
    HttpApiProblem::from(StatusCode::BAD_REQUEST)
        .title("Bad Request")
        .detail("The request was malformed")
}

#[catch(404)]
pub fn not_found(req: &Request) -> HttpApiProblem {
    HttpApiProblem::from(StatusCode::NOT_FOUND)
        .title("Not Found")
        .detail(format!("The requested path '{}' was not found", req.uri()))
}
