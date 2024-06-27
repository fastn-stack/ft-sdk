// This code is taken from [async-stripe](https://github.com/arlyon/async-stripe/tree/0a00d31894191ee0c6b4bda31e0d52d59e8e93b7)
// Author: Alexander Lyon
// License under either of:
//      - Apache License, Version 2.0, (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
//      - MIT License (LICENSE-MIT or https://opensource.org/licenses/MIT)

// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{ReportingReportTypeId};
use crate::params::{Object, Timestamp};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "reporting_report_type".
///
/// For more details see <https://stripe.com/docs/api/reporting/report_type/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ReportingReportType {
    /// The [ID of the Report Type](https://stripe.com/docs/reporting/statements/api#available-report-types), such as `balance.summary.1`.
    pub id: ReportingReportTypeId,

    /// Most recent time for which this Report Type is available.
    ///
    /// Measured in seconds since the Unix epoch.
    pub data_available_end: Timestamp,

    /// Earliest time for which this Report Type is available.
    ///
    /// Measured in seconds since the Unix epoch.
    pub data_available_start: Timestamp,

    /// List of column names that are included by default when this Report Type gets run.
    ///
    /// (If the Report Type doesn't support the `columns` parameter, this will be null.).
    pub default_columns: Option<Vec<String>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Human-readable name of the Report Type.
    pub name: String,

    /// When this Report Type was latest updated.
    ///
    /// Measured in seconds since the Unix epoch.
    pub updated: Timestamp,

    /// Version of the Report Type.
    ///
    /// Different versions report with the same ID will have the same purpose, but may take different run parameters or have different result schemas.
    pub version: i64,
}

impl Object for ReportingReportType {
    type Id = ReportingReportTypeId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "reporting.report_type"
    }
}
