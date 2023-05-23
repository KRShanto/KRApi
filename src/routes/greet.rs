use crate::*;

/// Show a greeting message
///
/// This route is used to confirm that the server is running successfully.
///
/// ## Route
///
/// `GET` localhost:8090/greet
#[get("/greet")]
pub async fn route() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world! Welcome to KR Api!")
}
