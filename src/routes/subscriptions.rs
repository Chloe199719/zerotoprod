use actix_web::{post, HttpResponse, Responder, web};


#[derive(serde::Deserialize)]

pub struct FormData{
    pub email: String,
    pub name: String,
}


#[post("/subscribe")]
pub async fn subscribe(_form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok()
    
}