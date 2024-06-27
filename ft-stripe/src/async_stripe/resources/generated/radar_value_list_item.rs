// This code is taken from [async-stripe](https://github.com/arlyon/async-stripe/tree/0a00d31894191ee0c6b4bda31e0d52d59e8e93b7)
// Author: Alexander Lyon
// License under either of:
//      - Apache License, Version 2.0, (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
//      - MIT License (LICENSE-MIT or https://opensource.org/licenses/MIT)

// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{RadarValueListItemId};
use crate::params::{Object, Timestamp};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "RadarListListItem".
///
/// For more details see <https://stripe.com/docs/api/radar/value_list_items/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RadarValueListItem {
    /// Unique identifier for the object.
    pub id: RadarValueListItemId,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<Timestamp>,

    /// The name or email address of the user who added this item to the value list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub livemode: Option<bool>,

    /// The value of the item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,

    /// The identifier of the value list this item belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_list: Option<String>,
}

impl Object for RadarValueListItem {
    type Id = RadarValueListItemId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "radar.value_list_item"
    }
}
