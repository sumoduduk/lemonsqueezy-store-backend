mod webhook_router;

use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use serde_json::Value;

use crate::{
    db_model::{DataDB, Operation},
    internal_error,
    lemon_fn::create_checkout,
    AppState,
};

use self::webhook_router::WebhookPayload;

#[derive(Debug, Deserialize)]
pub struct CheckoutPayload {
    ids: Vec<String>, //email
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
        Ok(response) => Ok(Json(response)),
        Err(err) => {
            dbg!(err);
            Err((StatusCode::NOT_FOUND, "Shit happen".to_string()))
        }
    }
}

pub async fn checkout_url(
    State(state): State<AppState>,
    Json(payload): Json<CheckoutPayload>,
) -> Result<String, (StatusCode, String)> {
    let ids = payload.ids;

    let checkout_res = create_checkout(&ids, state.lemon, &state.pool).await;

    match checkout_res {
        Ok(res) => match serde_json::to_string_pretty(&res) {
            Ok(pretty) => Ok(pretty),
            Err(_) => Err((StatusCode::NOT_FOUND, "Error parse to string".to_string())),
        },
        Err(_) => Err((StatusCode::NOT_FOUND, "Shit happen".to_string())),
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
        Ok(data) => match serde_json::to_string_pretty(&data) {
            Ok(hasil) => Ok(hasil),
            Err(_) => Err((StatusCode::NOT_FOUND, "error parse to pretty".to_string())),
        },
        Err(err) => {
            dbg!(err);
            Err((StatusCode::NOT_FOUND, "Shit happen".to_string()))
        }
    }
}

pub async fn webhook_route(
    State(state): State<AppState>,
    Json(payload): Json<WebhookPayload>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    //code
    Ok((StatusCode::OK, "OK".to_string()))
}
