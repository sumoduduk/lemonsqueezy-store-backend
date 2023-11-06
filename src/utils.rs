pub mod json_to_vec;
pub mod time_manipulation;
pub mod vec_to_json;

use crate::db_model::DataDB;

pub fn extract_image(data: &[DataDB]) -> Vec<String> {
    let data_len = data.len();
    let mut arr_img = Vec::with_capacity(data_len);

    for image in data {
        arr_img.push(image.thumb_image.clone())
    }

    arr_img
}
