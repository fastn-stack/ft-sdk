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

/// The resource representing a Stripe "Shipping".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Shipping {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    /// The delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,

    /// Recipient name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Recipient phone (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// The tracking number for a physical product, obtained from the delivery service.
    ///
    /// If multiple tracking numbers were generated for this purchase, please separate them with commas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<String>,
}
