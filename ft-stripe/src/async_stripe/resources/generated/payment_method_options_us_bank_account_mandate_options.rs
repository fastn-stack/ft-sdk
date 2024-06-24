// This code is taken from [async-stripe](https://github.com/arlyon/async-stripe/tree/0a00d31894191ee0c6b4bda31e0d52d59e8e93b7)
// Author: Alexander Lyon
// License under either of:
//      - Apache License, Version 2.0, (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
//      - MIT License (LICENSE-MIT or https://opensource.org/licenses/MIT)

// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "payment_method_options_us_bank_account_mandate_options".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodOptionsUsBankAccountMandateOptions {
    /// Mandate collection method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<PaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod>,
}

/// An enum representing the possible values of an `PaymentMethodOptionsUsBankAccountMandateOptions`'s `collection_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod {
    Paper,
}

impl PaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod::Paper => "paper",
        }
    }
}

impl AsRef<str> for PaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentMethodOptionsUsBankAccountMandateOptionsCollectionMethod {
    fn default() -> Self {
        Self::Paper
    }
}
