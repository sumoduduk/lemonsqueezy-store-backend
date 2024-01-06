use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    aud: String,
    exp: usize,
    iat: usize,
    iss: String,
    sub: String,
    email: String,
    phone: String,
    app_metadata: serde_json::Value,
    user_metadata: serde_json::Value,
    role: String,
    aal: String,
    amr: Vec<serde_json::Value>,
    session_id: String,
}

pub fn decode_jwt(token: &str, jwt_value: &str) -> eyre::Result<(String, String)> {
    let secret = DecodingKey::from_secret(jwt_value.as_ref());

    let mut validation = Validation::new(Algorithm::HS256);
    // fix : for test
    // validation.validate_exp = false;
    validation.set_audience(&["authenticated"]);

    let res = decode::<Claims>(token, &secret, &validation)?;

    let data = res.claims;

    Ok((data.email, data.sub))
}

