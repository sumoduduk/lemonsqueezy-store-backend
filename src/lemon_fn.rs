mod checkout_struct;

use lemonsqueezy::types::{checkout::*, Data};
use lemonsqueezy::{checkout::Checkout, LemonSqueezy};

use crate::get_tomorrow_iso8601;

pub async fn create_checkout(lemon: LemonSqueezy) {
    //code
    let build_checkout = Checkout::build(lemon);

    let time_expires = get_tomorrow_iso8601();

    let options_product = CreateCheckoutProductOptions {
        name: Some("Bridebook Photo".to_string()),
        description: Some("Diversity Bride Photo by Bridebook.com".to_string()),
        media: todo!(),
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
        custom: None,
        variant_quantities: None,
    };

    let store_data: CreateCheckoutRelationShipData = CreateCheckoutRelationShipData {
        r#type: "stores".to_string(),
        id: "",
    };

    let relationships_chechkout = CreateCheckoutRelationships {
        store: todo!(),
        variant: todo!(),
    };

    let checkout = build_checkout.create(CreateCheckout {
        r#type: "checkout".to_string(),
        attributes: CreateCheckoutAttributes {
            custom_price: Some(400),
            product_options: Some(options_product),
            checkout_data: Some(data_checkout),
            expires_at: Some(time_expires),
            test_mode: Some(true),
            checkout_options: None,
        },
        relationships: Some(),
    });
}
