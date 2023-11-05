use serde::de::DeserializeOwned;
use serde_json::Value;

pub fn json_to_vec<T>(json: Value) -> Vec<T>
where
    T: DeserializeOwned,
{
    let mut arr = Vec::new();
    if let Value::Object(map) = json {
        for (_, value) in map {
            if let Ok(v) = serde_json::from_value(value) {
                arr.push(v);
            }
        }
    }

    arr
}

#[cfg(test)]
mod tests {

    use super::*;
    use serde_json::json;

    #[test]
    fn test_empty_json() {
        let json = json!({});
        let vec = json_to_vec::<String>(json);
        assert_eq!(vec, Vec::<String>::new());
    }

    #[test]
    fn test_json_with_strings() {
        let json = json!({
          "id_1": "1234",
          "id_2": "5678"
        });

        let expected = vec!["1234".to_string(), "5678".to_string()];

        let vec = json_to_vec::<String>(json);
        assert_eq!(vec, expected);
    }

    #[test]
    fn test_json_with_other_types() {
        let json = json!({
          "num": 123,
          "bool": true
        });

        let vec = json_to_vec::<String>(json);
        assert_eq!(vec, Vec::<String>::new());
    }
}
