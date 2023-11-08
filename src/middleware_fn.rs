use std::env;

use axum::{
    body::{self, BoxBody, Full},
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};

use dotenvy::dotenv;
use hmac::{Hmac, Mac};
use sha2::Sha256;

pub async fn get_sig(req: Request<BoxBody>, next: Next<BoxBody>) -> Result<Response, Response> {
    dotenv().ok();

    let sig_val = env::var("SIG_VALUE").expect("No SIG_VALUE found");

    let (parts, body_parts) = req.into_parts();

    let sig_header = parts.headers.get("X-Signature");

    match sig_header {
        Some(signature) => {
            let mut mac = Hmac::<Sha256>::new_from_slice(sig_val.as_bytes()).map_err(|err| {
                (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
            })?;

            let bytes_body = hyper::body::to_bytes(body_parts).await.map_err(|err| {
                (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
            })?;

            let bytes_clone = bytes_body.clone();

            mac.update(&bytes_clone[..]);

            let sig = signature.as_bytes();
            let result = mac.verify_slice(sig);

            match result {
                Ok(_) => {
                    let new_req = Request::from_parts(parts, body::boxed(Full::from(bytes_body)));
                    Ok(next.run(new_req).await)
                }
                Err(_) => Err((StatusCode::NOT_ACCEPTABLE).into_response()),
            }
        }
        None => Err((StatusCode::UNAUTHORIZED).into_response()),
    }
}
