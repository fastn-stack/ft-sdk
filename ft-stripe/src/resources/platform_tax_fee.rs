// Code taken from https://github.com/wyyerd/stripe-rs/tree/c2f03f8dec41e20b66f9bbe902b8384096ac653c
// ======================================
// This file was automatically generated.
// ======================================

use crate::params::Object;
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "PlatformTax".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlatformTaxFee {
    /// The Connected account that incurred this charge.
    pub account: String,

    /// The payment object that caused this tax to be inflicted.
    pub source_transaction: String,

    /// The type of tax (VAT).
    #[serde(rename = "type")]
    pub type_: String,
}

impl Object for PlatformTaxFee {
    type Id = ();
    fn id(&self) -> Self::Id {}
    fn object(&self) -> &'static str {
        "platform_tax_fee"
    }
}
