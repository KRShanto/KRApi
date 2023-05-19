use crate::*;

#[get("/greet")]
pub async fn route() -> HttpResponse {
    HttpResponse::Ok().body("Hello, world! Welcome to KR Api!")
}
