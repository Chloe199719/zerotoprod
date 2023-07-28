use actix_web::{ Responder, HttpResponse, get };

#[get("/health-check")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("I am alive!")
}
