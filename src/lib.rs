use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, post};

#[get("/greet")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/greet/{name}")]
async fn greet(path: web::Path<(String,)>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello 4th deploy {}!", path.0))
}

#[get("/health-check")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("I am alive!")
}


#[derive(serde::Deserialize)]

pub struct FormData{
    pub email: String,
    pub name: String,
}


#[post("/subscribe")]
pub async fn subscribe(_form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok()
    
}



pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(greet)
            .service(health_check)
            .service(subscribe)
    })
    .bind(("0.0.0.0", 3000))?.run().await
}