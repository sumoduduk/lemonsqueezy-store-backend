use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{types::Uuid, FromRow, Pool, Postgres};

use crate::PoolPg;

#[derive(Debug, FromRow, Serialize)]
pub struct DataDB {
    pub key_id: String,
    title: String,
    pub thumb_image: String,
}

#[derive(Debug, Serialize)]
pub struct PaymentHistory<'a> {
    pub id: &'a str,
    pub customer_id: Option<i64>,
    pub store_id: Option<i64>,
    pub name: Option<&'a str>,
    pub currency: &'a str,
    pub paid: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub total_paid: Option<i64>,
    pub email: Option<&'a str>,
    pub key_id: Vec<String>,
    pub user_id: Uuid,
}

pub enum Operation<'a> {
    GetDataById(Vec<String>), // CreateHistory,
    InsertPaymentHisory(PaymentHistory<'a>),
}

pub enum OperationResult {
    Fetched(Vec<DataDB>),
    Inserted,
}

use OperationResult::*;

impl<'a> Operation<'a> {
    pub async fn execute(&self, pool: &Pool<Postgres>) -> Result<OperationResult, sqlx::Error> {
        match self {
            Self::GetDataById(arr) => {
                let return_data = Self::fetch_by_id(arr, pool).await?;
                Ok(Fetched(return_data))
            }
            Self::InsertPaymentHisory(data) => {
                Self::insert_history_payment(data, pool).await?;
                Ok(Inserted)
            }
        }
    }

    async fn fetch_by_id(ids: &[String], pool: &PoolPg) -> Result<Vec<DataDB>, sqlx::Error> {
        let data_response: Vec<DataDB> = sqlx::query_as::<_, DataDB>(
            "
            SELECT key_id, thumb_image, title FROM bride_photo_thumbnails 
            WHERE key_id = ANY($1)
            ",
        )
        .bind(ids)
        .fetch_all(pool)
        .await?;

        dbg!(&data_response);
        Ok(data_response)
    }

    async fn insert_history_payment(
        history_data: &PaymentHistory<'a>,
        pool: &Pool<Postgres>,
    ) -> Result<(), sqlx::Error> {
        let query_str = r#"
                    WITH inserted_data AS (
                      INSERT INTO bridebook_payment_history (
                        email,
                        invoice_id,
                        key_id,
                        name,
                        customer_id,
                        store_id,
                        currency,
                        paid,
                        created_at,
                        updated_at,
                        total_paid,
                        user_id
                      )
                      VALUES (
                        $1,
                        $2,
                        unnest($3::text[]),
                        $4,
                        $5,
                        $6,
                        $7,
                        $8,
                        $9,
                        $10,
                        $11,
                        $12
                      )
                      RETURNING key_id
                    )
                    UPDATE bride_photo_thumbnails
                    SET paid = true
                    WHERE key_id IN (
                    SELECT key_id
                    FROM inserted_data
                    );
                        "#;
        let row_total = sqlx::query(query_str)
            .bind(history_data.email)
            .bind(history_data.id)
            .bind(history_data.key_id.clone())
            .bind(history_data.name)
            .bind(history_data.customer_id)
            .bind(history_data.store_id)
            .bind(history_data.currency)
            .bind(history_data.paid)
            .bind(history_data.created_at)
            .bind(history_data.updated_at)
            .bind(history_data.total_paid)
            .bind(history_data.user_id)
            .execute(pool)
            .await?;
        dbg!(row_total);
        Ok(())
    }
}

