#[ft_sdk::processor]
fn stripe_create_customer() -> ft_sdk::processor::Result {
    let customer = stripe_create_customer_();
    ft_sdk::println!("customer: {customer:?}");
    ft_sdk::processor::json(customer)
}

fn stripe_create_customer_() -> ft_stripe::Customer {
    let secret_key = ft_sdk::env::var("STRIPE_SECRET_KEY".to_string()).expect("Missing STRIPE_SECRET_KEY in env");
    ft_sdk::println!("secret_key: {secret_key}");
    let client = ft_stripe::Client::new(secret_key);
    let create_customer = {
        let mut create_customer = ft_stripe::CreateCustomer::new();
        create_customer.name = Some("Jenny Rosen");
        create_customer.email = Some("jennyrosen@example.com");
        create_customer
    };
    let response = ft_stripe::Customer::create(&client, create_customer);
    ft_sdk::println!("response:: {response:?}");
    response.unwrap()
}