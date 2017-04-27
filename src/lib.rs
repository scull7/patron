extern crate hyper;
extern crate hyper_rustls;
extern crate serde;
extern crate serde_json;
extern crate url as lib_url;

#[cfg(test)]
#[macro_use]
extern crate serde_derive;



mod builder;
pub mod client;
pub mod error;
mod request;
mod response;
mod types;
pub mod url;


pub fn from_str<'a, S>(url: S) -> builder::Builder<'a>
  where S: Into<std::borrow::Cow<'a, str>>
{

  let url = builder::UrlType::UrlString(url.into());
  builder::Builder::new(url)
}


pub fn from_url<'a>(url: url::Url<'a>) -> builder::Builder<'a> {
  let url = builder::UrlType::UrlObject(url);
  builder::Builder::new(url)
}
