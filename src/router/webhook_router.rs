use chrono::{DateTime, Utc};
use lemonsqueezy::orders::OrderResponse;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    db_model::{Operation, PaymentHistory},
    utils::{extract_custom_data, time_manipulation::parse_utc_datetime},
    PoolPg,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookPayload {
    meta: MetaObject,
    data: DataWebhookObject,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaObject {
    test_mode: Option<bool>,
    event_name: String,
    custom_data: Option<Value>,
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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub total_paid: Option<i64>,
}

fn extract_payload(payload: WebhookPayload) -> eyre::Result<ExtractedWebhookData> {
    let data_extract = ExtractedWebhookData {
        id: payload.data.attributes.identifier,
        test_mode: payload.meta.test_mode.is_some_and(|x| x),
        costumer_id: payload.data.attributes.customer_id,
        store_id: payload.data.attributes.store_id,
        user_email: payload.data.attributes.user_email,
        user_name: payload.data.attributes.user_name,
        currency: payload.data.attributes.currency,
        paid: payload.data.attributes.status.is_some_and(|x| x == "paid"),
        created_at: parse_utc_datetime(&payload.data.attributes.created_at)?,
        updated_at: parse_utc_datetime(&payload.data.attributes.updated_at)?,
        costum_data: payload.meta.custom_data,
        total_paid: payload.data.attributes.total,
    };

    Ok(data_extract)
}

pub async fn insert_to_db(payload: WebhookPayload, pool: &PoolPg) -> eyre::Result<()> {
    let extracted = extract_payload(payload);

    if let Ok(extracted_data) = extracted {
        dbg!(&extracted_data);
        if !extracted_data.test_mode && extracted_data.paid {
            if let Some(data_custom) = &extracted_data.costum_data {
                let (user_id, arr_custom) = extract_custom_data(data_custom.clone())?;
                let history = PaymentHistory {
                    id: &extracted_data.id,
                    customer_id: extracted_data.costumer_id,
                    store_id: extracted_data.store_id,
                    name: extracted_data.user_name.as_deref(),
                    currency: &extracted_data.currency,
                    paid: extracted_data.paid,
                    created_at: extracted_data.created_at,
                    updated_at: extracted_data.updated_at,
                    total_paid: extracted_data.total_paid,
                    email: extracted_data.user_email.as_deref(),
                    key_id: arr_custom,
                    user_id: &user_id,
                };

                let res = Operation::InsertPaymentHisory(history).execute(pool).await;

                match res {
                    Ok(_) => println!("inserted success"),
                    Err(err) => {
                        println!("insert to db error");
                        dbg!(err);
                    }
                }
            }
        }
    }
    Ok(())
}

