use serde::Serialize;
use sqlx::{FromRow, Pool, Postgres};

use crate::PoolPg;

#[derive(Debug, FromRow, Serialize)]
pub struct DataDB {
    key_id: String,
    title: String,
    pub thumb_image: String,
}

pub enum Operation {
    GetData,
    GetDataById(Vec<String>), // CreateHistory,
}

impl Operation {
    pub async fn execute(&self, pool: &Pool<Postgres>) -> Result<Vec<DataDB>, sqlx::Error> {
        match self {
            Self::GetData => {
                let return_data = Self::fetch_all(pool).await?;
                Ok(return_data)
            }
            Self::GetDataById(arr) => {
                let return_data = Self::fetch_by_id(arr, pool).await?;
                Ok(return_data)
            }
        }
    }

    async fn fetch_all(pool: &PoolPg) -> Result<Vec<DataDB>, sqlx::Error> {
        let data_response: Vec<DataDB> = sqlx::query_as::<_, DataDB>(
            "
            SELECT key_id, thumb_image, title FROM bride_photo_thumbnails 
            LIMIT 5
            ",
        )
        .fetch_all(pool)
        .await?;

        Ok(data_response)
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
}
