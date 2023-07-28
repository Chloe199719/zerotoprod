use actix_web::{ post, HttpResponse, Responder, web };
use sqlx::PgPool;
use tracing::Instrument;

#[derive(serde::Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

#[post("/subscribe")]
pub async fn subscribe(form: web::Form<FormData>, connection: web::Data<PgPool>) -> impl Responder {
    let request_id = uuid::Uuid::new_v4();

    let request_span =
        tracing::info_span!("Adding a new subscriber."
        , %request_id
        , email = %form.email
        , name = %form.name);
    let _request_span_guard = request_span.enter();
    let query_span = tracing::info_span!("Saving new subscriber details in the database.");
    // prettier-ignore
    match sqlx::query!(
                r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
                uuid::Uuid::new_v4(),
                form.email,
                form.name,
                chrono::Utc::now()
            )
            .execute(connection.get_ref()).instrument(query_span).await
    {
        Ok(_) => {
            tracing::info!("request_id {} - New subscriber details have been saved in the database.", request_id);
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            tracing::error!("request_id {} - Failed to execute query: {:?}", request_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
