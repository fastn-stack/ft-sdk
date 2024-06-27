// This code is taken from [async-stripe](https://github.com/arlyon/async-stripe/tree/0a00d31894191ee0c6b4bda31e0d52d59e8e93b7)
// Author: Alexander Lyon
// License under either of:
//      - Apache License, Version 2.0, (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
//      - MIT License (LICENSE-MIT or https://opensource.org/licenses/MIT)

// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

use crate::resources::Address;

/// The resource representing a Stripe "billing_details".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct BillingDetails {
    /// Billing address.
    pub address: Option<Address>,

    /// Email address.
    pub email: Option<String>,

    /// Full name.
    pub name: Option<String>,

    /// Billing phone number (including extension).
    pub phone: Option<String>,
}
