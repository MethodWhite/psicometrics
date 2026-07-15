use async_stripe::{
    Client, CheckoutSession, CreateCheckoutSession, BillingPortalSession, 
    CreateBillingPortalSession, Event,
};

#[tokio::main]
async fn main() {
    let client = Client::new("sk_test_placeholder");
    println!("client created: {:?}", client.client_id());
}
