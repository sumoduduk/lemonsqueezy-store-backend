mod constant_time;
pub mod json_to_vec;
pub mod time_manipulation;
pub mod vec_to_json;

use eyre::eyre;
use serde_json::{json, Value};

use crate::db_model::DataDB;

pub fn extract_image(data: &[DataDB]) -> Vec<String> {
    let data_len = data.len();
    let mut arr_img = Vec::with_capacity(data_len);

    for image in data {
        arr_img.push(image.thumb_image.clone())
    }

    arr_img
}

pub fn make_custom_data(key_ids: &[String], user_id: &str) -> eyre::Result<Value> {
    let arr_str = serde_json::to_string(key_ids)?;

    let data = json!({
    "user_id" : user_id,
    "key_ids": arr_str
    });

    Ok(data)
}

pub fn extract_custom_data(data: Value) -> eyre::Result<(String, Vec<String>)> {
    let user_id = data["user_id"]
        .as_str()
        .ok_or_else(|| eyre!("user_id not found"))?;
    let key_ids = data["key_ids"]
        .as_str()
        .ok_or_else(|| eyre!("key_ids not found"))?;

    let key_ids = serde_json::from_str::<Vec<String>>(key_ids)?;

    Ok((user_id.to_string(), key_ids))
}

#[test]
fn test_costum_data_1() -> eyre::Result<()> {
    let json_data = json!({
    "key_ids" : r#"["ani","budi"]"#,
    "user_id": "abcde"
    });

    let key_ids = &[String::from("ani"), String::from("budi")];
    let user_id = "abcde";

    let res = make_custom_data(key_ids, user_id)?;

    assert_eq!(res, json_data);
    Ok(())
}

#[test]
fn test_extract_custom_data_1() -> eyre::Result<()> {
    let json_data = json!({
    "key_ids" : r#"["ani","budi"]"#,
    "user_id": "abcde"
    });

    let result = extract_custom_data(json_data)?;

    let arr_keys = vec!["ani".to_owned(), "budi".to_owned()];
    let user_id = "abcde".to_string();

    assert_eq!(result, (user_id, arr_keys));
    Ok(())
}

