use serde_json::{json, Value};

use crate::db_model::DataDB;

pub fn execute_to_json(data: &[DataDB]) -> Value {
    let mut json_object = json!({});

    for (i, elem) in data.iter().enumerate() {
        json_object[format!("id_{}", i + 1)] = json!(elem.key_id)
    }

    println!("key object_costum : {}", &json_object);
    json_object
}
