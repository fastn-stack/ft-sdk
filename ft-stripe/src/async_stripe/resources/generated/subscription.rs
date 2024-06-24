// This code is taken from [async-stripe](https://github.com/arlyon/async-stripe/tree/0a00d31894191ee0c6b4bda31e0d52d59e8e93b7)
// Author: Alexander Lyon
// License under either of:
//      - Apache License, Version 2.0, (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
//      - MIT License (LICENSE-MIT or https://opensource.org/licenses/MIT)

// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

use crate::client::{Client, Response};
use crate::ids::{CouponId, CustomerId, PlanId, PriceId, PromotionCodeId, SubscriptionId};
use crate::params::{
    Deleted, Expand, Expandable, List, Metadata, Object, Paginable, RangeQuery, Timestamp,
};
use crate::resources::{
    Account, Application, CollectionMethod, ConnectAccountReference, Currency, Customer, Discount,
    Invoice, InvoicePaymentMethodOptionsAcssDebit, InvoicePaymentMethodOptionsBancontact,
    InvoicePaymentMethodOptionsCustomerBalance, InvoicePaymentMethodOptionsKonbini,
    InvoicePaymentMethodOptionsUsBankAccount, PaymentMethod, PaymentSource, Scheduled, SetupIntent,
    SubscriptionBillingThresholds, SubscriptionItem, SubscriptionItemBillingThresholds,
    SubscriptionSchedule, SubscriptionTransferData, SubscriptionsTrialsResourceTrialSettings,
    TaxRate, TestHelpersTestClock,
};

/// The resource representing a Stripe "Subscription".
///
/// For more details see <https://stripe.com/docs/api/subscriptions/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Subscription {
    /// Unique identifier for the object.
    pub id: SubscriptionId,

    /// ID of the Connect Application that created the subscription.
    pub application: Option<Expandable<Application>>,

    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice total that will be transferred to the application owner's Stripe account.
    pub application_fee_percent: Option<f64>,

    pub automatic_tax: SubscriptionAutomaticTax,

    /// The reference point that aligns future [billing cycle](https://stripe.com/docs/subscriptions/billing-cycle) dates.
    ///
    /// It sets the day of week for `week` intervals, the day of month for `month` and `year` intervals, and the month of year for `year` intervals.
    /// The timestamp is in UTC format.
    pub billing_cycle_anchor: Timestamp,

    /// The fixed values used to calculate the `billing_cycle_anchor`.
    pub billing_cycle_anchor_config: Option<SubscriptionsResourceBillingCycleAnchorConfig>,

    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    pub billing_thresholds: Option<SubscriptionBillingThresholds>,

    /// A date in the future at which the subscription will automatically get canceled.
    pub cancel_at: Option<Timestamp>,

    /// If the subscription has been canceled with the `at_period_end` flag set to `true`, `cancel_at_period_end` on the subscription will be true.
    ///
    /// You can use this attribute to determine whether a subscription that has a status of active is scheduled to be canceled at the end of the current period.
    pub cancel_at_period_end: bool,

    /// If the subscription has been canceled, the date of that cancellation.
    ///
    /// If the subscription was canceled with `cancel_at_period_end`, `canceled_at` will reflect the time of the most recent update request, not the end of the subscription period when the subscription is automatically moved to a canceled state.
    pub canceled_at: Option<Timestamp>,

    /// Details about why this subscription was cancelled.
    pub cancellation_details: Option<CancellationDetails>,

    /// Either `charge_automatically`, or `send_invoice`.
    ///
    /// When charging automatically, Stripe will attempt to pay this subscription at the end of the cycle using the default source attached to the customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
    pub collection_method: Option<CollectionMethod>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// End of the current period that the subscription has been invoiced for.
    ///
    /// At the end of this period, a new invoice will be created.
    pub current_period_end: Timestamp,

    /// Start of the current period that the subscription has been invoiced for.
    pub current_period_start: Timestamp,

    /// ID of the customer who owns the subscription.
    pub customer: Expandable<Customer>,

    /// Number of days a customer has to pay invoices generated by this subscription.
    ///
    /// This value will be `null` for subscriptions where `collection_method=charge_automatically`.
    pub days_until_due: Option<u32>,

    /// ID of the default payment method for the subscription.
    ///
    /// It must belong to the customer associated with the subscription.
    /// This takes precedence over `default_source`.
    /// If neither are set, invoices will use the customer's [invoice_settings.default_payment_method](https://stripe.com/docs/api/customers/object#customer_object-invoice_settings-default_payment_method) or [default_source](https://stripe.com/docs/api/customers/object#customer_object-default_source).
    pub default_payment_method: Option<Expandable<PaymentMethod>>,

    /// ID of the default payment source for the subscription.
    ///
    /// It must belong to the customer associated with the subscription and be in a chargeable state.
    /// If `default_payment_method` is also set, `default_payment_method` will take precedence.
    /// If neither are set, invoices will use the customer's [invoice_settings.default_payment_method](https://stripe.com/docs/api/customers/object#customer_object-invoice_settings-default_payment_method) or [default_source](https://stripe.com/docs/api/customers/object#customer_object-default_source).
    pub default_source: Option<Expandable<PaymentSource>>,

    /// The tax rates that will apply to any subscription item that does not have `tax_rates` set.
    ///
    /// Invoices created will have their `default_tax_rates` populated from the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<TaxRate>>,

    /// The subscription's description, meant to be displayable to the customer.
    ///
    /// Use this field to optionally store an explanation of the subscription for rendering in Stripe surfaces and certain local payment methods UIs.
    pub description: Option<String>,

    /// Describes the current discount applied to this subscription, if there is one.
    ///
    /// When billing, a discount applied to a subscription overrides a discount applied on a customer-wide basis.
    pub discount: Option<Discount>,

    /// If the subscription has ended, the date the subscription ended.
    pub ended_at: Option<Timestamp>,

    /// List of subscription items, each with an attached price.
    pub items: List<SubscriptionItem>,

    /// The most recent invoice this subscription has generated.
    pub latest_invoice: Option<Expandable<Invoice>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// Specifies the approximate timestamp on which any pending invoice items will be billed according to the schedule provided at `pending_invoice_item_interval`.
    pub next_pending_invoice_item_invoice: Option<Timestamp>,

    /// The account (if any) the charge was made on behalf of for charges associated with this subscription.
    ///
    /// See the Connect documentation for details.
    pub on_behalf_of: Option<Expandable<Account>>,

    /// If specified, payment collection for this subscription will be paused.
    pub pause_collection: Option<SubscriptionsResourcePauseCollection>,

    /// Payment settings passed on to invoices created by the subscription.
    pub payment_settings: Option<SubscriptionsResourcePaymentSettings>,

    /// Specifies an interval for how often to bill for any pending invoice items.
    ///
    /// It is analogous to calling [Create an invoice](https://stripe.com/docs/api#create_invoice) for the given subscription at the specified interval.
    pub pending_invoice_item_interval: Option<SubscriptionPendingInvoiceItemInterval>,

    /// You can use this [SetupIntent](https://stripe.com/docs/api/setup_intents) to collect user authentication when creating a subscription without immediate payment or updating a subscription's payment method, allowing you to optimize for off-session payments.
    ///
    /// Learn more in the [SCA Migration Guide](https://stripe.com/docs/billing/migration/strong-customer-authentication#scenario-2).
    pub pending_setup_intent: Option<Expandable<SetupIntent>>,

    /// If specified, [pending updates](https://stripe.com/docs/billing/subscriptions/pending-updates) that will be applied to the subscription once the `latest_invoice` has been paid.
    pub pending_update: Option<SubscriptionsResourcePendingUpdate>,

    /// The schedule attached to the subscription.
    pub schedule: Option<Expandable<SubscriptionSchedule>>,

    /// Date when the subscription was first created.
    ///
    /// The date might differ from the `created` date due to backdating.
    pub start_date: Timestamp,

    /// Possible values are `incomplete`, `incomplete_expired`, `trialing`, `active`, `past_due`, `canceled`, or `unpaid`.
    ///
    /// For `collection_method=charge_automatically` a subscription moves into `incomplete` if the initial payment attempt fails.
    /// A subscription in this state can only have metadata and default_source updated.
    /// Once the first invoice is paid, the subscription moves into an `active` state.
    /// If the first invoice is not paid within 23 hours, the subscription transitions to `incomplete_expired`.
    /// This is a terminal state, the open invoice will be voided and no further invoices will be generated.
    /// A subscription that is currently in a trial period is `trialing` and moves to `active` when the trial period is over.
    /// If subscription `collection_method=charge_automatically`, it becomes `past_due` when payment is required but cannot be paid (due to failed payment or awaiting additional user actions).
    /// Once Stripe has exhausted all payment retry attempts, the subscription will become `canceled` or `unpaid` (depending on your subscriptions settings).
    /// If subscription `collection_method=send_invoice` it becomes `past_due` when its invoice is not paid by the due date, and `canceled` or `unpaid` if it is still not paid by an additional deadline after that.
    /// Note that when a subscription has a status of `unpaid`, no subsequent invoices will be attempted (invoices will be created, but then immediately automatically closed).
    /// After receiving updated payment information from a customer, you may choose to reopen and pay their closed invoices.
    pub status: SubscriptionStatus,

    /// ID of the test clock this subscription belongs to.
    pub test_clock: Option<Expandable<TestHelpersTestClock>>,

    /// The account (if any) the subscription's payments will be attributed to for tax reporting, and where funds from each payment will be transferred to for each of the subscription's invoices.
    pub transfer_data: Option<SubscriptionTransferData>,

    /// If the subscription has a trial, the end of that trial.
    pub trial_end: Option<Timestamp>,

    /// Settings related to subscription trials.
    pub trial_settings: Option<SubscriptionsTrialsResourceTrialSettings>,

    /// If the subscription has a trial, the beginning of that trial.
    pub trial_start: Option<Timestamp>,
}

impl Subscription {
    /// By default, returns a list of subscriptions that have not been canceled.
    ///
    /// In order to list canceled subscriptions, specify `status=canceled`.
    pub fn list(client: &Client, params: &ListSubscriptions<'_>) -> Response<List<Subscription>> {
        client.get_query("/subscriptions", params)
    }

    /// Creates a new subscription on an existing customer.
    ///
    /// Each customer can have up to 500 active or scheduled subscriptions.  When you create a subscription with `collection_method=charge_automatically`, the first invoice is finalized as part of the request. The `payment_behavior` parameter determines the exact behavior of the initial payment.  To start subscriptions where the first invoice always begins in a `draft` status, use [subscription schedules](https://stripe.com/docs/billing/subscriptions/subscription-schedules#managing) instead. Schedules provide the flexibility to model more complex billing configurations that change over time.
    pub fn create(client: &Client, params: CreateSubscription<'_>) -> Response<Subscription> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form("/subscriptions", &params)
    }

    /// Retrieves the subscription with the given ID.
    pub fn retrieve(
        client: &Client,
        id: &SubscriptionId,
        expand: &[&str],
    ) -> Response<Subscription> {
        client.get_query(&format!("/subscriptions/{}", id), Expand { expand })
    }

    /// Updates an existing subscription to match the specified parameters.
    /// When changing prices or quantities, we optionally prorate the price we charge next month to make up for any price changes.
    /// To preview how the proration is calculated, use the [upcoming invoice](https://stripe.com/docs/api/invoices/upcoming) endpoint.
    ///
    /// By default, we prorate subscription changes.
    ///
    /// For example, if a customer signs up on May 1 for a $100 price, they’ll be billed $100 immediately.
    /// If on May 15 they switch to a $200 price, then on June 1 they’ll be billed $250 ($200 for a renewal of her subscription, plus a $50 prorating adjustment for half of the previous month’s $100 difference).
    /// Similarly, a downgrade generates a credit that is applied to the next invoice.
    /// We also prorate when you make quantity changes.  Switching prices does not normally change the billing date or generate an immediate charge unless:  <ul> <li>The billing interval is changed (for example, from monthly to yearly).</li> <li>The subscription moves from free to paid, or paid to free.</li> <li>A trial starts or ends.</li> </ul>  In these cases, we apply a credit for the unused time on the previous price, immediately charge the customer using the new price, and reset the billing date.  If you want to charge for an upgrade immediately, pass `proration_behavior` as `always_invoice` to create prorations, automatically invoice the customer for those proration adjustments, and attempt to collect payment.
    /// If you pass `create_prorations`, the prorations are created but not automatically invoiced.
    /// If you want to bill the customer for the prorations before the subscription’s renewal date, you need to manually [invoice the customer](https://stripe.com/docs/api/invoices/create).  If you don’t want to prorate, set the `proration_behavior` option to `none`.
    /// With this option, the customer is billed $100 on May 1 and $200 on June 1.
    /// Similarly, if you set `proration_behavior` to `none` when switching between different billing intervals (for example, from monthly to yearly), we don’t generate any credits for the old subscription’s unused time.
    /// We still reset the billing date and bill immediately for the new subscription.  Updating the quantity on a subscription many times in an hour may result in [rate limiting](https://stripe.com/docs/rate-limits).
    /// If you need to bill for a frequently changing quantity, consider integrating [usage-based billing](https://stripe.com/docs/billing/subscriptions/usage-based) instead.
    pub fn update(
        client: &Client,
        id: &SubscriptionId,
        params: UpdateSubscription<'_>,
    ) -> Response<Subscription> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form(&format!("/subscriptions/{}", id), &params)
    }

    /// Cancels a customer’s subscription immediately.
    ///
    /// The customer will not be charged again for the subscription.  Note, however, that any pending invoice items that you’ve created will still be charged for at the end of the period, unless manually [deleted](https://stripe.com/docs/api#delete_invoiceitem).
    /// If you’ve set the subscription to cancel at the end of the period, any pending prorations will also be left in place and collected at the end of the period.
    /// But if the subscription is set to cancel immediately, pending prorations will be removed.  By default, upon subscription cancellation, Stripe will stop automatic collection of all finalized invoices for the customer.
    /// This is intended to prevent unexpected payment attempts after the customer has canceled a subscription.
    /// However, you can resume automatic collection of the invoices manually after subscription cancellation to have us proceed.
    /// Or, you could check for unpaid invoices before allowing the customer to cancel the subscription at all.
    pub fn delete(client: &Client, id: &SubscriptionId) -> Response<Deleted<SubscriptionId>> {
        client.delete(&format!("/subscriptions/{}", id))
    }
}

impl Object for Subscription {
    type Id = SubscriptionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "subscription"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CancellationDetails {
    /// Additional comments about why the user canceled the subscription, if the subscription was canceled explicitly by the user.
    pub comment: Option<String>,

    /// The customer submitted reason for why they canceled, if the subscription was canceled explicitly by the user.
    pub feedback: Option<CancellationDetailsFeedback>,

    /// Why this subscription was canceled.
    pub reason: Option<CancellationDetailsReason>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SubscriptionAutomaticTax {
    /// Whether Stripe automatically computes tax on this subscription.
    pub enabled: bool,

    /// The account that's liable for tax.
    ///
    /// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
    /// The tax transaction is returned in the report of the connected account.
    pub liability: Option<ConnectAccountReference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SubscriptionPendingInvoiceItemInterval {
    /// Specifies invoicing frequency.
    ///
    /// Either `day`, `week`, `month` or `year`.
    pub interval: PlanInterval,

    /// The number of intervals between invoices.
    ///
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    pub interval_count: u64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SubscriptionsResourceBillingCycleAnchorConfig {
    /// The day of the month of the billing_cycle_anchor.
    pub day_of_month: i64,

    /// The hour of the day of the billing_cycle_anchor.
    pub hour: Option<i64>,

    /// The minute of the hour of the billing_cycle_anchor.
    pub minute: Option<i64>,

    /// The month to start full cycle billing periods.
    pub month: Option<i64>,

    /// The second of the minute of the billing_cycle_anchor.
    pub second: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SubscriptionsResourcePauseCollection {
    /// The payment collection behavior for this subscription while paused.
    ///
    /// One of `keep_as_draft`, `mark_uncollectible`, or `void`.
    pub behavior: SubscriptionsResourcePauseCollectionBehavior,

    /// The time after which the subscription will resume collecting payments.
    pub resumes_at: Option<Timestamp>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SubscriptionsResourcePaymentSettings {
    /// Payment-method-specific configuration to provide to invoices created by the subscription.
    pub payment_method_options: Option<SubscriptionsResourcePaymentMethodOptions>,

    /// The list of payment method types to provide to every invoice created by the subscription.
    ///
    /// If not set, Stripe attempts to automatically determine the types to use by looking at the invoice’s default payment method, the subscription’s default payment method, the customer’s default payment method, and your [invoice template settings](https://dashboard.stripe.com/settings/billing/invoice).
    pub payment_method_types: Option<Vec<SubscriptionsResourcePaymentSettingsPaymentMethodTypes>>,

    /// Either `off`, or `on_subscription`.
    ///
    /// With `on_subscription` Stripe updates `subscription.default_payment_method` when a subscription payment succeeds.
    pub save_default_payment_method:
        Option<SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SubscriptionsResourcePaymentMethodOptions {
    /// This sub-hash contains details about the Canadian pre-authorized debit payment method options to pass to invoices created by the subscription.
    pub acss_debit: Option<InvoicePaymentMethodOptionsAcssDebit>,

    /// This sub-hash contains details about the Bancontact payment method options to pass to invoices created by the subscription.
    pub bancontact: Option<InvoicePaymentMethodOptionsBancontact>,

    /// This sub-hash contains details about the Card payment method options to pass to invoices created by the subscription.
    pub card: Option<SubscriptionPaymentMethodOptionsCard>,

    /// This sub-hash contains details about the Bank transfer payment method options to pass to invoices created by the subscription.
    pub customer_balance: Option<InvoicePaymentMethodOptionsCustomerBalance>,

    /// This sub-hash contains details about the Konbini payment method options to pass to invoices created by the subscription.
    pub konbini: Option<InvoicePaymentMethodOptionsKonbini>,

    /// This sub-hash contains details about the ACH direct debit payment method options to pass to invoices created by the subscription.
    pub us_bank_account: Option<InvoicePaymentMethodOptionsUsBankAccount>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SubscriptionPaymentMethodOptionsCard {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<InvoiceMandateOptionsCard>,

    /// Selected network to process this Subscription on.
    ///
    /// Depends on the available networks of the card attached to the Subscription.
    /// Can be only set confirm-time.
    pub network: Option<SubscriptionPaymentMethodOptionsCardNetwork>,

    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
    ///
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    pub request_three_d_secure: Option<SubscriptionPaymentMethodOptionsCardRequestThreeDSecure>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InvoiceMandateOptionsCard {
    /// Amount to be charged for future payments.
    pub amount: Option<i64>,

    /// One of `fixed` or `maximum`.
    ///
    /// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
    /// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
    pub amount_type: Option<InvoiceMandateOptionsCardAmountType>,

    /// A description of the mandate or subscription that is meant to be displayed to the customer.
    pub description: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SubscriptionsResourcePendingUpdate {
    /// If the update is applied, determines the date of the first full invoice, and, for plans with `month` or `year` intervals, the day of the month for subsequent invoices.
    ///
    /// The timestamp is in UTC format.
    pub billing_cycle_anchor: Option<Timestamp>,

    /// The point after which the changes reflected by this update will be discarded and no longer applied.
    pub expires_at: Timestamp,

    /// List of subscription items, each with an attached plan, that will be set if the update is applied.
    pub subscription_items: Option<Vec<SubscriptionItem>>,

    /// Unix timestamp representing the end of the trial period the customer will get before being charged for the first time, if the update is applied.
    pub trial_end: Option<Timestamp>,

    /// Indicates if a plan's `trial_period_days` should be applied to the subscription.
    ///
    /// Setting `trial_end` per subscription is preferred, and this defaults to `false`.
    /// Setting this flag to `true` together with `trial_end` is not allowed.
    /// See [Using trial periods on subscriptions](https://stripe.com/docs/billing/subscriptions/trials) to learn more.
    pub trial_from_plan: Option<bool>,
}

/// The parameters for `Subscription::create`.
#[derive(Clone, Debug, Serialize)]
pub struct CreateSubscription<'a> {
    /// A list of prices and quantities that will generate invoice items appended to the next invoice for this subscription.
    ///
    /// You may pass up to 20 items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_invoice_items: Option<Vec<AddInvoiceItems>>,

    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice total that will be transferred to the application owner's Stripe account.
    /// The request must be made by a platform account on a connected account in order to set an application fee percentage.
    /// For more information, see the application fees [documentation](https://stripe.com/docs/connect/subscriptions#collecting-fees-on-subscriptions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,

    /// Automatic tax settings for this subscription.
    ///
    /// We recommend you only include this parameter when the existing value is being changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<CreateSubscriptionAutomaticTax>,

    /// For new subscriptions, a past timestamp to backdate the subscription's start date to.
    ///
    /// If set, the first invoice will contain a proration for the timespan between the start date and the current time.
    /// Can be combined with trials and the billing cycle anchor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backdate_start_date: Option<Timestamp>,

    /// A future timestamp in UTC format to anchor the subscription's [billing cycle](https://stripe.com/docs/subscriptions/billing-cycle).
    ///
    /// The anchor is the reference point that aligns future billing cycle dates.
    /// It sets the day of week for `week` intervals, the day of month for `month` and `year` intervals, and the month of year for `year` intervals.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor: Option<Timestamp>,

    /// Mutually exclusive with billing_cycle_anchor and only valid with monthly and yearly price intervals.
    ///
    /// When provided, the billing_cycle_anchor is set to the next occurence of the day_of_month at the hour, minute, and second UTC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor_config: Option<CreateSubscriptionBillingCycleAnchorConfig>,

    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    ///
    /// Pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<SubscriptionBillingThresholds>,

    /// A timestamp at which the subscription should cancel.
    ///
    /// If set to a date before the current period ends, this will cause a proration if prorations have been enabled using `proration_behavior`.
    /// If set during a future period, this will always cause a proration for that period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_at: Option<Timestamp>,

    /// Boolean indicating whether this subscription should cancel at the end of the current period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_at_period_end: Option<bool>,

    /// Either `charge_automatically`, or `send_invoice`.
    ///
    /// When charging automatically, Stripe will attempt to pay this subscription at the end of the cycle using the default source attached to the customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
    /// Defaults to `charge_automatically`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<CollectionMethod>,

    /// The ID of the coupon to apply to this subscription.
    ///
    /// A coupon applied to a subscription will only affect invoices created for that particular subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<CouponId>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// The identifier of the customer to subscribe.
    pub customer: CustomerId,

    /// Number of days a customer has to pay invoices generated by this subscription.
    ///
    /// Valid only for subscriptions where `collection_method` is set to `send_invoice`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<u32>,

    /// ID of the default payment method for the subscription.
    ///
    /// It must belong to the customer associated with the subscription.
    /// This takes precedence over `default_source`.
    /// If neither are set, invoices will use the customer's [invoice_settings.default_payment_method](https://stripe.com/docs/api/customers/object#customer_object-invoice_settings-default_payment_method) or [default_source](https://stripe.com/docs/api/customers/object#customer_object-default_source).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<&'a str>,

    /// ID of the default payment source for the subscription.
    ///
    /// It must belong to the customer associated with the subscription and be in a chargeable state.
    /// If `default_payment_method` is also set, `default_payment_method` will take precedence.
    /// If neither are set, invoices will use the customer's [invoice_settings.default_payment_method](https://stripe.com/docs/api/customers/object#customer_object-invoice_settings-default_payment_method) or [default_source](https://stripe.com/docs/api/customers/object#customer_object-default_source).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_source: Option<&'a str>,

    /// The tax rates that will apply to any subscription item that does not have `tax_rates` set.
    ///
    /// Invoices created will have their `default_tax_rates` populated from the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<String>>,

    /// The subscription's description, meant to be displayable to the customer.
    ///
    /// Use this field to optionally store an explanation of the subscription for rendering in Stripe surfaces and certain local payment methods UIs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// All invoices will be billed using the specified settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<CreateSubscriptionInvoiceSettings>,

    /// A list of up to 20 subscription items, each with an attached price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<CreateSubscriptionItems>>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Indicates if a customer is on or off-session while an invoice payment is attempted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub off_session: Option<bool>,

    /// The account on behalf of which to charge, for each of the subscription's invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,

    /// Only applies to subscriptions with `collection_method=charge_automatically`.
    ///
    /// Use `allow_incomplete` to create subscriptions with `status=incomplete` if the first invoice cannot be paid.
    ///
    /// Creating subscriptions with this status allows you to manage scenarios where additional user actions are needed to pay a subscription's invoice.
    /// For example, SCA regulation may require 3DS authentication to complete payment.
    /// See the [SCA Migration Guide](https://stripe.com/docs/billing/migration/strong-customer-authentication) for Billing to learn more.
    /// This is the default behavior.  Use `default_incomplete` to create Subscriptions with `status=incomplete` when the first invoice requires payment, otherwise start as active.
    /// Subscriptions transition to `status=active` when successfully confirming the payment intent on the first invoice.
    /// This allows simpler management of scenarios where additional user actions are needed to pay a subscription’s invoice.
    /// Such as failed payments, [SCA regulation](https://stripe.com/docs/billing/migration/strong-customer-authentication), or collecting a mandate for a bank debit payment method.
    /// If the payment intent is not confirmed within 23 hours subscriptions transition to `status=incomplete_expired`, which is a terminal state.  Use `error_if_incomplete` if you want Stripe to return an HTTP 402 status code if a subscription's first invoice cannot be paid.
    /// For example, if a payment method requires 3DS authentication due to SCA regulation and further user action is needed, this parameter does not create a subscription and returns an error instead.
    /// This was the default behavior for API versions prior to 2019-03-14.
    /// See the [changelog](https://stripe.com/docs/upgrades#2019-03-14) to learn more.  `pending_if_incomplete` is only used with updates and cannot be passed when creating a subscription.  Subscriptions with `collection_method=send_invoice` are automatically activated regardless of the first invoice status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_behavior: Option<SubscriptionPaymentBehavior>,

    /// Payment settings to pass to invoices created by the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_settings: Option<CreateSubscriptionPaymentSettings>,

    /// Specifies an interval for how often to bill for any pending invoice items.
    ///
    /// It is analogous to calling [Create an invoice](https://stripe.com/docs/api#create_invoice) for the given subscription at the specified interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_invoice_item_interval: Option<CreateSubscriptionPendingInvoiceItemInterval>,

    /// The API ID of a promotion code to apply to this subscription.
    ///
    /// A promotion code applied to a subscription will only affect invoices created for that particular subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<PromotionCodeId>,

    /// Determines how to handle [prorations](https://stripe.com/docs/subscriptions/billing-cycle#prorations) resulting from the `billing_cycle_anchor`.
    ///
    /// If no value is passed, the default is `create_prorations`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<SubscriptionProrationBehavior>,

    /// If specified, the funds from the subscription's invoices will be transferred to the destination and the ID of the resulting transfers will be found on the resulting charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<CreateSubscriptionTransferData>,

    /// Unix timestamp representing the end of the trial period the customer will get before being charged for the first time.
    ///
    /// If set, trial_end will override the default trial period of the plan the customer is being subscribed to.
    /// The special value `now` can be provided to end the customer's trial immediately.
    /// Can be at most two years from `billing_cycle_anchor`.
    /// See [Using trial periods on subscriptions](https://stripe.com/docs/billing/subscriptions/trials) to learn more.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<Scheduled>,

    /// Indicates if a plan's `trial_period_days` should be applied to the subscription.
    ///
    /// Setting `trial_end` per subscription is preferred, and this defaults to `false`.
    /// Setting this flag to `true` together with `trial_end` is not allowed.
    /// See [Using trial periods on subscriptions](https://stripe.com/docs/billing/subscriptions/trials) to learn more.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_from_plan: Option<bool>,

    /// Integer representing the number of trial period days before the customer is charged for the first time.
    ///
    /// This will always overwrite any trials that might apply via a subscribed plan.
    /// See [Using trial periods on subscriptions](https://stripe.com/docs/billing/subscriptions/trials) to learn more.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<u32>,

    /// Settings related to subscription trials.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_settings: Option<CreateSubscriptionTrialSettings>,
}

impl<'a> CreateSubscription<'a> {
    pub fn new(customer: CustomerId) -> Self {
        CreateSubscription {
            add_invoice_items: Default::default(),
            application_fee_percent: Default::default(),
            automatic_tax: Default::default(),
            backdate_start_date: Default::default(),
            billing_cycle_anchor: Default::default(),
            billing_cycle_anchor_config: Default::default(),
            billing_thresholds: Default::default(),
            cancel_at: Default::default(),
            cancel_at_period_end: Default::default(),
            collection_method: Default::default(),
            coupon: Default::default(),
            currency: Default::default(),
            customer,
            days_until_due: Default::default(),
            default_payment_method: Default::default(),
            default_source: Default::default(),
            default_tax_rates: Default::default(),
            description: Default::default(),
            expand: Default::default(),
            invoice_settings: Default::default(),
            items: Default::default(),
            metadata: Default::default(),
            off_session: Default::default(),
            on_behalf_of: Default::default(),
            payment_behavior: Default::default(),
            payment_settings: Default::default(),
            pending_invoice_item_interval: Default::default(),
            promotion_code: Default::default(),
            proration_behavior: Default::default(),
            transfer_data: Default::default(),
            trial_end: Default::default(),
            trial_from_plan: Default::default(),
            trial_period_days: Default::default(),
            trial_settings: Default::default(),
        }
    }
}

/// The parameters for `Subscription::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListSubscriptions<'a> {
    /// Filter subscriptions by their automatic tax settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<ListSubscriptionsAutomaticTax>,

    /// The collection method of the subscriptions to retrieve.
    ///
    /// Either `charge_automatically` or `send_invoice`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<CollectionMethod>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_period_end: Option<RangeQuery<Timestamp>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_period_start: Option<RangeQuery<Timestamp>>,

    /// The ID of the customer whose subscriptions will be retrieved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<SubscriptionId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// The ID of the plan whose subscriptions will be retrieved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<PlanId>,

    /// Filter for subscriptions that contain this recurring price ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<PriceId>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<SubscriptionId>,

    /// The status of the subscriptions to retrieve.
    ///
    /// Passing in a value of `canceled` will return all canceled subscriptions, including those belonging to deleted customers.
    /// Pass `ended` to find subscriptions that are canceled and subscriptions that are expired due to [incomplete payment](https://stripe.com/docs/billing/subscriptions/overview#subscription-statuses).
    /// Passing in a value of `all` will return subscriptions of all statuses.
    /// If no value is supplied, all subscriptions that have not been canceled are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<SubscriptionStatusFilter>,

    /// Filter for subscriptions that are associated with the specified test clock.
    ///
    /// The response will not include subscriptions with test clocks if this and the customer parameter is not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_clock: Option<&'a str>,
}

impl<'a> ListSubscriptions<'a> {
    pub fn new() -> Self {
        ListSubscriptions {
            automatic_tax: Default::default(),
            collection_method: Default::default(),
            created: Default::default(),
            current_period_end: Default::default(),
            current_period_start: Default::default(),
            customer: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            plan: Default::default(),
            price: Default::default(),
            starting_after: Default::default(),
            status: Default::default(),
            test_clock: Default::default(),
        }
    }
}
impl Paginable for ListSubscriptions<'_> {
    type O = Subscription;
    fn set_last(&mut self, item: Self::O) {
        self.starting_after = Some(item.id());
    }
}
/// The parameters for `Subscription::update`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct UpdateSubscription<'a> {
    /// A list of prices and quantities that will generate invoice items appended to the next invoice for this subscription.
    ///
    /// You may pass up to 20 items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_invoice_items: Option<Vec<AddInvoiceItems>>,

    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice total that will be transferred to the application owner's Stripe account.
    /// The request must be made by a platform account on a connected account in order to set an application fee percentage.
    /// For more information, see the application fees [documentation](https://stripe.com/docs/connect/subscriptions#collecting-fees-on-subscriptions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,

    /// Automatic tax settings for this subscription.
    ///
    /// We recommend you only include this parameter when the existing value is being changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<UpdateSubscriptionAutomaticTax>,

    /// Either `now` or `unchanged`.
    ///
    /// Setting the value to `now` resets the subscription's billing cycle anchor to the current time (in UTC).
    /// For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor: Option<SubscriptionBillingCycleAnchor>,

    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    ///
    /// Pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<SubscriptionBillingThresholds>,

    /// A timestamp at which the subscription should cancel.
    ///
    /// If set to a date before the current period ends, this will cause a proration if prorations have been enabled using `proration_behavior`.
    /// If set during a future period, this will always cause a proration for that period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_at: Option<Timestamp>,

    /// Boolean indicating whether this subscription should cancel at the end of the current period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_at_period_end: Option<bool>,

    /// Details about why this subscription was cancelled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_details: Option<UpdateSubscriptionCancellationDetails>,

    /// Either `charge_automatically`, or `send_invoice`.
    ///
    /// When charging automatically, Stripe will attempt to pay this subscription at the end of the cycle using the default source attached to the customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
    /// Defaults to `charge_automatically`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<CollectionMethod>,

    /// The ID of the coupon to apply to this subscription.
    ///
    /// A coupon applied to a subscription will only affect invoices created for that particular subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<CouponId>,

    /// Number of days a customer has to pay invoices generated by this subscription.
    ///
    /// Valid only for subscriptions where `collection_method` is set to `send_invoice`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<u32>,

    /// ID of the default payment method for the subscription.
    ///
    /// It must belong to the customer associated with the subscription.
    /// This takes precedence over `default_source`.
    /// If neither are set, invoices will use the customer's [invoice_settings.default_payment_method](https://stripe.com/docs/api/customers/object#customer_object-invoice_settings-default_payment_method) or [default_source](https://stripe.com/docs/api/customers/object#customer_object-default_source).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<&'a str>,

    /// ID of the default payment source for the subscription.
    ///
    /// It must belong to the customer associated with the subscription and be in a chargeable state.
    /// If `default_payment_method` is also set, `default_payment_method` will take precedence.
    /// If neither are set, invoices will use the customer's [invoice_settings.default_payment_method](https://stripe.com/docs/api/customers/object#customer_object-invoice_settings-default_payment_method) or [default_source](https://stripe.com/docs/api/customers/object#customer_object-default_source).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_source: Option<String>,

    /// The tax rates that will apply to any subscription item that does not have `tax_rates` set.
    ///
    /// Invoices created will have their `default_tax_rates` populated from the subscription.
    /// Pass an empty string to remove previously-defined tax rates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<String>>,

    /// The subscription's description, meant to be displayable to the customer.
    ///
    /// Use this field to optionally store an explanation of the subscription for rendering in Stripe surfaces and certain local payment methods UIs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// All invoices will be billed using the specified settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<UpdateSubscriptionInvoiceSettings>,

    /// A list of up to 20 subscription items, each with an attached price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<UpdateSubscriptionItems>>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Indicates if a customer is on or off-session while an invoice payment is attempted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub off_session: Option<bool>,

    /// The account on behalf of which to charge, for each of the subscription's invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,

    /// If specified, payment collection for this subscription will be paused.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pause_collection: Option<UpdateSubscriptionPauseCollection>,

    /// Use `allow_incomplete` to transition the subscription to `status=past_due` if a payment is required but cannot be paid.
    ///
    /// This allows you to manage scenarios where additional user actions are needed to pay a subscription's invoice.
    /// For example, SCA regulation may require 3DS authentication to complete payment.
    /// See the [SCA Migration Guide](https://stripe.com/docs/billing/migration/strong-customer-authentication) for Billing to learn more.
    /// This is the default behavior.  Use `default_incomplete` to transition the subscription to `status=past_due` when payment is required and await explicit confirmation of the invoice's payment intent.
    /// This allows simpler management of scenarios where additional user actions are needed to pay a subscription’s invoice.
    /// Such as failed payments, [SCA regulation](https://stripe.com/docs/billing/migration/strong-customer-authentication), or collecting a mandate for a bank debit payment method.  Use `pending_if_incomplete` to update the subscription using [pending updates](https://stripe.com/docs/billing/subscriptions/pending-updates).
    /// When you use `pending_if_incomplete` you can only pass the parameters [supported by pending updates](https://stripe.com/docs/billing/pending-updates-reference#supported-attributes).  Use `error_if_incomplete` if you want Stripe to return an HTTP 402 status code if a subscription's invoice cannot be paid.
    /// For example, if a payment method requires 3DS authentication due to SCA regulation and further user action is needed, this parameter does not update the subscription and returns an error instead.
    /// This was the default behavior for API versions prior to 2019-03-14.
    /// See the [changelog](https://stripe.com/docs/upgrades#2019-03-14) to learn more.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_behavior: Option<SubscriptionPaymentBehavior>,

    /// Payment settings to pass to invoices created by the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_settings: Option<UpdateSubscriptionPaymentSettings>,

    /// Specifies an interval for how often to bill for any pending invoice items.
    ///
    /// It is analogous to calling [Create an invoice](https://stripe.com/docs/api#create_invoice) for the given subscription at the specified interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_invoice_item_interval: Option<UpdateSubscriptionPendingInvoiceItemInterval>,

    /// The promotion code to apply to this subscription.
    ///
    /// A promotion code applied to a subscription will only affect invoices created for that particular subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<PromotionCodeId>,

    /// Determines how to handle [prorations](https://stripe.com/docs/subscriptions/billing-cycle#prorations) when the billing cycle changes (e.g., when switching plans, resetting `billing_cycle_anchor=now`, or starting a trial), or if an item's `quantity` changes.
    ///
    /// The default value is `create_prorations`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<SubscriptionProrationBehavior>,

    /// If set, the proration will be calculated as though the subscription was updated at the given time.
    ///
    /// This can be used to apply exactly the same proration that was previewed with [upcoming invoice](https://stripe.com/docs/api#upcoming_invoice) endpoint.
    /// It can also be used to implement custom proration logic, such as prorating by day instead of by second, by providing the time that you wish to use for proration calculations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_date: Option<Timestamp>,

    /// If specified, the funds from the subscription's invoices will be transferred to the destination and the ID of the resulting transfers will be found on the resulting charges.
    ///
    /// This will be unset if you POST an empty value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<UpdateSubscriptionTransferData>,

    /// Unix timestamp representing the end of the trial period the customer will get before being charged for the first time.
    ///
    /// This will always overwrite any trials that might apply via a subscribed plan.
    /// If set, trial_end will override the default trial period of the plan the customer is being subscribed to.
    /// The special value `now` can be provided to end the customer's trial immediately.
    /// Can be at most two years from `billing_cycle_anchor`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<Scheduled>,

    /// Indicates if a plan's `trial_period_days` should be applied to the subscription.
    ///
    /// Setting `trial_end` per subscription is preferred, and this defaults to `false`.
    /// Setting this flag to `true` together with `trial_end` is not allowed.
    /// See [Using trial periods on subscriptions](https://stripe.com/docs/billing/subscriptions/trials) to learn more.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_from_plan: Option<bool>,

    /// Settings related to subscription trials.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_settings: Option<UpdateSubscriptionTrialSettings>,
}

impl<'a> UpdateSubscription<'a> {
    pub fn new() -> Self {
        UpdateSubscription {
            add_invoice_items: Default::default(),
            application_fee_percent: Default::default(),
            automatic_tax: Default::default(),
            billing_cycle_anchor: Default::default(),
            billing_thresholds: Default::default(),
            cancel_at: Default::default(),
            cancel_at_period_end: Default::default(),
            cancellation_details: Default::default(),
            collection_method: Default::default(),
            coupon: Default::default(),
            days_until_due: Default::default(),
            default_payment_method: Default::default(),
            default_source: Default::default(),
            default_tax_rates: Default::default(),
            description: Default::default(),
            expand: Default::default(),
            invoice_settings: Default::default(),
            items: Default::default(),
            metadata: Default::default(),
            off_session: Default::default(),
            on_behalf_of: Default::default(),
            pause_collection: Default::default(),
            payment_behavior: Default::default(),
            payment_settings: Default::default(),
            pending_invoice_item_interval: Default::default(),
            promotion_code: Default::default(),
            proration_behavior: Default::default(),
            proration_date: Default::default(),
            transfer_data: Default::default(),
            trial_end: Default::default(),
            trial_from_plan: Default::default(),
            trial_settings: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AddInvoiceItems {
    /// The ID of the price object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<InvoiceItemPriceData>,

    /// Quantity for this item.
    ///
    /// Defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    /// The tax rates which apply to the item.
    ///
    /// When set, the `default_tax_rates` do not apply to this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSubscriptionAutomaticTax {
    /// Enabled automatic tax calculation which will automatically compute tax rates on all invoices generated by the subscription.
    pub enabled: bool,

    /// The account that's liable for tax.
    ///
    /// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
    /// The tax transaction is returned in the report of the connected account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liability: Option<CreateSubscriptionAutomaticTaxLiability>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSubscriptionBillingCycleAnchorConfig {
    /// The day of the month the billing_cycle_anchor should be.
    ///
    /// Ranges from 1 to 31.
    pub day_of_month: i64,

    /// The hour of the day the billing_cycle_anchor should be.
    ///
    /// Ranges from 0 to 23.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hour: Option<i64>,

    /// The minute of the hour the billing_cycle_anchor should be.
    ///
    /// Ranges from 0 to 59.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minute: Option<i64>,

    /// The month to start full cycle billing periods.
    ///
    /// Ranges from 1 to 12.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<i64>,

    /// The second of the minute the billing_cycle_anchor should be.
    ///
    /// Ranges from 0 to 59.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSubscriptionInvoiceSettings {
    /// The account tax IDs associated with the subscription.
    ///
    /// Will be set on invoices generated by the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_tax_ids: Option<Vec<String>>,

    /// The connected account that issues the invoice.
    ///
    /// The invoice is presented with the branding and support information of the specified account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<CreateSubscriptionInvoiceSettingsIssuer>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSubscriptionItems {
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    ///
    /// When updating, pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<CreateSubscriptionItemsBillingThresholds>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Plan ID for this item, as a string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,

    /// The ID of the price object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<SubscriptionItemPriceData>,

    /// Quantity for this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    /// A list of [Tax Rate](https://stripe.com/docs/api/tax_rates) ids.
    ///
    /// These Tax Rates will override the [`default_tax_rates`](https://stripe.com/docs/api/subscriptions/create#create_subscription-default_tax_rates) on the Subscription.
    /// When updating, pass an empty string to remove previously-defined tax rates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSubscriptionPaymentSettings {
    /// Payment-method-specific configuration to provide to invoices created by the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<CreateSubscriptionPaymentSettingsPaymentMethodOptions>,

    /// The list of payment method types (e.g.
    ///
    /// card) to provide to the invoice’s PaymentIntent.
    /// If not set, Stripe attempts to automatically determine the types to use by looking at the invoice’s default payment method, the subscription’s default payment method, the customer’s default payment method, and your [invoice template settings](https://dashboard.stripe.com/settings/billing/invoice).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<Vec<CreateSubscriptionPaymentSettingsPaymentMethodTypes>>,

    /// Either `off`, or `on_subscription`.
    ///
    /// With `on_subscription` Stripe updates `subscription.default_payment_method` when a subscription payment succeeds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_default_payment_method:
        Option<CreateSubscriptionPaymentSettingsSaveDefaultPaymentMethod>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSubscriptionPendingInvoiceItemInterval {
    /// Specifies invoicing frequency.
    ///
    /// Either `day`, `week`, `month` or `year`.
    pub interval: PlanInterval,

    /// The number of intervals between invoices.
    ///
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSubscriptionTransferData {
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice total that will be transferred to the destination account.
    /// By default, the entire amount is transferred to the destination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_percent: Option<f64>,

    /// ID of an existing, connected Stripe account.
    pub destination: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSubscriptionTrialSettings {
    /// Defines how the subscription should behave when the user's free trial ends.
    pub end_behavior: CreateSubscriptionTrialSettingsEndBehavior,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ListSubscriptionsAutomaticTax {
    /// Enabled automatic tax calculation which will automatically compute tax rates on all invoices generated by the subscription.
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSubscriptionAutomaticTax {
    /// Enabled automatic tax calculation which will automatically compute tax rates on all invoices generated by the subscription.
    pub enabled: bool,

    /// The account that's liable for tax.
    ///
    /// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
    /// The tax transaction is returned in the report of the connected account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liability: Option<UpdateSubscriptionAutomaticTaxLiability>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSubscriptionCancellationDetails {
    /// Additional comments about why the user canceled the subscription, if the subscription was canceled explicitly by the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    /// The customer submitted reason for why they canceled, if the subscription was canceled explicitly by the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feedback: Option<UpdateSubscriptionCancellationDetailsFeedback>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSubscriptionInvoiceSettings {
    /// The account tax IDs associated with the subscription.
    ///
    /// Will be set on invoices generated by the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_tax_ids: Option<Vec<String>>,

    /// The connected account that issues the invoice.
    ///
    /// The invoice is presented with the branding and support information of the specified account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<UpdateSubscriptionInvoiceSettingsIssuer>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSubscriptionItems {
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    ///
    /// When updating, pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<SubscriptionItemBillingThresholds>,

    /// Delete all usage for a given subscription item.
    ///
    /// Allowed only when `deleted` is set to `true` and the current plan's `usage_type` is `metered`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear_usage: Option<bool>,

    /// A flag that, if set to `true`, will delete the specified item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,

    /// Subscription item to update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Plan ID for this item, as a string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,

    /// The ID of the price object.
    ///
    /// When changing a subscription item's price, `quantity` is set to 1 unless a `quantity` parameter is provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<SubscriptionItemPriceData>,

    /// Quantity for this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    /// A list of [Tax Rate](https://stripe.com/docs/api/tax_rates) ids.
    ///
    /// These Tax Rates will override the [`default_tax_rates`](https://stripe.com/docs/api/subscriptions/create#create_subscription-default_tax_rates) on the Subscription.
    /// When updating, pass an empty string to remove previously-defined tax rates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSubscriptionPauseCollection {
    /// The payment collection behavior for this subscription while paused.
    ///
    /// One of `keep_as_draft`, `mark_uncollectible`, or `void`.
    pub behavior: UpdateSubscriptionPauseCollectionBehavior,

    /// The time after which the subscription will resume collecting payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resumes_at: Option<Timestamp>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSubscriptionPaymentSettings {
    /// Payment-method-specific configuration to provide to invoices created by the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<UpdateSubscriptionPaymentSettingsPaymentMethodOptions>,

    /// The list of payment method types (e.g.
    ///
    /// card) to provide to the invoice’s PaymentIntent.
    /// If not set, Stripe attempts to automatically determine the types to use by looking at the invoice’s default payment method, the subscription’s default payment method, the customer’s default payment method, and your [invoice template settings](https://dashboard.stripe.com/settings/billing/invoice).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<Vec<UpdateSubscriptionPaymentSettingsPaymentMethodTypes>>,

    /// Either `off`, or `on_subscription`.
    ///
    /// With `on_subscription` Stripe updates `subscription.default_payment_method` when a subscription payment succeeds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_default_payment_method:
        Option<UpdateSubscriptionPaymentSettingsSaveDefaultPaymentMethod>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSubscriptionPendingInvoiceItemInterval {
    /// Specifies invoicing frequency.
    ///
    /// Either `day`, `week`, `month` or `year`.
    pub interval: PlanInterval,

    /// The number of intervals between invoices.
    ///
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSubscriptionTransferData {
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice total that will be transferred to the destination account.
    /// By default, the entire amount is transferred to the destination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_percent: Option<f64>,

    /// ID of an existing, connected Stripe account.
    pub destination: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSubscriptionTrialSettings {
    /// Defines how the subscription should behave when the user's free trial ends.
    pub end_behavior: UpdateSubscriptionTrialSettingsEndBehavior,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSubscriptionAutomaticTaxLiability {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,

    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: CreateSubscriptionAutomaticTaxLiabilityType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSubscriptionInvoiceSettingsIssuer {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,

    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: CreateSubscriptionInvoiceSettingsIssuerType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSubscriptionItemsBillingThresholds {
    /// Number of units that meets the billing threshold to advance the subscription to a new billing period (e.g., it takes 10 $5 units to meet a $50 [monetary threshold](https://stripe.com/docs/api/subscriptions/update#update_subscription-billing_thresholds-amount_gte)).
    pub usage_gte: i64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSubscriptionPaymentSettingsPaymentMethodOptions {
    /// This sub-hash contains details about the Canadian pre-authorized debit payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebit>,

    /// This sub-hash contains details about the Bancontact payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<CreateSubscriptionPaymentSettingsPaymentMethodOptionsBancontact>,

    /// This sub-hash contains details about the Card payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CreateSubscriptionPaymentSettingsPaymentMethodOptionsCard>,

    /// This sub-hash contains details about the Bank transfer payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance:
        Option<CreateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalance>,

    /// This sub-hash contains details about the Konbini payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<CreateSubscriptionPaymentSettingsPaymentMethodOptionsKonbini>,

    /// This sub-hash contains details about the ACH direct debit payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccount>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSubscriptionTrialSettingsEndBehavior {
    /// Indicates how the subscription should change when the trial ends if the user did not provide a payment method.
    pub missing_payment_method: CreateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InvoiceItemPriceData {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// The ID of the product that this price will belong to.
    pub product: String,

    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    ///
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<InvoiceItemPriceDataTaxBehavior>,

    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,

    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SubscriptionItemPriceData {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// The ID of the product that this price will belong to.
    pub product: String,

    /// The recurring components of a price such as `interval` and `interval_count`.
    pub recurring: SubscriptionItemPriceDataRecurring,

    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    ///
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<SubscriptionItemPriceDataTaxBehavior>,

    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,

    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSubscriptionAutomaticTaxLiability {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,

    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: UpdateSubscriptionAutomaticTaxLiabilityType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSubscriptionInvoiceSettingsIssuer {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,

    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: UpdateSubscriptionInvoiceSettingsIssuerType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSubscriptionPaymentSettingsPaymentMethodOptions {
    /// This sub-hash contains details about the Canadian pre-authorized debit payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebit>,

    /// This sub-hash contains details about the Bancontact payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsBancontact>,

    /// This sub-hash contains details about the Card payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCard>,

    /// This sub-hash contains details about the Bank transfer payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance:
        Option<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalance>,

    /// This sub-hash contains details about the Konbini payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsKonbini>,

    /// This sub-hash contains details about the ACH direct debit payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccount>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSubscriptionTrialSettingsEndBehavior {
    /// Indicates how the subscription should change when the trial ends if the user did not provide a payment method.
    pub missing_payment_method: UpdateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebit {
    /// Additional fields for Mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options:
        Option<CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions>,

    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSubscriptionPaymentSettingsPaymentMethodOptionsBancontact {
    /// Preferred language of the Bancontact authorization page that the customer is redirected to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language:
        Option<CreateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSubscriptionPaymentSettingsPaymentMethodOptionsCard {
    /// Configuration options for setting up an eMandate for cards issued in India.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options:
        Option<CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptions>,

    /// Selected network to process this Subscription on.
    ///
    /// Depends on the available networks of the card attached to the Subscription.
    /// Can be only set confirm-time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork>,

    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
    ///
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_three_d_secure:
        Option<CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalance {
    /// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer:
        Option<CreateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer>,

    /// The funding method type to be used when there are not enough funds in the customer balance.
    ///
    /// Permitted values include: `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_type: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSubscriptionPaymentSettingsPaymentMethodOptionsKonbini {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccount {
    /// Additional fields for Financial Connections Session creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections: Option<
        CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections,
    >,

    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<
        CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod,
    >,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SubscriptionItemPriceDataRecurring {
    /// Specifies billing frequency.
    ///
    /// Either `day`, `week`, `month` or `year`.
    pub interval: PlanInterval,

    /// The number of intervals between subscription billings.
    ///
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of three years interval allowed (3 years, 36 months, or 156 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebit {
    /// Additional fields for Mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options:
        Option<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions>,

    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSubscriptionPaymentSettingsPaymentMethodOptionsBancontact {
    /// Preferred language of the Bancontact authorization page that the customer is redirected to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language:
        Option<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCard {
    /// Configuration options for setting up an eMandate for cards issued in India.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options:
        Option<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptions>,

    /// Selected network to process this Subscription on.
    ///
    /// Depends on the available networks of the card attached to the Subscription.
    /// Can be only set confirm-time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork>,

    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
    ///
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_three_d_secure:
        Option<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalance {
    /// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer:
        Option<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer>,

    /// The funding method type to be used when there are not enough funds in the customer balance.
    ///
    /// Permitted values include: `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_type: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSubscriptionPaymentSettingsPaymentMethodOptionsKonbini {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccount {
    /// Additional fields for Financial Connections Session creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections: Option<
        UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections,
    >,

    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<
        UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod,
    >,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions {
    /// Transaction type of the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<
        CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType,
    >,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptions {
    /// Amount to be charged for future payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// One of `fixed` or `maximum`.
    ///
    /// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
    /// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_type:
        Option<CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType>,

    /// A description of the mandate or subscription that is meant to be displayed to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer {

    /// Configuration for eu_bank_transfer funding type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_bank_transfer: Option<CreateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer>,

    /// The bank transfer type that can be used for funding.
    ///
    /// Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, `mx_bank_transfer`, or `us_bank_transfer`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections {

    /// The list of permissions to request.
    ///
    /// If this parameter is passed, the `payment_method` permission must be included.
    /// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions>>,

    /// List of data features that you would like to retrieve upon account creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefetch: Option<Vec<CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions {
    /// Transaction type of the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<
        UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType,
    >,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptions {
    /// Amount to be charged for future payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// One of `fixed` or `maximum`.
    ///
    /// If `fixed`, the `amount` param refers to the exact amount to be charged in future payments.
    /// If `maximum`, the amount charged can be up to the value passed for the `amount` param.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_type:
        Option<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType>,

    /// A description of the mandate or subscription that is meant to be displayed to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer {

    /// Configuration for eu_bank_transfer funding type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_bank_transfer: Option<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer>,

    /// The bank transfer type that can be used for funding.
    ///
    /// Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, `mx_bank_transfer`, or `us_bank_transfer`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections {

    /// The list of permissions to request.
    ///
    /// If this parameter is passed, the `payment_method` permission must be included.
    /// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions>>,

    /// List of data features that you would like to retrieve upon account creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefetch: Option<Vec<UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer
{
    /// The desired country code of the bank account information.
    ///
    /// Permitted values include: `BE`, `DE`, `ES`, `FR`, `IE`, or `NL`.
    pub country: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer
{
    /// The desired country code of the bank account information.
    ///
    /// Permitted values include: `BE`, `DE`, `ES`, `FR`, `IE`, or `NL`.
    pub country: String,
}

/// An enum representing the possible values of an `CancellationDetails`'s `feedback` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CancellationDetailsFeedback {
    CustomerService,
    LowQuality,
    MissingFeatures,
    Other,
    SwitchedService,
    TooComplex,
    TooExpensive,
    Unused,
}

impl CancellationDetailsFeedback {
    pub fn as_str(self) -> &'static str {
        match self {
            CancellationDetailsFeedback::CustomerService => "customer_service",
            CancellationDetailsFeedback::LowQuality => "low_quality",
            CancellationDetailsFeedback::MissingFeatures => "missing_features",
            CancellationDetailsFeedback::Other => "other",
            CancellationDetailsFeedback::SwitchedService => "switched_service",
            CancellationDetailsFeedback::TooComplex => "too_complex",
            CancellationDetailsFeedback::TooExpensive => "too_expensive",
            CancellationDetailsFeedback::Unused => "unused",
        }
    }
}

impl AsRef<str> for CancellationDetailsFeedback {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CancellationDetailsFeedback {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CancellationDetailsFeedback {
    fn default() -> Self {
        Self::CustomerService
    }
}

/// An enum representing the possible values of an `CancellationDetails`'s `reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CancellationDetailsReason {
    CancellationRequested,
    PaymentDisputed,
    PaymentFailed,
}

impl CancellationDetailsReason {
    pub fn as_str(self) -> &'static str {
        match self {
            CancellationDetailsReason::CancellationRequested => "cancellation_requested",
            CancellationDetailsReason::PaymentDisputed => "payment_disputed",
            CancellationDetailsReason::PaymentFailed => "payment_failed",
        }
    }
}

impl AsRef<str> for CancellationDetailsReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CancellationDetailsReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CancellationDetailsReason {
    fn default() -> Self {
        Self::CancellationRequested
    }
}

/// An enum representing the possible values of an `CreateSubscriptionAutomaticTaxLiability`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionAutomaticTaxLiabilityType {
    Account,
    #[serde(rename = "self")]
    Self_,
}

impl CreateSubscriptionAutomaticTaxLiabilityType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSubscriptionAutomaticTaxLiabilityType::Account => "account",
            CreateSubscriptionAutomaticTaxLiabilityType::Self_ => "self",
        }
    }
}

impl AsRef<str> for CreateSubscriptionAutomaticTaxLiabilityType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSubscriptionAutomaticTaxLiabilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateSubscriptionAutomaticTaxLiabilityType {
    fn default() -> Self {
        Self::Account
    }
}

/// An enum representing the possible values of an `CreateSubscriptionInvoiceSettingsIssuer`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionInvoiceSettingsIssuerType {
    Account,
    #[serde(rename = "self")]
    Self_,
}

impl CreateSubscriptionInvoiceSettingsIssuerType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSubscriptionInvoiceSettingsIssuerType::Account => "account",
            CreateSubscriptionInvoiceSettingsIssuerType::Self_ => "self",
        }
    }
}

impl AsRef<str> for CreateSubscriptionInvoiceSettingsIssuerType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSubscriptionInvoiceSettingsIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateSubscriptionInvoiceSettingsIssuerType {
    fn default() -> Self {
        Self::Account
    }
}

/// An enum representing the possible values of an `CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions`'s `transaction_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    Business,
    Personal,
}

impl CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Business => "business",
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Personal => "personal",
        }
    }
}

impl AsRef<str>
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn default() -> Self {
        Self::Business
    }
}

/// An enum representing the possible values of an `CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebit`'s `verification_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::Automatic => "automatic",
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::Instant => "instant",
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::Microdeposits => "microdeposits",
        }
    }
}

impl AsRef<str>
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    fn default() -> Self {
        Self::Automatic
    }
}

/// An enum representing the possible values of an `CreateSubscriptionPaymentSettingsPaymentMethodOptionsBancontact`'s `preferred_language` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    De,
    En,
    Fr,
    Nl,
}

impl CreateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::De => "de",
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::En => "en",
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::Fr => "fr",
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::Nl => "nl",
        }
    }
}

impl AsRef<str>
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    fn default() -> Self {
        Self::De
    }
}

/// An enum representing the possible values of an `CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptions`'s `amount_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType {
    Fixed,
    Maximum,
}

impl CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType::Fixed => "fixed",
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType::Maximum => "maximum",
        }
    }
}

impl AsRef<str>
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType
{
    fn default() -> Self {
        Self::Fixed
    }
}

/// An enum representing the possible values of an `CreateSubscriptionPaymentSettingsPaymentMethodOptionsCard`'s `network` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork {
    Amex,
    CartesBancaires,
    Diners,
    Discover,
    EftposAu,
    Interac,
    Jcb,
    Mastercard,
    Unionpay,
    Unknown,
    Visa,
}

impl CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork::Amex => "amex",
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork::CartesBancaires => {
                "cartes_bancaires"
            }
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork::Diners => "diners",
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork::Discover => {
                "discover"
            }
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork::EftposAu => {
                "eftpos_au"
            }
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork::Interac => "interac",
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork::Jcb => "jcb",
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork::Mastercard => {
                "mastercard"
            }
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork::Unionpay => {
                "unionpay"
            }
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork::Unknown => "unknown",
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork::Visa => "visa",
        }
    }
}

impl AsRef<str> for CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork {
    fn default() -> Self {
        Self::Amex
    }
}

/// An enum representing the possible values of an `CreateSubscriptionPaymentSettingsPaymentMethodOptionsCard`'s `request_three_d_secure` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
    Challenge,
}

impl CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure::Any => "any",
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure::Automatic => "automatic",
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure::Challenge => "challenge",
        }
    }
}

impl AsRef<str> for CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure
{
    fn default() -> Self {
        Self::Any
    }
}

/// An enum representing the possible values of an `CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections`'s `permissions` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}

impl CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions::Balances => "balances",
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions::Ownership => "ownership",
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions::PaymentMethod => "payment_method",
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions::Transactions => "transactions",
        }
    }
}

impl AsRef<str> for CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    fn default() -> Self {
        Self::Balances
    }
}

/// An enum representing the possible values of an `CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections`'s `prefetch` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    Balances,
    Transactions,
}

impl
    CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch::Balances => "balances",
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch::Transactions => "transactions",
        }
    }
}

impl AsRef<str> for CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch {
    fn default() -> Self {
        Self::Balances
    }
}

/// An enum representing the possible values of an `CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccount`'s `verification_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod::Automatic => "automatic",
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod::Instant => "instant",
            CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod::Microdeposits => "microdeposits",
        }
    }
}

impl AsRef<str>
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod
{
    fn default() -> Self {
        Self::Automatic
    }
}

/// An enum representing the possible values of an `CreateSubscriptionPaymentSettings`'s `payment_method_types` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionPaymentSettingsPaymentMethodTypes {
    AchCreditTransfer,
    AchDebit,
    AcssDebit,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Boleto,
    Card,
    Cashapp,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Konbini,
    Link,
    P24,
    Paynow,
    Paypal,
    Promptpay,
    SepaCreditTransfer,
    SepaDebit,
    Sofort,
    UsBankAccount,
    WechatPay,
}

impl CreateSubscriptionPaymentSettingsPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSubscriptionPaymentSettingsPaymentMethodTypes::AchCreditTransfer => {
                "ach_credit_transfer"
            }
            CreateSubscriptionPaymentSettingsPaymentMethodTypes::AchDebit => "ach_debit",
            CreateSubscriptionPaymentSettingsPaymentMethodTypes::AcssDebit => "acss_debit",
            CreateSubscriptionPaymentSettingsPaymentMethodTypes::AuBecsDebit => "au_becs_debit",
            CreateSubscriptionPaymentSettingsPaymentMethodTypes::BacsDebit => "bacs_debit",
            CreateSubscriptionPaymentSettingsPaymentMethodTypes::Bancontact => "bancontact",
            CreateSubscriptionPaymentSettingsPaymentMethodTypes::Boleto => "boleto",
            CreateSubscriptionPaymentSettingsPaymentMethodTypes::Card => "card",
            CreateSubscriptionPaymentSettingsPaymentMethodTypes::Cashapp => "cashapp",
            CreateSubscriptionPaymentSettingsPaymentMethodTypes::CustomerBalance => {
                "customer_balance"
            }
            CreateSubscriptionPaymentSettingsPaymentMethodTypes::Eps => "eps",
            CreateSubscriptionPaymentSettingsPaymentMethodTypes::Fpx => "fpx",
            CreateSubscriptionPaymentSettingsPaymentMethodTypes::Giropay => "giropay",
            CreateSubscriptionPaymentSettingsPaymentMethodTypes::Grabpay => "grabpay",
            CreateSubscriptionPaymentSettingsPaymentMethodTypes::Ideal => "ideal",
            CreateSubscriptionPaymentSettingsPaymentMethodTypes::Konbini => "konbini",
            CreateSubscriptionPaymentSettingsPaymentMethodTypes::Link => "link",
            CreateSubscriptionPaymentSettingsPaymentMethodTypes::P24 => "p24",
            CreateSubscriptionPaymentSettingsPaymentMethodTypes::Paynow => "paynow",
            CreateSubscriptionPaymentSettingsPaymentMethodTypes::Paypal => "paypal",
            CreateSubscriptionPaymentSettingsPaymentMethodTypes::Promptpay => "promptpay",
            CreateSubscriptionPaymentSettingsPaymentMethodTypes::SepaCreditTransfer => {
                "sepa_credit_transfer"
            }
            CreateSubscriptionPaymentSettingsPaymentMethodTypes::SepaDebit => "sepa_debit",
            CreateSubscriptionPaymentSettingsPaymentMethodTypes::Sofort => "sofort",
            CreateSubscriptionPaymentSettingsPaymentMethodTypes::UsBankAccount => "us_bank_account",
            CreateSubscriptionPaymentSettingsPaymentMethodTypes::WechatPay => "wechat_pay",
        }
    }
}

impl AsRef<str> for CreateSubscriptionPaymentSettingsPaymentMethodTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSubscriptionPaymentSettingsPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateSubscriptionPaymentSettingsPaymentMethodTypes {
    fn default() -> Self {
        Self::AchCreditTransfer
    }
}

/// An enum representing the possible values of an `CreateSubscriptionPaymentSettings`'s `save_default_payment_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionPaymentSettingsSaveDefaultPaymentMethod {
    Off,
    OnSubscription,
}

impl CreateSubscriptionPaymentSettingsSaveDefaultPaymentMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSubscriptionPaymentSettingsSaveDefaultPaymentMethod::Off => "off",
            CreateSubscriptionPaymentSettingsSaveDefaultPaymentMethod::OnSubscription => {
                "on_subscription"
            }
        }
    }
}

impl AsRef<str> for CreateSubscriptionPaymentSettingsSaveDefaultPaymentMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSubscriptionPaymentSettingsSaveDefaultPaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateSubscriptionPaymentSettingsSaveDefaultPaymentMethod {
    fn default() -> Self {
        Self::Off
    }
}

/// An enum representing the possible values of an `CreateSubscriptionTrialSettingsEndBehavior`'s `missing_payment_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod {
    Cancel,
    CreateInvoice,
    Pause,
}

impl CreateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod::Cancel => "cancel",
            CreateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod::CreateInvoice => {
                "create_invoice"
            }
            CreateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod::Pause => "pause",
        }
    }
}

impl AsRef<str> for CreateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod {
    fn default() -> Self {
        Self::Cancel
    }
}

/// An enum representing the possible values of an `InvoiceItemPriceData`'s `tax_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceItemPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl InvoiceItemPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            InvoiceItemPriceDataTaxBehavior::Exclusive => "exclusive",
            InvoiceItemPriceDataTaxBehavior::Inclusive => "inclusive",
            InvoiceItemPriceDataTaxBehavior::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for InvoiceItemPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InvoiceItemPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for InvoiceItemPriceDataTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}

/// An enum representing the possible values of an `InvoiceMandateOptionsCard`'s `amount_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceMandateOptionsCardAmountType {
    Fixed,
    Maximum,
}

impl InvoiceMandateOptionsCardAmountType {
    pub fn as_str(self) -> &'static str {
        match self {
            InvoiceMandateOptionsCardAmountType::Fixed => "fixed",
            InvoiceMandateOptionsCardAmountType::Maximum => "maximum",
        }
    }
}

impl AsRef<str> for InvoiceMandateOptionsCardAmountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InvoiceMandateOptionsCardAmountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for InvoiceMandateOptionsCardAmountType {
    fn default() -> Self {
        Self::Fixed
    }
}

/// An enum representing the possible values of an `SubscriptionPendingInvoiceItemInterval`'s `interval` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PlanInterval {
    Day,
    Month,
    Week,
    Year,
}

impl PlanInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            PlanInterval::Day => "day",
            PlanInterval::Month => "month",
            PlanInterval::Week => "week",
            PlanInterval::Year => "year",
        }
    }
}

impl AsRef<str> for PlanInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PlanInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PlanInterval {
    fn default() -> Self {
        Self::Day
    }
}

/// An enum representing the possible values of an `UpdateSubscription`'s `billing_cycle_anchor` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionBillingCycleAnchor {
    Now,
    Unchanged,
}

impl SubscriptionBillingCycleAnchor {
    pub fn as_str(self) -> &'static str {
        match self {
            SubscriptionBillingCycleAnchor::Now => "now",
            SubscriptionBillingCycleAnchor::Unchanged => "unchanged",
        }
    }
}

impl AsRef<str> for SubscriptionBillingCycleAnchor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for SubscriptionBillingCycleAnchor {
    fn default() -> Self {
        Self::Now
    }
}

/// An enum representing the possible values of an `SubscriptionItemPriceData`'s `tax_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionItemPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl SubscriptionItemPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            SubscriptionItemPriceDataTaxBehavior::Exclusive => "exclusive",
            SubscriptionItemPriceDataTaxBehavior::Inclusive => "inclusive",
            SubscriptionItemPriceDataTaxBehavior::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for SubscriptionItemPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionItemPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for SubscriptionItemPriceDataTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}

/// An enum representing the possible values of an `CreateSubscription`'s `payment_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionPaymentBehavior {
    AllowIncomplete,
    DefaultIncomplete,
    ErrorIfIncomplete,
    PendingIfIncomplete,
}

impl SubscriptionPaymentBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            SubscriptionPaymentBehavior::AllowIncomplete => "allow_incomplete",
            SubscriptionPaymentBehavior::DefaultIncomplete => "default_incomplete",
            SubscriptionPaymentBehavior::ErrorIfIncomplete => "error_if_incomplete",
            SubscriptionPaymentBehavior::PendingIfIncomplete => "pending_if_incomplete",
        }
    }
}

impl AsRef<str> for SubscriptionPaymentBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionPaymentBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for SubscriptionPaymentBehavior {
    fn default() -> Self {
        Self::AllowIncomplete
    }
}

/// An enum representing the possible values of an `SubscriptionPaymentMethodOptionsCard`'s `network` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionPaymentMethodOptionsCardNetwork {
    Amex,
    CartesBancaires,
    Diners,
    Discover,
    EftposAu,
    Interac,
    Jcb,
    Mastercard,
    Unionpay,
    Unknown,
    Visa,
}

impl SubscriptionPaymentMethodOptionsCardNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            SubscriptionPaymentMethodOptionsCardNetwork::Amex => "amex",
            SubscriptionPaymentMethodOptionsCardNetwork::CartesBancaires => "cartes_bancaires",
            SubscriptionPaymentMethodOptionsCardNetwork::Diners => "diners",
            SubscriptionPaymentMethodOptionsCardNetwork::Discover => "discover",
            SubscriptionPaymentMethodOptionsCardNetwork::EftposAu => "eftpos_au",
            SubscriptionPaymentMethodOptionsCardNetwork::Interac => "interac",
            SubscriptionPaymentMethodOptionsCardNetwork::Jcb => "jcb",
            SubscriptionPaymentMethodOptionsCardNetwork::Mastercard => "mastercard",
            SubscriptionPaymentMethodOptionsCardNetwork::Unionpay => "unionpay",
            SubscriptionPaymentMethodOptionsCardNetwork::Unknown => "unknown",
            SubscriptionPaymentMethodOptionsCardNetwork::Visa => "visa",
        }
    }
}

impl AsRef<str> for SubscriptionPaymentMethodOptionsCardNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionPaymentMethodOptionsCardNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for SubscriptionPaymentMethodOptionsCardNetwork {
    fn default() -> Self {
        Self::Amex
    }
}

/// An enum representing the possible values of an `SubscriptionPaymentMethodOptionsCard`'s `request_three_d_secure` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
    Challenge,
}

impl SubscriptionPaymentMethodOptionsCardRequestThreeDSecure {
    pub fn as_str(self) -> &'static str {
        match self {
            SubscriptionPaymentMethodOptionsCardRequestThreeDSecure::Any => "any",
            SubscriptionPaymentMethodOptionsCardRequestThreeDSecure::Automatic => "automatic",
            SubscriptionPaymentMethodOptionsCardRequestThreeDSecure::Challenge => "challenge",
        }
    }
}

impl AsRef<str> for SubscriptionPaymentMethodOptionsCardRequestThreeDSecure {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionPaymentMethodOptionsCardRequestThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for SubscriptionPaymentMethodOptionsCardRequestThreeDSecure {
    fn default() -> Self {
        Self::Any
    }
}

/// An enum representing the possible values of an `CreateSubscription`'s `proration_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}

impl SubscriptionProrationBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            SubscriptionProrationBehavior::AlwaysInvoice => "always_invoice",
            SubscriptionProrationBehavior::CreateProrations => "create_prorations",
            SubscriptionProrationBehavior::None => "none",
        }
    }
}

impl AsRef<str> for SubscriptionProrationBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for SubscriptionProrationBehavior {
    fn default() -> Self {
        Self::AlwaysInvoice
    }
}

/// An enum representing the possible values of an `Subscription`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionStatus {
    Active,
    Canceled,
    Incomplete,
    IncompleteExpired,
    PastDue,
    Paused,
    Trialing,
    Unpaid,
}

impl SubscriptionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            SubscriptionStatus::Active => "active",
            SubscriptionStatus::Canceled => "canceled",
            SubscriptionStatus::Incomplete => "incomplete",
            SubscriptionStatus::IncompleteExpired => "incomplete_expired",
            SubscriptionStatus::PastDue => "past_due",
            SubscriptionStatus::Paused => "paused",
            SubscriptionStatus::Trialing => "trialing",
            SubscriptionStatus::Unpaid => "unpaid",
        }
    }
}

impl AsRef<str> for SubscriptionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for SubscriptionStatus {
    fn default() -> Self {
        Self::Active
    }
}

/// An enum representing the possible values of an `ListSubscriptions`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionStatusFilter {
    Active,
    All,
    Canceled,
    Ended,
    Incomplete,
    IncompleteExpired,
    PastDue,
    Paused,
    Trialing,
    Unpaid,
}

impl SubscriptionStatusFilter {
    pub fn as_str(self) -> &'static str {
        match self {
            SubscriptionStatusFilter::Active => "active",
            SubscriptionStatusFilter::All => "all",
            SubscriptionStatusFilter::Canceled => "canceled",
            SubscriptionStatusFilter::Ended => "ended",
            SubscriptionStatusFilter::Incomplete => "incomplete",
            SubscriptionStatusFilter::IncompleteExpired => "incomplete_expired",
            SubscriptionStatusFilter::PastDue => "past_due",
            SubscriptionStatusFilter::Paused => "paused",
            SubscriptionStatusFilter::Trialing => "trialing",
            SubscriptionStatusFilter::Unpaid => "unpaid",
        }
    }
}

impl AsRef<str> for SubscriptionStatusFilter {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionStatusFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for SubscriptionStatusFilter {
    fn default() -> Self {
        Self::Active
    }
}

/// An enum representing the possible values of an `SubscriptionsResourcePauseCollection`'s `behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionsResourcePauseCollectionBehavior {
    KeepAsDraft,
    MarkUncollectible,
    Void,
}

impl SubscriptionsResourcePauseCollectionBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            SubscriptionsResourcePauseCollectionBehavior::KeepAsDraft => "keep_as_draft",
            SubscriptionsResourcePauseCollectionBehavior::MarkUncollectible => "mark_uncollectible",
            SubscriptionsResourcePauseCollectionBehavior::Void => "void",
        }
    }
}

impl AsRef<str> for SubscriptionsResourcePauseCollectionBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionsResourcePauseCollectionBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for SubscriptionsResourcePauseCollectionBehavior {
    fn default() -> Self {
        Self::KeepAsDraft
    }
}

/// An enum representing the possible values of an `SubscriptionsResourcePaymentSettings`'s `payment_method_types` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionsResourcePaymentSettingsPaymentMethodTypes {
    AchCreditTransfer,
    AchDebit,
    AcssDebit,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Boleto,
    Card,
    Cashapp,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Konbini,
    Link,
    P24,
    Paynow,
    Paypal,
    Promptpay,
    SepaCreditTransfer,
    SepaDebit,
    Sofort,
    UsBankAccount,
    WechatPay,
}

impl SubscriptionsResourcePaymentSettingsPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            SubscriptionsResourcePaymentSettingsPaymentMethodTypes::AchCreditTransfer => {
                "ach_credit_transfer"
            }
            SubscriptionsResourcePaymentSettingsPaymentMethodTypes::AchDebit => "ach_debit",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypes::AcssDebit => "acss_debit",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypes::AuBecsDebit => "au_becs_debit",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypes::BacsDebit => "bacs_debit",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypes::Bancontact => "bancontact",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypes::Boleto => "boleto",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypes::Card => "card",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypes::Cashapp => "cashapp",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypes::CustomerBalance => {
                "customer_balance"
            }
            SubscriptionsResourcePaymentSettingsPaymentMethodTypes::Eps => "eps",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypes::Fpx => "fpx",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypes::Giropay => "giropay",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypes::Grabpay => "grabpay",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypes::Ideal => "ideal",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypes::Konbini => "konbini",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypes::Link => "link",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypes::P24 => "p24",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypes::Paynow => "paynow",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypes::Paypal => "paypal",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypes::Promptpay => "promptpay",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypes::SepaCreditTransfer => {
                "sepa_credit_transfer"
            }
            SubscriptionsResourcePaymentSettingsPaymentMethodTypes::SepaDebit => "sepa_debit",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypes::Sofort => "sofort",
            SubscriptionsResourcePaymentSettingsPaymentMethodTypes::UsBankAccount => {
                "us_bank_account"
            }
            SubscriptionsResourcePaymentSettingsPaymentMethodTypes::WechatPay => "wechat_pay",
        }
    }
}

impl AsRef<str> for SubscriptionsResourcePaymentSettingsPaymentMethodTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionsResourcePaymentSettingsPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for SubscriptionsResourcePaymentSettingsPaymentMethodTypes {
    fn default() -> Self {
        Self::AchCreditTransfer
    }
}

/// An enum representing the possible values of an `SubscriptionsResourcePaymentSettings`'s `save_default_payment_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod {
    Off,
    OnSubscription,
}

impl SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod::Off => "off",
            SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod::OnSubscription => {
                "on_subscription"
            }
        }
    }
}

impl AsRef<str> for SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod {
    fn default() -> Self {
        Self::Off
    }
}

/// An enum representing the possible values of an `UpdateSubscriptionAutomaticTaxLiability`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionAutomaticTaxLiabilityType {
    Account,
    #[serde(rename = "self")]
    Self_,
}

impl UpdateSubscriptionAutomaticTaxLiabilityType {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSubscriptionAutomaticTaxLiabilityType::Account => "account",
            UpdateSubscriptionAutomaticTaxLiabilityType::Self_ => "self",
        }
    }
}

impl AsRef<str> for UpdateSubscriptionAutomaticTaxLiabilityType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSubscriptionAutomaticTaxLiabilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateSubscriptionAutomaticTaxLiabilityType {
    fn default() -> Self {
        Self::Account
    }
}

/// An enum representing the possible values of an `UpdateSubscriptionCancellationDetails`'s `feedback` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionCancellationDetailsFeedback {
    CustomerService,
    LowQuality,
    MissingFeatures,
    Other,
    SwitchedService,
    TooComplex,
    TooExpensive,
    Unused,
}

impl UpdateSubscriptionCancellationDetailsFeedback {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSubscriptionCancellationDetailsFeedback::CustomerService => "customer_service",
            UpdateSubscriptionCancellationDetailsFeedback::LowQuality => "low_quality",
            UpdateSubscriptionCancellationDetailsFeedback::MissingFeatures => "missing_features",
            UpdateSubscriptionCancellationDetailsFeedback::Other => "other",
            UpdateSubscriptionCancellationDetailsFeedback::SwitchedService => "switched_service",
            UpdateSubscriptionCancellationDetailsFeedback::TooComplex => "too_complex",
            UpdateSubscriptionCancellationDetailsFeedback::TooExpensive => "too_expensive",
            UpdateSubscriptionCancellationDetailsFeedback::Unused => "unused",
        }
    }
}

impl AsRef<str> for UpdateSubscriptionCancellationDetailsFeedback {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSubscriptionCancellationDetailsFeedback {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateSubscriptionCancellationDetailsFeedback {
    fn default() -> Self {
        Self::CustomerService
    }
}

/// An enum representing the possible values of an `UpdateSubscriptionInvoiceSettingsIssuer`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionInvoiceSettingsIssuerType {
    Account,
    #[serde(rename = "self")]
    Self_,
}

impl UpdateSubscriptionInvoiceSettingsIssuerType {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSubscriptionInvoiceSettingsIssuerType::Account => "account",
            UpdateSubscriptionInvoiceSettingsIssuerType::Self_ => "self",
        }
    }
}

impl AsRef<str> for UpdateSubscriptionInvoiceSettingsIssuerType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSubscriptionInvoiceSettingsIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateSubscriptionInvoiceSettingsIssuerType {
    fn default() -> Self {
        Self::Account
    }
}

/// An enum representing the possible values of an `UpdateSubscriptionPauseCollection`'s `behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionPauseCollectionBehavior {
    KeepAsDraft,
    MarkUncollectible,
    Void,
}

impl UpdateSubscriptionPauseCollectionBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSubscriptionPauseCollectionBehavior::KeepAsDraft => "keep_as_draft",
            UpdateSubscriptionPauseCollectionBehavior::MarkUncollectible => "mark_uncollectible",
            UpdateSubscriptionPauseCollectionBehavior::Void => "void",
        }
    }
}

impl AsRef<str> for UpdateSubscriptionPauseCollectionBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSubscriptionPauseCollectionBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateSubscriptionPauseCollectionBehavior {
    fn default() -> Self {
        Self::KeepAsDraft
    }
}

/// An enum representing the possible values of an `UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions`'s `transaction_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    Business,
    Personal,
}

impl UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Business => "business",
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Personal => "personal",
        }
    }
}

impl AsRef<str>
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn default() -> Self {
        Self::Business
    }
}

/// An enum representing the possible values of an `UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebit`'s `verification_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::Automatic => "automatic",
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::Instant => "instant",
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::Microdeposits => "microdeposits",
        }
    }
}

impl AsRef<str>
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    fn default() -> Self {
        Self::Automatic
    }
}

/// An enum representing the possible values of an `UpdateSubscriptionPaymentSettingsPaymentMethodOptionsBancontact`'s `preferred_language` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    De,
    En,
    Fr,
    Nl,
}

impl UpdateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::De => "de",
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::En => "en",
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::Fr => "fr",
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::Nl => "nl",
        }
    }
}

impl AsRef<str>
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    fn default() -> Self {
        Self::De
    }
}

/// An enum representing the possible values of an `UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptions`'s `amount_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType {
    Fixed,
    Maximum,
}

impl UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType::Fixed => "fixed",
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType::Maximum => "maximum",
        }
    }
}

impl AsRef<str>
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardMandateOptionsAmountType
{
    fn default() -> Self {
        Self::Fixed
    }
}

/// An enum representing the possible values of an `UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCard`'s `network` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork {
    Amex,
    CartesBancaires,
    Diners,
    Discover,
    EftposAu,
    Interac,
    Jcb,
    Mastercard,
    Unionpay,
    Unknown,
    Visa,
}

impl UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork::Amex => "amex",
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork::CartesBancaires => {
                "cartes_bancaires"
            }
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork::Diners => "diners",
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork::Discover => {
                "discover"
            }
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork::EftposAu => {
                "eftpos_au"
            }
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork::Interac => "interac",
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork::Jcb => "jcb",
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork::Mastercard => {
                "mastercard"
            }
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork::Unionpay => {
                "unionpay"
            }
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork::Unknown => "unknown",
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork::Visa => "visa",
        }
    }
}

impl AsRef<str> for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardNetwork {
    fn default() -> Self {
        Self::Amex
    }
}

/// An enum representing the possible values of an `UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCard`'s `request_three_d_secure` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
    Challenge,
}

impl UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure::Any => "any",
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure::Automatic => "automatic",
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure::Challenge => "challenge",
        }
    }
}

impl AsRef<str> for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure
{
    fn default() -> Self {
        Self::Any
    }
}

/// An enum representing the possible values of an `UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections`'s `permissions` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}

impl UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions::Balances => "balances",
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions::Ownership => "ownership",
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions::PaymentMethod => "payment_method",
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions::Transactions => "transactions",
        }
    }
}

impl AsRef<str> for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    fn default() -> Self {
        Self::Balances
    }
}

/// An enum representing the possible values of an `UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections`'s `prefetch` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    Balances,
    Transactions,
}

impl
    UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch::Balances => "balances",
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch::Transactions => "transactions",
        }
    }
}

impl AsRef<str> for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch {
    fn default() -> Self {
        Self::Balances
    }
}

/// An enum representing the possible values of an `UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccount`'s `verification_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod::Automatic => "automatic",
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod::Instant => "instant",
            UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod::Microdeposits => "microdeposits",
        }
    }
}

impl AsRef<str>
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for UpdateSubscriptionPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod
{
    fn default() -> Self {
        Self::Automatic
    }
}

/// An enum representing the possible values of an `UpdateSubscriptionPaymentSettings`'s `payment_method_types` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionPaymentSettingsPaymentMethodTypes {
    AchCreditTransfer,
    AchDebit,
    AcssDebit,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Boleto,
    Card,
    Cashapp,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Konbini,
    Link,
    P24,
    Paynow,
    Paypal,
    Promptpay,
    SepaCreditTransfer,
    SepaDebit,
    Sofort,
    UsBankAccount,
    WechatPay,
}

impl UpdateSubscriptionPaymentSettingsPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSubscriptionPaymentSettingsPaymentMethodTypes::AchCreditTransfer => {
                "ach_credit_transfer"
            }
            UpdateSubscriptionPaymentSettingsPaymentMethodTypes::AchDebit => "ach_debit",
            UpdateSubscriptionPaymentSettingsPaymentMethodTypes::AcssDebit => "acss_debit",
            UpdateSubscriptionPaymentSettingsPaymentMethodTypes::AuBecsDebit => "au_becs_debit",
            UpdateSubscriptionPaymentSettingsPaymentMethodTypes::BacsDebit => "bacs_debit",
            UpdateSubscriptionPaymentSettingsPaymentMethodTypes::Bancontact => "bancontact",
            UpdateSubscriptionPaymentSettingsPaymentMethodTypes::Boleto => "boleto",
            UpdateSubscriptionPaymentSettingsPaymentMethodTypes::Card => "card",
            UpdateSubscriptionPaymentSettingsPaymentMethodTypes::Cashapp => "cashapp",
            UpdateSubscriptionPaymentSettingsPaymentMethodTypes::CustomerBalance => {
                "customer_balance"
            }
            UpdateSubscriptionPaymentSettingsPaymentMethodTypes::Eps => "eps",
            UpdateSubscriptionPaymentSettingsPaymentMethodTypes::Fpx => "fpx",
            UpdateSubscriptionPaymentSettingsPaymentMethodTypes::Giropay => "giropay",
            UpdateSubscriptionPaymentSettingsPaymentMethodTypes::Grabpay => "grabpay",
            UpdateSubscriptionPaymentSettingsPaymentMethodTypes::Ideal => "ideal",
            UpdateSubscriptionPaymentSettingsPaymentMethodTypes::Konbini => "konbini",
            UpdateSubscriptionPaymentSettingsPaymentMethodTypes::Link => "link",
            UpdateSubscriptionPaymentSettingsPaymentMethodTypes::P24 => "p24",
            UpdateSubscriptionPaymentSettingsPaymentMethodTypes::Paynow => "paynow",
            UpdateSubscriptionPaymentSettingsPaymentMethodTypes::Paypal => "paypal",
            UpdateSubscriptionPaymentSettingsPaymentMethodTypes::Promptpay => "promptpay",
            UpdateSubscriptionPaymentSettingsPaymentMethodTypes::SepaCreditTransfer => {
                "sepa_credit_transfer"
            }
            UpdateSubscriptionPaymentSettingsPaymentMethodTypes::SepaDebit => "sepa_debit",
            UpdateSubscriptionPaymentSettingsPaymentMethodTypes::Sofort => "sofort",
            UpdateSubscriptionPaymentSettingsPaymentMethodTypes::UsBankAccount => "us_bank_account",
            UpdateSubscriptionPaymentSettingsPaymentMethodTypes::WechatPay => "wechat_pay",
        }
    }
}

impl AsRef<str> for UpdateSubscriptionPaymentSettingsPaymentMethodTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSubscriptionPaymentSettingsPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateSubscriptionPaymentSettingsPaymentMethodTypes {
    fn default() -> Self {
        Self::AchCreditTransfer
    }
}

/// An enum representing the possible values of an `UpdateSubscriptionPaymentSettings`'s `save_default_payment_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionPaymentSettingsSaveDefaultPaymentMethod {
    Off,
    OnSubscription,
}

impl UpdateSubscriptionPaymentSettingsSaveDefaultPaymentMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSubscriptionPaymentSettingsSaveDefaultPaymentMethod::Off => "off",
            UpdateSubscriptionPaymentSettingsSaveDefaultPaymentMethod::OnSubscription => {
                "on_subscription"
            }
        }
    }
}

impl AsRef<str> for UpdateSubscriptionPaymentSettingsSaveDefaultPaymentMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSubscriptionPaymentSettingsSaveDefaultPaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateSubscriptionPaymentSettingsSaveDefaultPaymentMethod {
    fn default() -> Self {
        Self::Off
    }
}

/// An enum representing the possible values of an `UpdateSubscriptionTrialSettingsEndBehavior`'s `missing_payment_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod {
    Cancel,
    CreateInvoice,
    Pause,
}

impl UpdateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod::Cancel => "cancel",
            UpdateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod::CreateInvoice => {
                "create_invoice"
            }
            UpdateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod::Pause => "pause",
        }
    }
}

impl AsRef<str> for UpdateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdateSubscriptionTrialSettingsEndBehaviorMissingPaymentMethod {
    fn default() -> Self {
        Self::Cancel
    }
}
