// This code is taken from [async-stripe](https://github.com/arlyon/async-stripe/tree/0a00d31894191ee0c6b4bda31e0d52d59e8e93b7)
// Author: Alexander Lyon
// License under either of:
//      - Apache License, Version 2.0, (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
//      - MIT License (LICENSE-MIT or https://opensource.org/licenses/MIT)

use serde::{Deserialize, Serialize};

use crate::client::{Client, Response};
use crate::ids::{CustomerId, InvoiceId};
use crate::resources::{Currency, InvoiceLineItem};

impl InvoiceLineItem {
    /// Creates an invoice line item.
    ///
    /// For more details see <https://stripe.com/docs/api#invoice_line_item_object>.
    pub fn create(client: &Client, params: CreateInvoiceLineItem<'_>) -> Response<InvoiceLineItem> {
        client.post_form("/invoiceitems", &params)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateInvoiceLineItem<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discountable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<InvoiceId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<bool>,
}

impl CreateInvoiceLineItem<'_> {
    pub fn new() -> Self {
        CreateInvoiceLineItem {
            amount: None,
            currency: None,
            customer: None,
            description: None,
            discountable: None,
            invoice: None,
            subscription: None,
        }
    }
}
