use actix_web::{ post, HttpResponse, Responder, web };
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String,
}
#[tracing::instrument(
    name = "Adding a new subscriber.",
    skip(form, connection),
    fields(
        request_id = %uuid::Uuid::new_v4(),
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
#[post("/subscribe")]
pub async fn subscribe(form: web::Form<FormData>, connection: web::Data<PgPool>) -> impl Responder {
    match insert_subscriber(&connection, &form).await {
        Ok(_) => { HttpResponse::Ok().finish() }
        Err(_) => { HttpResponse::InternalServerError().finish() }
    }
}

#[tracing::instrument(name = "Saving new subscriber details in the database.", skip(pool, form), fields(
    subscriber_email = %form.email,
    subscriber_name = %form.name
))]
pub async fn insert_subscriber(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    // prettier-ignore
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        uuid::Uuid::new_v4(),
        form.email,
        form.name,
        chrono::Utc::now()
    )
        .execute(pool).await
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            e
        })?;
    Ok(())
}
