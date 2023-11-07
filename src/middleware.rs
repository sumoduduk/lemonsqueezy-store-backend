use std::env;

use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

use dotenvy::dotenv;
use hex_literal::hex;
use hmac::{Hmac, Mac};
use sha2::Sha256;

pub async fn get_sig<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    dotenv().ok();

    let sig_val = env::var("SIG_VALUE").expect("No SIG_VALUE found");

    let (part, bofy) = req.into_parts

    let sig_header = req.headers().get("X-Signature");
    let body = req.body();

    match sig_header {
        Some(signature) => {
            let mut mac = Hmac::<Sha256>::new_from_slice(sig_val.as_bytes());
            match mac {
                Ok(mac_value) => {
                    mac_value.update(body);
                    let digest = mac_value.finalize().into_bytes();

                    let code_bytes = hex!(signature);

                    let result = mac_value.verify_slice(&code_bytes[..]);

                    match result {
                        Ok(_) => Ok(next.run(req).await),
                        Err(_) => Err(StatusCode::NOT_ACCEPTABLE),
                    }
                }
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
        None => Err(StatusCode::UNAUTHORIZED),
    }
}

