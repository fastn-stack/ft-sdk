// Code is inspired from https://github.com/wyyerd/stripe-rs/tree/c2f03f8dec41e20b66f9bbe902b8384096ac653c

use ft_stripe::{Client, CreateCustomer, Customer, CustomerId, List, ListCustomers, Response, UpdateCustomer};
use ft_stripe::params::{Deleted, Expand};
impl Client {
    /// Returns a list of your customers.
    ///
    /// The customers are returned sorted by creation date, with the most recent customers appearing first.
    pub fn list_customers(&self, params: &ListCustomers<'_>) -> Response<List<Customer>> {
        self.get_query("/customers", &params)
    }

    /// Creates a new customer object.
    pub fn create_customer(&self, params: CreateCustomer<'_>) -> Response<Customer> {
        self.post_form("/customers", &params)
    }

    /// Retrieves a Customer object.
    pub fn retrieve_customer(&self, id: &CustomerId, expand: &[&str]) -> Response<Customer> {
        self.get_query(&format!("/customers/{}", id), &Expand { expand })
    }

    /// Updates the specified customer by setting the values of the parameters passed.
    ///
    /// Any parameters not provided will be left unchanged.
    /// For example, if you pass the **source** parameter, that becomes the customer’s active source (e.g., a card) to be used for all charges in the future.
    /// When you update a customer to a new valid card source by passing the **source** parameter: for each of the customer’s current subscriptions, if the subscription bills automatically and is in the `past_due` state, then the latest open invoice for the subscription with automatic collection enabled will be retried.
    /// This retry will not count as an automatic retry, and will not affect the next regularly scheduled payment for the invoice.
    /// Changing the **default_source** for a customer will not trigger this behavior.  This request accepts mostly the same arguments as the customer creation call.
    pub fn update_customer(
        &self,
        id: &CustomerId,
        params: UpdateCustomer<'_>,
    ) -> Response<Customer> {
        self.post_form(&format!("/customers/{}", id), &params)
    }

    /// Permanently deletes a customer.
    ///
    /// It cannot be undone.
    /// Also immediately cancels any active subscriptions on the customer.
    pub fn delete_customer(&self, id: &CustomerId) -> Response<Deleted<CustomerId>> {
        self.delete(&format!("/customers/{}", id))
    }
}