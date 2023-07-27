pub mod configuration;
pub mod routes;
pub mod startup;

use actix_web::{ App,  HttpServer};
use routes::subscribe;
use routes::health_check;




pub async fn run(address:(String, u16)) -> std::io::Result<()> {
   
    HttpServer::new(|| {
        App::new()

            .service(health_check)
            .service(subscribe)
    })
    .bind(address)?.run().await
}