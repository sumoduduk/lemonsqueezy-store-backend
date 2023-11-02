use axum::{extract::State, http::StatusCode, Json};

use crate::{
    db_model::{DataDB, Operation},
    internal_error,
    lemon_fn::create_checkout,
    AppState,
};

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

pub async fn checkout_url(State(state): State<AppState>) -> Result<String, (StatusCode, String)> {
    //code
    let checkout_res = create_checkout(state.lemon, &state.pool).await;

    match checkout_res {
        Ok(res) => match serde_json::to_string_pretty(&res) {
            Ok(pretty) => Ok(pretty),
            Err(_) => Err((StatusCode::NOT_FOUND, "Shit happen".to_string())),
        },
        Err(_) => Err((StatusCode::NOT_FOUND, "Shit happen".to_string())),
    }
}
