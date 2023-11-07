mod db_model;
mod lemon_fn;
mod middleware;
mod router;
mod utils;

use std::{env, net::SocketAddr};

use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use dotenvy::dotenv;
use lemonsqueezy::LemonSqueezy;
use serde::Serialize;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tower_http::cors::{Any, CorsLayer};

use crate::router::{checkout_url, get_all, get_by_id, webhook_route};

type PoolPg = Pool<Postgres>;

#[derive(Clone)]
pub struct AppState {
    pool: PoolPg,
    lemon: LemonSqueezy,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_origin(Any);

    let database_uri = env::var("DATABASE_URL").expect("No database uri on environment");
    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_uri)
        .await
        .expect("Failed to create a pool postgress");

    let lemon_api = env::var("LEMON_API").expect("LEMON_API not present");

    let lemonsqueezy = lemonsqueezy::LemonSqueezy::new(lemon_api);

    let app_state = AppState {
        pool,
        lemon: lemonsqueezy,
    };

    let app = Router::new()
        .route("/", get(home))
        .route("/get_all", get(get_all))
        .route("/checkout", post(checkout_url))
        .route("/data_id", post(get_by_id))
        .route("/webhook", post(webhook_route))
        .with_state(app_state)
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on {addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize)]
struct HomeStruct {
    response: String,
    message: String,
}

async fn home() -> Json<HomeStruct> {
    Json(HomeStruct {
        response: "OKE".to_string(),
        message: "Conection Successfull".to_string(),
    })
}

fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

fn one_hour_from_now() -> String {
    let tomorrow = Utc::now()
        .checked_add_signed(chrono::Duration::minutes(60))
        .expect("Valid date");
    tomorrow.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string()
}

