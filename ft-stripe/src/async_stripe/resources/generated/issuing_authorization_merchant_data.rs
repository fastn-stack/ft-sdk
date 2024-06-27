// This code is taken from [async-stripe](https://github.com/arlyon/async-stripe/tree/0a00d31894191ee0c6b4bda31e0d52d59e8e93b7)
// Author: Alexander Lyon
// License under either of:
//      - Apache License, Version 2.0, (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
//      - MIT License (LICENSE-MIT or https://opensource.org/licenses/MIT)

// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "IssuingAuthorizationMerchantData".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct MerchantData {

    /// A categorization of the seller's type of business.
    ///
    /// See our [merchant categories guide](https://stripe.com/docs/issuing/merchant-categories) for a list of possible values.
    pub category: String,

    /// The merchant category code for the seller’s business.
    pub category_code: String,

    /// City where the seller is located.
    pub city: Option<String>,

    /// Country where the seller is located.
    pub country: Option<String>,

    /// Name of the seller.
    pub name: Option<String>,

    /// Identifier assigned to the seller by the card network.
    ///
    /// Different card networks may assign different network_id fields to the same merchant.
    pub network_id: String,

    /// Postal code where the seller is located.
    pub postal_code: Option<String>,

    /// State where the seller is located.
    pub state: Option<String>,

    /// An ID assigned by the seller to the location of the sale.
    pub terminal_id: Option<String>,

    /// URL provided by the merchant on a 3DS request.
    pub url: Option<String>,
}
