use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutRequest {
    #[serde(rename = "type")]
    checkout_type: String,
    attributes: CheckoutAttributes,
    relationships: VariantRelationship,
}

pub struct CheckoutAttributesRequest {
    custom_price: i32,
    product_options: ProductOptions,
    checkout_options: CheckoutOptions,
    checkout_data: CheckoutData,
    preview: Preview,
    expires_at: Option<String>,
    created_at: String,
    updated_at: String,
    test_mode: bool,
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct VariantRelationship {
    store: RelationshipData,
    variant: RelationshipData,
}

#[derive(Debug, Serialize, Deserialize)]
struct RelationshipData {
    data: RelationshipType,
}

#[derive(Debug, Serialize, Deserialize)]
struct RelationshipType {
    r#type: String,
    id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutResponse {
    #[serde(rename = "type")]
    checkout_type: String,
    id: String,
    attributes: CheckoutAttributes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutAttributes {
    store_id: i32,
    variant_id: i32,
    custom_price: Option<i32>,
    product_options: ProductOptions,
    checkout_options: CheckoutOptions,
    checkout_data: CheckoutData,
    preview: Preview,
    expires_at: Option<String>,
    created_at: String,
    updated_at: String,
    test_mode: bool,
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductOptions {
    name: String,
    description: String,
    media: Vec<String>,
    redirect_url: String,
    receipt_button_text: String,
    receipt_link_url: String,
    receipt_thank_you_note: String,
    enabled_variants: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutOptions {
    embed: bool,
    media: bool,
    logo: bool,
    desc: bool,
    discount: bool,
    dark: bool,
    subscription_preview: bool,
    button_color: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutData {
    email: String,
    name: String,
    billing_address: BillingAddress,
    tax_number: String,
    discount_code: String,
    custom: Vec<String>,
    variant_quantities: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BillingAddress {
    country: String,
    zip: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Preview {
    currency: String,
    currency_rate: f64,
    subtotal: i32,
    discount_total: i32,
    tax: i32,
    total: i32,
    subtotal_usd: i32,
    discount_total_usd: i32,
    tax_usd: i32,
    total_usd: i32,
    subtotal_formatted: String,
    discount_total_formatted: String,
    tax_formatted: String,
    total_formatted: String,
}
