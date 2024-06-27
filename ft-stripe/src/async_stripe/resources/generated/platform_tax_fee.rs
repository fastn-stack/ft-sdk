// This code is taken from [async-stripe](https://github.com/arlyon/async-stripe/tree/0a00d31894191ee0c6b4bda31e0d52d59e8e93b7)
// Author: Alexander Lyon
// License under either of:
//      - Apache License, Version 2.0, (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
//      - MIT License (LICENSE-MIT or https://opensource.org/licenses/MIT)

// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

use crate::ids::PlatformTaxFeeId;
use crate::params::Object;

/// The resource representing a Stripe "PlatformTax".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PlatformTaxFee {
    /// Unique identifier for the object.
    pub id: PlatformTaxFeeId,

    /// The Connected account that incurred this charge.
    pub account: String,

    /// The payment object that caused this tax to be inflicted.
    pub source_transaction: String,

    /// The type of tax (VAT).
    #[serde(rename = "type")]
    pub type_: String,
}

impl Object for PlatformTaxFee {
    type Id = PlatformTaxFeeId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "platform_tax_fee"
    }
}
