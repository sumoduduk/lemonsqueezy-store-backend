mod checkout_struct;

use lemonsqueezy::{checkout::Checkout, LemonSqueezy};

pub async fn create_checkout(lemon: LemonSqueezy) {
    //code
    let build_checkout = Checkout::build(lemon);
}
