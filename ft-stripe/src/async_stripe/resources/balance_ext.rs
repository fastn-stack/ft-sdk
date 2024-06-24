// This code is taken from [async-stripe](https://github.com/arlyon/async-stripe/tree/0a00d31894191ee0c6b4bda31e0d52d59e8e93b7)
// Author: Alexander Lyon
// License under either of:
//      - Apache License, Version 2.0, (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
//      - MIT License (LICENSE-MIT or https://opensource.org/licenses/MIT)

use crate::client::{Client, Response};
use crate::ids::AccountId;
use crate::resources::Balance;

impl Balance {
    /// Retrieves balance object by AccountId. Does not change stripe_account of the client.
    ///
    /// For more details see <https://stripe.com/docs/api/balance/balance_retrieve>.
    pub fn retrieve(client: &Client, account_id: Option<AccountId>) -> Response<Balance> {
        match account_id {
            Some(account_id) => client.clone().with_stripe_account(account_id).get("/balance"),
            None => client.get("/balance"),
        }
    }
}
