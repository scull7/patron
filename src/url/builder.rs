
use error;
use hyper;
use std;
use types;


pub struct Builder<'a> {
  scheme: types::Scheme,
  host: &'a str,
  port: u16,
  path_segments: types::PathSegments<'a>,
  query_params: types::QueryParams<'a>,
}


impl<'a> Builder<'a> {
  pub fn new() -> Builder<'a> {
    Builder {
      scheme: types::Scheme::Http,
      host: "localhost",
      port: 80,
      path_segments: std::vec::Vec::new(),
      query_params: std::collections::HashMap::new(),
    }
  }


  pub fn set_scheme(&mut self, scheme: types::Scheme) -> &mut Builder<'a> {
    self.scheme = scheme;
    self
  }


  pub fn set_host<S>(&mut self, host: S) -> &mut Builder<'a>
    where S: Into<std::borrow::Cow<'a, str>>
  {
    self.host = host.into();
    self
  }


  pub fn set_port(&mut self, port: u16) -> &mut Builder<'a> {
    self.port = port;
    self
  }


  pub fn add_path<S>(&mut self, path: S) -> &mut Builder<'a>
    where S: Into<std::borrow::Cow<'a, str>>
  {
    self.path_segments.push(path.into());
    self
  }


  pub fn add_query_param<K, V>(&mut self, key: K, val: V) -> &mut Builder<'a>
    where K: Into<std::borrow::Cow<'a, str>>,
          V: Into<std::borrow::Cow<'a, str>>
  {
    self.query_params.insert(key.into(), val.into());
    self
  }
}


impl<'a> std::str::FromStr for Builder<'a> {
  type Err = error::Error;


  fn from_str(s: &str) -> std::result::Result<self::Builder<'a>, Self::Err> {

    let url = try!(hyper::Url::parse(s));
    let scheme = try!(types::Scheme::from_url(url));


    Ok(Builder {
         scheme: scheme,
         host: "localhost",
         port: 80,
         path_segments: std::vec::Vec::new(),
         query_params: std::collections::HashMap::new(),
       })
  }
}
