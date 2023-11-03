use lemonsqueezy::types::{checkout::*, Data};
use lemonsqueezy::utils::Response;
use lemonsqueezy::{checkout::Checkout, LemonSqueezy};
use serde_json::json;

use crate::db_model::Operation;
use crate::utils::{extract_image, vec_to_json};
use crate::{one_hour_from_now, PoolPg};

pub async fn create_checkout(
    ids: &[String],
    lemon: LemonSqueezy,
    pool: &PoolPg,
) -> eyre::Result<Response<CheckoutResponse>> {
    //code

    let build_checkout = Checkout::build(lemon);

    let time_expires = one_hour_from_now();

    let len = ids.len() as i64;

    let res = Operation::GetDataById(ids.to_vec()).execute(pool).await?;

    let arr_img = extract_image(&res);

    let total_prices = 400 * len;

    let options_product = CreateCheckoutProductOptions {
        name: Some("Test Costum Webhook Photo".to_string()),
        description: Some("Diversity Bride Photo by Bridebook.com".to_string()),
        media: Some(arr_img),
        redirect_url: Some("https://lemonsqueezy.com".to_string()),
        receipt_button_text: None,
        receipt_link_url: None,
        receipt_thank_you_note: None,
        enabled_variants: None,
    };

    let data_checkout = CreateCheckoutCheckoutData {
        email: Some("shoemakeraiko@gmail.com".to_string()),
        name: None,
        billing_address: None,
        tax_number: None,
        discount_code: None,
        custom: Some(json!({

        "user_id": "123"
              })),
        variant_quantities: None,
    };

    let store_data: CreateCheckoutRelationShipData = CreateCheckoutRelationShipData {
        r#type: "stores".to_string(),
        id: "50443".to_string(),
    };

    let variant_data: CreateCheckoutRelationShipData = CreateCheckoutRelationShipData {
        r#type: "variants".to_string(),
        id: "146143".to_string(),
    };

    let relationships_chechkout = CreateCheckoutRelationships {
        store: Data { data: store_data },
        variant: Data { data: variant_data },
    };

    let checkout_master = CreateCheckout {
        r#type: "checkouts".to_string(),
        attributes: CreateCheckoutAttributes {
            custom_price: Some(total_prices),
            product_options: Some(options_product),
            checkout_data: Some(data_checkout),
            expires_at: Some(time_expires),
            test_mode: Some(true),
            checkout_options: None,
        },
        relationships: Some(relationships_chechkout),
    };

    let pretty_json = serde_json::to_string_pretty(&checkout_master)?;

    println!("data : {}", pretty_json);

    let checkout = build_checkout.create(checkout_master).await;

    match checkout {
        Ok(data) => Ok(data),
        Err(error) => {
            dbg!(&error);
            Err(error.into())
        }
    }
}
