mod jwt_fn;
mod webhook_router;

use axum::{extract::State, http::StatusCode, Json};
use lemonsqueezy::{checkout::CheckoutResponse, utils::Response};
use serde::Deserialize;
use tokio::task;

use crate::{lemon_fn::create_checkout, AppState};

use self::{
    jwt_fn::decode_jwt,
    webhook_router::{insert_to_db, WebhookPayload},
};

#[derive(Debug, Deserialize)]
pub struct CheckoutPayload {
    ids: Vec<String>,
    token: String,
    name_product: String,
    description: String,
    variant_id: String,
}

pub async fn checkout_url(
    State(state): State<AppState>,
    Json(payload): Json<CheckoutPayload>,
) -> Result<Json<Response<CheckoutResponse>>, (StatusCode, String)> {
    let ids = payload.ids;

    let email_res = decode_jwt(&payload.token);
    match email_res {
        Ok((email, user_id)) => {
            let checkout_res = create_checkout(
                &ids,
                state.lemon,
                &state.pool,
                email,
                user_id,
                payload.name_product,
                payload.description,
                payload.variant_id,
                "50443".to_string(),
            )
            .await;

            match checkout_res {
                Ok(res) => Ok(Json(res)),

                Err(_) => Err((StatusCode::NOT_FOUND, "Something went wrong".to_string())),
            }
        }
        Err(err) => {
            println!(" error {}", err);
            Err((StatusCode::UNAUTHORIZED, "Not Authorized".to_owned()))
        }
    }
}

pub async fn webhook_route(
    State(state): State<AppState>,
    Json(payload): Json<WebhookPayload>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let pool = state.pool;
    dbg!(&payload);

    task::spawn(async move {
        let _ = insert_to_db(payload, &pool).await;
    });

    Ok((StatusCode::OK, "OK".to_string()))
}

