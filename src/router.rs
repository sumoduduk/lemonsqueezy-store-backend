use axum::{extract::State, http::StatusCode, Json};

use crate::{
    db_model::{DataDB, Operation},
    internal_error, AppState, PoolPg,
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

pub async fn checkout_url(State(pool): State<PoolPg>) {
    //code
}
