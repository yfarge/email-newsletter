use actix_web::{web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::SubscriptionToken;

#[derive(Deserialize)]
pub struct Parameters {
    subscription_token: String,
}

impl TryFrom<Parameters> for SubscriptionToken {
    type Error = String;

    fn try_from(value: Parameters) -> Result<Self, Self::Error> {
        SubscriptionToken::parse(value.subscription_token)
    }
}

#[tracing::instrument(name = "Confirm a pending subscriber.", skip(_parameters, db_pool))]
pub async fn confirm(
    _parameters: web::Query<Parameters>,
    db_pool: web::Data<PgPool>,
) -> HttpResponse {
    let subscription_token = match _parameters.0.try_into() {
        Ok(subscription_token) => subscription_token,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

    let id = match get_subscriber_id_from_token(&db_pool, subscription_token).await {
        Ok(subscriber_id) => subscriber_id,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match id {
        None => HttpResponse::NotFound().finish(),
        Some(id) => {
            if confirm_subscriber(&db_pool, id).await.is_err() {
                return HttpResponse::InternalServerError().finish();
            }
            HttpResponse::Ok().finish()
        }
    }
}

#[tracing::instrument(name = "Mark subscriber as confirmed", skip(db_pool, subscriber_id))]
pub async fn confirm_subscriber(db_pool: &PgPool, subscriber_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE subscriptions SET status = 'confirmed' WHERE id = $1"#,
        subscriber_id
    )
    .execute(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query {:?}.", e);
        e
    })?;
    Ok(())
}

#[tracing::instrument(
    name = "Get subscriber_id from token",
    skip(db_pool, subscription_token)
)]
pub async fn get_subscriber_id_from_token(
    db_pool: &PgPool,
    subscription_token: SubscriptionToken,
) -> Result<Option<Uuid>, sqlx::Error> {
    let result = sqlx::query!(
        r#"SELECT subscriber_id from subscription_tokens
        WHERE subscription_token = $1"#,
        subscription_token.as_ref()
    )
    .fetch_optional(db_pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query {:?}.", e);
        e
    })?;
    Ok(result.map(|r| r.subscriber_id))
}