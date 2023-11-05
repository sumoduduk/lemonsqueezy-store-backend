use axum::{http::StatusCode, Json};
use lemonsqueezy::orders::OrderResponse;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::utils::json_to_vec::json_to_vec;

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookPayload {
    meta: MetaObject,
    data: DataWebhookObject,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaObject {
    test_mode: Option<bool>,
    event_name: String,
    costum_data: Option<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataWebhookObject {
    r#type: String,
    id: String,
    attributes: OrderResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtractedWebhookData {
    pub id: String,
    pub test_mode: bool,
    pub costum_data: Option<Value>,
    pub costumer_id: Option<i64>,
    pub store_id: Option<i64>,
    pub user_email: Option<String>,
    pub user_name: Option<String>,
    pub currency: String,
    pub paid: bool,
    pub created_at: String,
    pub updated_at: String,
}

fn extract_payload(payload: WebhookPayload) -> ExtractedWebhookData {
    let data_extract = ExtractedWebhookData {
        id: payload.data.attributes.identifier,
        test_mode: payload.meta.test_mode.is_some_and(|x| x),
        costumer_id: payload.data.attributes.customer_id,
        store_id: payload.data.attributes.store_id,
        user_email: payload.data.attributes.user_email,
        user_name: payload.data.attributes.user_name,
        currency: payload.data.attributes.currency,
        paid: payload.data.attributes.status.is_some_and(|x| x == "paid"),
        created_at: payload.data.attributes.created_at,
        updated_at: payload.data.attributes.updated_at,
        costum_data: payload.meta.costum_data,
    };

    return data_extract;
}

pub async fn insert_to_db(payload: WebhookPayload) {
    let extracted = extract_payload(payload);

    dbg!(&extracted);
    if !extracted.test_mode && extracted.paid {
        if let Some(data_custom) = &extracted.costum_data {
            let arr_custom = json_to_vec::<String>(data_custom.clone())
        }
    }
}
