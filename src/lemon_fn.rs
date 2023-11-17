use std::env;

use lemonsqueezy::types::{checkout::*, Data};
use lemonsqueezy::utils::Response;
use lemonsqueezy::{checkout::Checkout, LemonSqueezy};

use crate::db_model::{Operation, OperationResult};
use crate::utils::{extract_image, make_custom_data};
use crate::{one_hour_from_now, PoolPg};

use eyre::Report;

pub async fn create_checkout(
    ids: &[String],
    lemon: LemonSqueezy,
    pool: &PoolPg,
    email: String,
    user_id: String,
    name_product: String,
    description: String,
) -> eyre::Result<Response<CheckoutResponse>> {
    //code

    let build_checkout = Checkout::build(lemon);

    let time_expires = one_hour_from_now();

    let len = ids.len() as i64;

    let res = Operation::GetDataById(ids.to_vec()).execute(pool).await?;

    match res {
        OperationResult::Fetched(arr_data) => {
            let redirect_uri = env::var("REDIRECT_URI").expect("REDIRECT_URI are not present");
            let arr_img = extract_image(&arr_data);

            let custom_data = make_custom_data(ids, &user_id)?;

            let total_prices = 400 * len;

            let options_product = CreateCheckoutProductOptions {
                //need fix dynamic name
                name: Some(name_product),
                //need fix dynamic description
                description: Some(description),
                media: Some(arr_img),
                redirect_url: Some(redirect_uri),
                receipt_button_text: None,
                receipt_link_url: None,
                receipt_thank_you_note: None,
                enabled_variants: None,
            };

            let data_checkout = CreateCheckoutCheckoutData {
                email: Some(email),
                name: None,
                billing_address: None,
                tax_number: None,
                discount_code: None,
                custom: Some(custom_data),
                variant_quantities: None,
            };

            //fix: id variant and store_id neer to be in enviroment variable

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
        OperationResult::Inserted => Err(Report::msg("Something went wrong")),
    }
}

