use std::collections::HashMap;
use std::fmt::Display;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::resources::{ApiVersion, Currency};
use crate::{
    AccountId, ApplicationId,
};

#[derive(Clone, Default)]
pub struct AppInfo {
    pub name: String,
    pub url: Option<String>,
    pub version: Option<String>,
}

impl Display for AppInfo {
    /// Formats a plugin's 'App Info' that can be added to the end of a User-Agent string.
    ///
    /// This formatting matches that of other libraries, and if changed then it should be changed everywhere.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.version, &self.url) {
            (Some(a), Some(b)) => write!(f, "{}/{} ({})", &self.name, a, b),
            (Some(a), None) => write!(f, "{}/{}", &self.name, a),
            (None, Some(b)) => write!(f, "{} ({})", &self.name, b),
            _ => write!(f, "{}", self.name),
        }
    }
}

#[derive(Clone)]
pub struct Headers {
    pub stripe_version: ApiVersion,
    pub user_agent: String,

    pub client_id: Option<ApplicationId>,
    pub stripe_account: Option<AccountId>,
}

impl Headers {
    pub fn to_array(&self) -> [(&str, Option<&str>); 4] {
        [
            ("Client-Id", self.client_id.as_deref()),
            ("Stripe-Account", self.stripe_account.as_deref()),
            ("Stripe-Version", Some(self.stripe_version.as_str())),
            ("User-Agent", Some(&self.user_agent)),
        ]
    }
}

/// Implemented by types which represent stripe objects.
pub trait Object {
    /// The canonical id type for this object.
    type Id;
    /// The id of the object.
    fn id(&self) -> Self::Id;
    /// The object's type, typically represented in wire format as the `object` property.
    fn object(&self) -> &'static str;
}

/// A deleted object.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Deleted<T> {
    /// Unique identifier for the object.
    pub id: T,
    /// Always true for a deleted object.
    pub deleted: bool,
}

/// The `Expand` struct is used to serialize `expand` arguments in retrieve and list apis.
#[doc(hidden)]
#[derive(Serialize)]
pub struct Expand<'a> {
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],
}

impl Expand<'_> {
    pub(crate) fn is_empty(expand: &[&str]) -> bool {
        expand.is_empty()
    }
}

/// An id or object.
///
/// By default stripe will return an id for most fields, but if more detail is
/// necessary the `expand` parameter can be provided to ask for the id to be
/// loaded as an object instead:
///
/// ```rust,ignore
/// Charge::retrieve(&client, &charge_id, &["invoice.customer"])
/// ```
///
/// For more details see <https://stripe.com/docs/api/expanding_objects>.
#[derive(Clone, Debug, Serialize, Deserialize)] // TODO: Implement deserialize by hand for better error messages
#[serde(untagged)]
pub enum Expandable<T: Object> {
    Id(T::Id),
    Object(Box<T>),
}

impl<T> Expandable<T>
where
    T: Object,
    T::Id: Clone + Default,
{
    pub fn id(&self) -> T::Id {
        match self {
            Expandable::Id(id) => id.clone(),
            Expandable::Object(obj) => obj.id(),
        }
    }
}

impl<T: Object> Default for Expandable<T>
where
    T::Id: Default,
{
    fn default() -> Self {
        Expandable::Id(Default::default())
    }
}

impl<T: Object> Expandable<T> {
    pub fn is_object(&self) -> bool {
        match self {
            Expandable::Id(_) => false,
            Expandable::Object(_) => true,
        }
    }

    pub fn as_object(&self) -> Option<&T> {
        match self {
            Expandable::Id(_) => None,
            Expandable::Object(obj) => Some(obj),
        }
    }

    pub fn into_object(self) -> Option<T> {
        match self {
            Expandable::Id(_) => None,
            Expandable::Object(obj) => Some(*obj),
        }
    }
}

/// Implemented by types which support cursor-based pagination,
/// typically with an id, allowing them to be fetched using a `List`
/// returned by the corresponding "list" api request.
pub trait Paginate {
    type Cursor: AsCursor;
    fn cursor(&self) -> Self::Cursor;
}

pub trait AsCursor: AsRef<str> {}

impl<'a> AsCursor for &'a str {}
impl AsCursor for String {}

impl<T> Paginate for T
where
    T: Object,
    T::Id: AsCursor,
{
    type Cursor = T::Id;
    fn cursor(&self) -> Self::Cursor {
        self.id()
    }
}

pub trait Paginable {
    type O: Object + Send;
    fn set_last(&mut self, item: Self::O);
}

pub trait PaginableList {
    type O: Paginate + DeserializeOwned + Send + Sync + 'static + Clone + std::fmt::Debug;
    fn new(data: Vec<Self::O>, url: String, has_more: bool, total_count: Option<u64>) -> Self;
    fn get_data_mut(&mut self) -> &mut Vec<Self::O>;
    fn get_data(&self) -> &Vec<Self::O>;
    fn get_url(&self) -> String;
    fn get_total_count(&self) -> Option<u64>;
    fn has_more(&self) -> bool;
}

/// A single page of a cursor-paginated list of a search object.
///
/// For more details, see <https://stripe.com/docs/api/pagination/search>
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchList<T> {
    pub object: String,
    pub url: String,
    pub has_more: bool,
    pub data: Vec<T>,
    pub next_page: Option<String>,
    pub total_count: Option<u64>,
}

impl<T> Default for SearchList<T> {
    fn default() -> Self {
        SearchList {
            object: String::new(),
            data: Vec::new(),
            has_more: false,
            total_count: None,
            url: String::new(),
            next_page: None,
        }
    }
}

impl<T: Paginate + DeserializeOwned + Send + Sync + 'static + Clone + std::fmt::Debug> PaginableList
    for SearchList<T>
{
    type O = T;

    fn new(
        data: Vec<Self::O>,
        url: String,
        has_more: bool,
        total_count: Option<u64>,
    ) -> SearchList<T> {
        Self { object: "".to_string(), url, has_more, data, next_page: None, total_count }
    }

    fn get_data_mut(&mut self) -> &mut Vec<Self::O> {
        &mut self.data
    }

    fn get_data(&self) -> &Vec<Self::O> {
        &self.data
    }
    fn get_url(&self) -> String {
        self.url.clone()
    }
    fn get_total_count(&self) -> Option<u64> {
        self.total_count
    }
    fn has_more(&self) -> bool {
        self.has_more
    }
}

impl<T: Paginate + DeserializeOwned + Send + Sync + 'static + Clone + std::fmt::Debug> PaginableList
    for List<T>
{
    type O = T;

    fn new(data: Vec<Self::O>, url: String, has_more: bool, total_count: Option<u64>) -> List<T> {
        Self { url, has_more, data, total_count }
    }

    fn get_data_mut(&mut self) -> &mut Vec<Self::O> {
        &mut self.data
    }

    fn get_data(&self) -> &Vec<Self::O> {
        &self.data
    }

    fn get_url(&self) -> String {
        self.url.clone()
    }
    fn get_total_count(&self) -> Option<u64> {
        self.total_count
    }
    fn has_more(&self) -> bool {
        self.has_more
    }
}

impl<T> SearchList<T> {
    pub fn paginate<P>(self, params: P) -> ListPaginator<SearchList<T>, P> {
        ListPaginator { page: self, params }
    }
}

/// A single page of a cursor-paginated list of an object.
///
/// For more details, see <https://stripe.com/docs/api/pagination>
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct List<T> {
    pub data: Vec<T>,
    pub has_more: bool,
    pub total_count: Option<u64>,
    pub url: String,
}

impl<T> Default for List<T> {
    fn default() -> Self {
        List { data: Vec::new(), has_more: false, total_count: None, url: String::new() }
    }
}

impl<T> List<T> {
    pub fn paginate<P>(self, params: P) -> ListPaginator<List<T>, P> {
        ListPaginator { page: self, params }
    }
}

#[derive(Debug)]
pub struct ListPaginator<T, P> {
    pub page: T,
    pub params: P,
}


pub type CurrencyMap<V> = HashMap<Currency, V>;
pub type Metadata = HashMap<String, String>;
pub type Timestamp = i64;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct RangeBounds<T> {
    pub gt: Option<T>,
    pub gte: Option<T>,
    pub lt: Option<T>,
    pub lte: Option<T>,
}

impl<T> Default for RangeBounds<T> {
    fn default() -> Self {
        RangeBounds { gt: None, gte: None, lt: None, lte: None }
    }
}

/// A set of generic request parameters that can be used on
/// list endpoints to filter their results by some timestamp.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum RangeQuery<T> {
    Exact(T),
    Bounds(RangeBounds<T>),
}

impl<T> RangeQuery<T> {
    /// Filter results to exactly match a given value
    pub fn eq(value: T) -> RangeQuery<T> {
        RangeQuery::Exact(value)
    }

    /// Filter results to be after a given value
    pub fn gt(value: T) -> RangeQuery<T> {
        RangeQuery::Bounds(RangeBounds { gt: Some(value), ..Default::default() })
    }

    /// Filter results to be after or equal to a given value
    pub fn gte(value: T) -> RangeQuery<T> {
        RangeQuery::Bounds(RangeBounds { gte: Some(value), ..Default::default() })
    }

    /// Filter results to be before to a given value
    pub fn lt(value: T) -> RangeQuery<T> {
        RangeQuery::Bounds(RangeBounds { lt: Some(value), ..Default::default() })
    }

    /// Filter results to be before or equal to a given value
    pub fn lte(value: T) -> RangeQuery<T> {
        RangeQuery::Bounds(RangeBounds { lte: Some(value), ..Default::default() })
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum IdOrCreate<'a, T> {
    Id(&'a str),
    Create(&'a T),
}

// NOTE: Only intended to handle conversion from ASCII CamelCase to SnakeCase
//   This function is used to convert static Rust identifiers to snakecase
// TODO: pub(crate) fn
pub fn to_snakecase(camel: &str) -> String {
    let mut i = 0;
    let mut snake = String::new();
    let mut chars = camel.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch.is_uppercase() {
            if i > 0 && !chars.peek().unwrap_or(&'A').is_uppercase() {
                snake.push('_');
            }
            snake.push(ch.to_lowercase().next().unwrap_or(ch));
        } else {
            snake.push(ch);
        }
        i += 1;
    }

    snake
}

#[cfg(test)]
mod tests {
    #[test]
    fn to_snakecase() {
        use super::to_snakecase;

        assert_eq!(to_snakecase("snake_case").as_str(), "snake_case");
        assert_eq!(to_snakecase("CamelCase").as_str(), "camel_case");
        assert_eq!(to_snakecase("XMLHttpRequest").as_str(), "xml_http_request");
        assert_eq!(to_snakecase("UPPER").as_str(), "upper");
        assert_eq!(to_snakecase("lower").as_str(), "lower");
    }
}
