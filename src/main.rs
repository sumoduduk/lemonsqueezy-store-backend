mod db_model;
mod lemon_fn;
mod middleware_fn;
mod router;
mod utils;

use axum::{
    body,
    http::{HeaderValue, Method},
    middleware,
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use lemonsqueezy::LemonSqueezy;
use serde::Serialize;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    ServiceBuilderExt,
};

use crate::{
    middleware_fn::get_sig,
    router::{checkout_url, webhook_route},
};

type PoolPg = Pool<Postgres>;

#[derive(Clone)]
pub struct AppState {
    pool: PoolPg,
    lemon: LemonSqueezy,
    redirect_uri: String,
    jwt_value: String,
    sig_val: String,
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_secrets::Secrets] secrets: shuttle_secrets::SecretStore,
) -> shuttle_axum::ShuttleAxum {
    let redirect_uri = secrets
        .get("REDIRECT_URI")
        .expect("REDIRECT_URI not present");

    let remote = redirect_uri.parse::<HeaderValue>().unwrap();
    let local = "http://localhost:4004".parse::<HeaderValue>().unwrap();

    let allowed = [remote, local];

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any)
        .allow_origin(allowed);

    let database_uri = secrets
        .get("DATABASE_URL")
        .expect("No database uri on environment");

    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_uri)
        .await
        .expect("Failed to create a pool postgress");

    let lemon_api = secrets.get("LEMON_API").expect("LEMON_API not present");
    let jwt_value = secrets.get("JWT_VALUE").expect("JWT_VALUE are not present");
    let sig_val = secrets.get("SIG_VALUE").expect("No SIG_VALUE found");

    let lemonsqueezy = lemonsqueezy::LemonSqueezy::new(lemon_api);

    let app_state = AppState {
        pool,
        lemon: lemonsqueezy,
        redirect_uri,
        jwt_value,
        sig_val,
    };

    let app = Router::new()
        .route("/webhook", post(webhook_route))
        .layer(
            ServiceBuilder::new()
                .map_request_body(body::boxed)
                .layer(middleware::from_fn_with_state(app_state.clone(), get_sig)),
        )
        .route("/", get(home))
        .route("/checkout", post(checkout_url))
        .with_state(app_state)
        .layer(cors);

    Ok(app.into())
}

#[derive(Serialize)]
struct HomeStruct {
    response: String,
    message: String,
}

async fn home() -> Json<HomeStruct> {
    println!("waking up");
    Json(HomeStruct {
        response: "OKE".to_string(),
        message: "Conection Successfull".to_string(),
    })
}

fn one_hour_from_now() -> String {
    let tomorrow = Utc::now()
        .checked_add_signed(chrono::Duration::minutes(60))
        .expect("Valid date");
    tomorrow.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()
}

