use crate::config::{Client, Response};
use crate::ids::{CouponId, CustomerId, InvoiceId, PlanId, SubscriptionId, SubscriptionItemId};
use crate::params::{Metadata, Timestamp};
use crate::resources::{CollectionMethod, Invoice};
use serde::Serialize;

#[deprecated(since = "0.12.0")]
pub type InvoiceCollectionMethod = CollectionMethod;

impl Invoice {
    /// Retrieves the details of an upcoming invoice_id
    ///
    /// For more details see https://stripe.com/docs/api#upcoming_invoice
    pub fn upcoming(client: &Client, params: RetrieveUpcomingInvoice) -> Response<Invoice> {
        client.get_query("/invoices/upcoming", &params)
    }

    /// Pays an invoice.
    ///
    /// For more details see https://stripe.com/docs/api#pay_invoice.
    pub fn pay(client: &Client, invoice_id: &InvoiceId) -> Response<Invoice> {
        client.post(&format!("/invoices/{}/pay", invoice_id))
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct RetrieveUpcomingInvoice {
    pub customer: CustomerId, // this is a required param
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<CouponId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<SubscriptionId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_items: Option<SubscriptionItemFilter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_prorate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_proration_date: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_tax_percent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_trial_end: Option<Timestamp>,
}

impl RetrieveUpcomingInvoice {
    pub fn new(customer: CustomerId) -> Self {
        RetrieveUpcomingInvoice {
            customer,
            coupon: None,
            subscription: None,
            subscription_items: None,
            subscription_prorate: None,
            subscription_proration_date: None,
            subscription_tax_percent: None,
            subscription_trial_end: None,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct SubscriptionItemFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<SubscriptionItemId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<PlanId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
}
