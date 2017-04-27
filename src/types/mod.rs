
use std;


mod method;
mod scheme;


pub type QueryParams<'a> = std::collections::HashMap<String,
                                                     std::borrow::Cow<'a, str>>;


pub type PathSegments<'a> = std::vec::Vec<std::borrow::Cow<'a, str>>;


pub use self::method::Method;


pub use self::scheme::Scheme;
