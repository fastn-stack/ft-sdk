// This code is taken from [async-stripe](https://github.com/arlyon/async-stripe/tree/0a00d31894191ee0c6b4bda31e0d52d59e8e93b7)
// Author: Alexander Lyon
// License under either of:
//      - Apache License, Version 2.0, (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
//      - MIT License (LICENSE-MIT or https://opensource.org/licenses/MIT)

//! Structs of unknown origin that aren't picked
//! up by the codegen.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CountrySpecSupportedBankAccountCurrencies {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ExchangeRateRates {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct IssuingAuthorizationAmountDetails {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct NotificationEventData {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsCardInstallmentsPlan {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsCardPresent {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentMethodDetailsCardWallet {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RadarValueListItem {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SubscriptionTransferData {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ThreeDSecureDetails {}
