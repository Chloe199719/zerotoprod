use actix_web::{post, HttpResponse, Responder, web};
use sqlx:: PgPool;


#[derive(serde::Deserialize)]

pub struct FormData{
    pub email: String,
    pub name: String,
}


#[post("/subscribe")]
pub async fn subscribe(form: web::Form<FormData>, connection: web::Data<PgPool>) -> impl Responder {
    
    match  
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        uuid::Uuid::new_v4(),
        form.email,
        form.name,
        chrono::Utc::now(),
    ).execute(connection.get_ref()).await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) =>{
            eprintln!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        } 
    
    }
        
    
}