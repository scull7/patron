
use error;
use hyper;
use std;
use types;
use url;

use std::str::FromStr;


pub struct Request<'a> {
  method: types::Method,
  url: url::Url<'a>,
  headers: hyper::header::Headers,
  body: Option<String>,
}


impl<'a> Request<'a> {
  pub fn new() -> Request<'a> {
    Request {
      method: types::Method::Get,
      url: url::Url::new(),
      headers: hyper::header::Headers::new(),
      body: None,
    }
  }


  pub fn from_url<S>(url: S) -> std::result::Result<Request<'a>, error::Error>
    where S: Into<String>
  {
    let mut req = Request::new();
    try!(req.set_url(url));

    Ok(req)

  }


  fn set_url<S>(&mut self, url: S) -> std::result::Result<(), error::Error>
    where S: Into<String>
  {
    self.url = try!(url::Url::from_str(&url.into()));

    Ok(())
  }


  pub fn set_method(&mut self, method: types::Method) -> &mut Request<'a> {
    self.method = method;
    self
  }


  pub fn set_headers(&mut self,
                     headers: hyper::header::Headers)
                     -> &mut Request<'a> {
    self.headers = headers;
    self
  }


  pub fn set_body<S>(&mut self, body: S) -> &mut Request<'a>
    where S: Into<String>
  {
    self.body = Some(body.into());
    self
  }



  pub fn add_path<S>(&mut self, path: S) -> &mut Request<'a>
    where S: Into<String>
  {
    self.url.add_path(path.into());
    self
  }


  pub fn add_query_param<K, S>(&mut self, key: K, val: S)
    where K: Into<String>,
          S: Into<std::borrow::Cow<'a, str>>
  {
    self.url.add_query_param(key, val);
  }
}
