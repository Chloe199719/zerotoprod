use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(greet)
            .service(health_check)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}

#[get("/greet")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/greet/{name}")]
async fn greet(path: web::Path<(String,)>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello 4th deploy {}!", path.0))
}

#[get("/health-check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("I am alive!")
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    #[actix_web::test]
    async fn test_health_check() {
        let app = test::init_service(App::new().service(health_check)).await;
        let req = test::TestRequest::get().uri("/health-check").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
