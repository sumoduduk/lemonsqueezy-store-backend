mod jwt_fn;
mod webhook_router;

use axum::{extract::State, http::StatusCode, Json};
use lemonsqueezy::{checkout::CheckoutResponse, utils::Response};
use serde::Deserialize;
use tokio::task;

use crate::{
    db_model::{DataDB, Operation, OperationResult},
    internal_error,
    lemon_fn::create_checkout,
    AppState,
};

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
}

//mock
pub async fn mock_checkout(
    Json(payload): Json<CheckoutPayload>,
) -> Result<String, (StatusCode, String)> {
    dbg!(payload);

    Ok("OK".to_string())
}

pub async fn get_all(
    State(state): State<AppState>,
) -> Result<Json<Vec<DataDB>>, (StatusCode, String)> {
    let pool = state.pool;

    let res = Operation::GetData
        .execute(&pool)
        .await
        .map_err(internal_error);

    match res {
        Ok(response) => match response {
            OperationResult::Fetched(data) => Ok(Json(data)),
            OperationResult::Inserted => {
                Err((StatusCode::NOT_FOUND, "Something went wrong".to_string()))
            }
        },
        Err(err) => {
            dbg!(err);
            Err((StatusCode::NOT_FOUND, "Something went wrong".to_string()))
        }
    }
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
            )
            .await;

            match checkout_res {
                Ok(res) => Ok(Json(res)),

                Err(_) => Err((StatusCode::NOT_FOUND, "Something went wrong".to_string())),
            }
        }
        Err(_) => Err((StatusCode::UNAUTHORIZED, "Not Authorized".to_owned())),
    }
}

pub async fn get_by_id(
    State(state): State<AppState>,
    Json(payload): Json<CheckoutPayload>,
) -> Result<String, (StatusCode, String)> {
    //code
    let ids = payload.ids;
    dbg!(&ids);
    let res = Operation::GetDataById(ids)
        .execute(&state.pool)
        .await
        .map_err(internal_error);

    match res {
        Ok(data) => match data {
            OperationResult::Fetched(result) => match serde_json::to_string_pretty(&result) {
                Ok(hasil) => Ok(hasil),
                Err(_) => Err((StatusCode::NOT_FOUND, "error parse to pretty".to_string())),
            },
            OperationResult::Inserted => {
                Err((StatusCode::NOT_FOUND, "Something went wrong".to_string()))
            }
        },
        Err(err) => {
            dbg!(err);
            Err((StatusCode::NOT_FOUND, "Something went wrong".to_string()))
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

