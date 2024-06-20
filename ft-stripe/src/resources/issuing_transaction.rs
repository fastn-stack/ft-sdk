// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::IssuingTransactionId;
use crate::params::{Expandable, Metadata, Object, Timestamp};
use crate::resources::{
    BalanceTransaction, Currency, IssuingAuthorization, IssuingCard, IssuingCardholder,
    IssuingTransactionType, MerchantData,
};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "IssuingTransaction".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingTransaction {
    /// Unique identifier for the object.
    pub id: IssuingTransactionId,

    /// The transaction amount, which will be reflected in your balance.
    ///
    /// This amount is in your currency and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount: i64,

    /// The `Authorization` object that led to this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<Expandable<IssuingAuthorization>>,

    /// ID of the [balance transaction](https://stripe.com/docs/api/balance_transactions) associated with this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_transaction: Option<Expandable<BalanceTransaction>>,

    /// The card used to make this transaction.
    pub card: Expandable<IssuingCard>,

    /// The cardholder to whom this transaction belongs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder: Option<Expandable<IssuingCardholder>>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The amount that the merchant will receive, denominated in `merchant_currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    ///
    /// It will be different from `amount` if the merchant is taking payment in a different currency.
    pub merchant_amount: i64,

    /// The currency with which the merchant is taking payment.
    pub merchant_currency: Currency,

    pub merchant_data: MerchantData,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// Additional purchase information that is optionally provided by the merchant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_details: Option<IssuingTransactionPurchaseDetails>,

    /// The nature of the transaction.
    #[serde(rename = "type")]
    pub type_: IssuingTransactionType,
}

impl Object for IssuingTransaction {
    type Id = IssuingTransactionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "issuing.transaction"
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingTransactionPurchaseDetails {
    /// Information about the flight that was purchased with this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flight: Option<IssuingTransactionFlightData>,

    /// Information about fuel that was purchased with this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fuel: Option<IssuingTransactionFuelData>,

    /// Information about lodging that was purchased with this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lodging: Option<IssuingTransactionLodgingData>,

    /// The line items in the purchase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt: Option<Vec<IssuingTransactionReceiptData>>,

    /// A merchant-specific order number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingTransactionFlightData {
    /// The time that the flight departed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_at: Option<i64>,

    /// The name of the passenger.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passenger_name: Option<String>,

    /// Whether the ticket is refundable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refundable: Option<bool>,

    /// The legs of the trip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<IssuingTransactionFlightDataLeg>>,

    /// The travel agency that issued the ticket.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub travel_agency: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingTransactionFlightDataLeg {
    /// The flight's destination airport code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrival_airport_code: Option<String>,

    /// The airline carrier code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,

    /// The airport code that the flight departed from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_airport_code: Option<String>,

    /// The flight number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flight_number: Option<String>,

    /// The flight's service class.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_class: Option<String>,

    /// Whether a stopover is allowed on this flight.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopover_allowed: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingTransactionFuelData {
    /// The type of fuel that was purchased.
    ///
    /// One of `diesel`, `unleaded_plus`, `unleaded_regular`, `unleaded_super`, or `other`.
    #[serde(rename = "type")]
    pub type_: String,

    /// The units for `volume`.
    ///
    /// One of `us_gallon` or `liter`.
    pub unit: String,

    /// The cost in cents per each unit of fuel, represented as a decimal string with at most 12 decimal places.
    pub unit_cost_decimal: String,

    /// The volume of the fuel that was pumped, represented as a decimal string with at most 12 decimal places.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_decimal: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingTransactionLodgingData {
    /// The time of checking into the lodging.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_in_at: Option<i64>,

    /// The number of nights stayed at the lodging.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nights: Option<i64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingTransactionReceiptData {
    /// The description of the item.
    ///
    /// The maximum length of this field is 26 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The quantity of the item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f64>,

    /// The total for this line item in cents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,

    /// The unit cost of the item in cents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_cost: Option<i64>,
}
