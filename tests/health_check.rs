
use std::collections::HashMap;

use actix_web::{test, App, http::header::ContentType};
use zerotoprod::{health_check, subscribe};

#[actix_web::test]
async fn test_health_check() {
    let app = test::init_service(App::new().service(health_check)).await;
    let req = test::TestRequest::get().uri("/health-check").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

}

#[actix_web::test]
async fn subscribe_returns_a_200_for_valid_from_data(){

    // let data = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let map : HashMap<&str, &str> = [("name", "le guin"), ("email", "ursula_le_guin%40gmail.com")].iter().cloned().collect();
    let app = test::init_service(App::new().service(subscribe)).await;
    let req = test::TestRequest::post().uri("/subscribe").insert_header(ContentType::form_url_encoded()).set_form(map).to_request();
    let resp = test::call_service(&app, req).await;
    println!("{:?}", resp.status());
    assert!(resp.status().is_success());
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