// Code taken from https://github.com/wyyerd/stripe-rs/tree/c2f03f8dec41e20b66f9bbe902b8384096ac653c
use crate::config::{Client, Response};
use crate::ids::{PayoutDestinationId, PayoutId};
use crate::params::Object;
use crate::resources::{Payout, PayoutDestination};

impl Payout {
    /// Cancels the payout.
    ///
    /// For more details see [https://stripe.com/docs/api/payouts/cancel](https://stripe.com/docs/api/payouts/cancel).
    pub fn cancel(client: &Client, id: &PayoutId) -> Response<Payout> {
        client.post(&format!("/payouts/{}/cancel", id))
    }
}

impl Object for PayoutDestination {
    type Id = PayoutDestinationId;
    fn id(&self) -> Self::Id {
        match self {
            PayoutDestination::BankAccount(x) => PayoutDestinationId::BankAccount(x.id()),
            PayoutDestination::Card(x) => PayoutDestinationId::Card(x.id()),
        }
    }
    fn object(&self) -> &'static str {
        match self {
            PayoutDestination::BankAccount(x) => x.object(),
            PayoutDestination::Card(x) => x.object(),
        }
    }
}
