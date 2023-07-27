
use std::collections::HashMap;
use sqlx::Executor;
use actix_web::web;
use actix_web::{test, App, http::header::ContentType};
use sqlx::PgPool;
use zerotoprod::routes::subscribe;
use zerotoprod::routes::health_check;
use zerotoprod::configuration::{get_configuration, DatabaseSettings};
use sqlx::{ PgConnection, Connection};


#[actix_web::test]
async fn test_health_check() {
    let app = test::init_service(App::new().service(health_check)).await;
    let req = test::TestRequest::get().uri("/health-check").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // create database
   let mut connection = PgConnection::connect(&config.connection_string_without_db()).await.expect("Failed to connect to Postgres. create database");
   connection.execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name)).await.expect("Failed to create database.");
    // migrate database
    let connection_pool = PgPool::connect(&config.connection_string()).await.expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations").run(&connection_pool).await.expect("Failed to migrate database.");
    // return connection pool
    connection_pool
   
}
async fn drop_test_db(config: &DatabaseSettings) {
    let mut connection = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres.");

    connection
        .execute(&*format!(r#"DROP DATABASE "{}";"#, config.database_name))
        .await
        .expect("Failed to drop test database.");
}


#[actix_web::test]
async fn subscribe_returns_a_200_for_valid_from_data(){
    let mut configuration = get_configuration().expect("Failed to read configuration.");
    configuration.database.database_name = "test".to_string();

    drop_test_db(&configuration.database).await;
    let connection_pool1 =  configure_database(&configuration.database).await;
    let connection_pool = web::Data::new(connection_pool1.clone());


    // let mut connection = PgConnection::connect(&connection_string).await.expect("Failed to connect to Postgres.");



    // let data = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    
    let map : HashMap<&str, &str> = [("name", "le guin"), ("email", "ursula_le_guin%40gmail.com")].iter().cloned().collect();
    let app = test::init_service(App::new().service(subscribe).app_data(connection_pool)).await;
    let req = test::TestRequest::post().uri("/subscribe").insert_header(ContentType::form_url_encoded()).set_form(&map).to_request();
    let resp = test::call_service(&app, req).await;
    println!("{:?}", resp.status());
    assert!(resp.status().is_success());
    std::env::set_var("DATABASE_URL", configuration.database.connection_string());
    let saved = sqlx::query!("SELECT email, name FROM subscriptions",).fetch_one(&connection_pool1).await.expect("Failed to fetch saved subscription.");
    let email = map.get("email").unwrap().to_string();
    let name = map.get("name").unwrap().to_string();
    assert_eq!(saved.email, email);
    assert_eq!(saved.name, name);
}

#[actix_web::test]
async fn subscribe_returns_a_400_when_data_is_missing(){
    let test1: HashMap<&str, &str> = [("name", "le guin")].iter().cloned().collect();
    let test2: HashMap<&str, &str> = [("email", "ursula_le_guin%40gmail.com")].iter().cloned().collect();
    let test3: HashMap<&str, &str> = [].iter().cloned().collect();

    let test_cases = vec![
        
        (test1, "missing the email"),
        (test2, "missing the name"),
        (test3, "missing both name and email"),
    ];
    let app = test::init_service(App::new().service(subscribe)).await;

    for test in test_cases{
        let req = test::TestRequest::post().uri("/subscribe").insert_header(ContentType::form_url_encoded()).set_form(test.0).to_request();
        let resp = test::call_service(&app, req).await;
       assert_eq!(resp.status().as_u16(), 400, "The API did not fail with 400 Bad Request when the payload was {}.", test.1)
    }


}