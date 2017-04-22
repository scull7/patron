
use error;
use hyper;
use serde_json;
use std;


mod method;
mod scheme;


pub type Client = std::sync::Arc<hyper::client::Client>;


pub type QueryParams<'a> = std::collections::HashMap<&'a str,
                                                     std::borrow::Cow<'a, str>>;


pub type PathSegments<'a> = std::vec::Vec<std::borrow::Cow<'a, str>>;


pub type Response = std::result::Result<serde_json::Value, error::Error>;


pub use self::method::Method;


pub use self::scheme::Scheme;


pub enum Body {
  Json(serde_json::Map<String, serde_json::Value>),
}
