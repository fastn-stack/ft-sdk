// This code is taken from [async-stripe](https://github.com/arlyon/async-stripe/tree/0a00d31894191ee0c6b4bda31e0d52d59e8e93b7)
// Author: Alexander Lyon
// License under either of:
//      - Apache License, Version 2.0, (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
//      - MIT License (LICENSE-MIT or https://opensource.org/licenses/MIT)

use serde::{Deserialize, Serialize};

use crate::client::{Client, Response};
use crate::ids::{PayoutDestinationId, PayoutId};
use crate::params::Object;
use crate::resources::{BankAccount, Card, Payout};

impl Payout {
    /// Cancels the payout.
    ///
    /// For more details see <https://stripe.com/docs/api/payouts/cancel>.
    pub fn cancel(client: &Client, id: &PayoutId) -> Response<Payout> {
        client.post(&format!("/payouts/{}/cancel", id))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum PayoutDestinationUnion {
    BankAccount(BankAccount),
    Card(Card),
}
impl std::default::Default for PayoutDestinationUnion {
    fn default() -> Self {
        Self::BankAccount(Default::default())
    }
}
impl Object for PayoutDestinationUnion {
    type Id = PayoutDestinationId;
    fn id(&self) -> Self::Id {
        match self {
            PayoutDestinationUnion::BankAccount(x) => PayoutDestinationId::BankAccount(x.id()),
            PayoutDestinationUnion::Card(x) => PayoutDestinationId::Card(x.id()),
        }
    }
    fn object(&self) -> &'static str {
        match self {
            PayoutDestinationUnion::BankAccount(x) => x.object(),
            PayoutDestinationUnion::Card(x) => x.object(),
        }
    }
}
