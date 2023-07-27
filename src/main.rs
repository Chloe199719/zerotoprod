use sqlx:: PgPool;
use zerotoprod::run;
use zerotoprod::configuration::get_configuration;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
   let configuration = get_configuration().expect("Failed to read configuration.");
   let address = (configuration.application_host, configuration.application_port);
   let connection_pool =  PgPool::connect(&configuration.database.connection_string()).await.expect("Failed to connect to Postgres.");
   run(address, connection_pool).await
}



