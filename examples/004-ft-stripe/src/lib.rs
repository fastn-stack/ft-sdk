#[ft_sdk::processor]
fn stripe_create_customer() -> ft_sdk::processor::Result {
    ft_sdk::println!("params: {path}");
    let customer = stripe_create_customer_();
    ft_sdk::processor::json(customer)
}

fn stripe_create_customer_() -> ft_stripe::Customer {
    let client = ft_stripe::Client::new("c2tfdGVzdF90UjNQWWJjVk5aWjc5NnRIODhTNFZRMnU6");
    let create_customer = {
        let mut create_customer = ft_stripe::CreateCustomer::new();
        create_customer.name = Some("Jenny Rosen");
        create_customer.email = Some("jennyrosen@example.com");
        create_customer
    };

    client.create_customer(create_customer).unwrap()
}