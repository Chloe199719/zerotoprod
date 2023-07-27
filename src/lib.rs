pub mod configuration;
pub mod routes;
pub mod startup;

use actix_web::web;
use actix_web::{ App,  HttpServer};
use routes::subscribe;
use routes::health_check;
use sqlx:: PgPool;




pub async fn run(address:(String, u16), db_pool:PgPool) -> std::io::Result<()> {
   let db_pool = web::Data::new(db_pool);
    HttpServer::new(move|| {
        App::new()

            .service(health_check)
            .service(subscribe)
            .app_data(db_pool.clone())
    })
    .bind(address)?.run().await
}