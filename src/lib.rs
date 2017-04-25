extern crate hyper;
extern crate hyper_rustls;
extern crate serde;
extern crate serde_json;
extern crate url as lib_url;

#[cfg(test)]
#[macro_use]
extern crate serde_derive;



mod error;
mod builder;
mod client;
mod request;
mod url;
pub mod types;



#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {}
}
