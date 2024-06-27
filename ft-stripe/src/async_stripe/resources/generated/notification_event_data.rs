// This code is taken from [async-stripe](https://github.com/arlyon/async-stripe/tree/0a00d31894191ee0c6b4bda31e0d52d59e8e93b7)
// Author: Alexander Lyon
// License under either of:
//      - Apache License, Version 2.0, (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
//      - MIT License (LICENSE-MIT or https://opensource.org/licenses/MIT)

// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "NotificationEventData".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct NotificationEventData {

    /// Object containing the names of the updated attributes and their values prior to the event (only included in events of type `*.updated`).
    ///
    /// If an array attribute has any updated elements, this object contains the entire array.
    /// In Stripe API versions 2017-04-06 or earlier, an updated array attribute in this object includes only the updated array elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_attributes: Option<NotificationEventDataPreviousAttributes>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct NotificationEventDataPreviousAttributes {
}
