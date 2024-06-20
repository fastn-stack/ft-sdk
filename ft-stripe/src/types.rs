// Code taken from https://github.com/wyyerd/stripe-rs/tree/c2f03f8dec41e20b66f9bbe902b8384096ac653c
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Address {
    /// Address line 1 or block/building number (e.g. Street address/PO Box/Company name)
    pub line1: Option<String>,
    /// Address line 2 or building details (e.g. Apartment/Suite/Unit/Building)
    pub line2: Option<String>,
    /// City (or Ward)
    pub city: Option<String>,
    /// State (or Prefecture)
    pub state: Option<String>,
    /// ZIP or postal code
    pub postal_code: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2))
    pub country: Option<String>,
    /// The town/cho-me (Japan only)
    pub town: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct CouponId(pub String);


#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct CustomField {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct ShippingParams {
    pub address: Address,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}
