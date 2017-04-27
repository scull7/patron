
use client;
use error;
use hyper;
use request;
use std;
use types;
use url;


use std::str::FromStr;


pub enum UrlType<'a> {
  UrlString(std::borrow::Cow<'a, str>),
  UrlObject(url::Url<'a>),
}


pub struct Builder<'a> {
  base_url: UrlType<'a>,
  headers: hyper::header::Headers,
  query_params: types::QueryParams<'a>,
  path_segments: types::PathSegments<'a>,
}


impl<'a> Builder<'a> {
  pub fn new(url: UrlType<'a>) -> Builder<'a> {

    let mut headers = hyper::header::Headers::new();
    headers.set(hyper::header::ContentType::json());

    Self {
      base_url: url,
      headers: headers,
      query_params: std::collections::HashMap::new(),
      path_segments: std::vec::Vec::new(),
    }
  }


  pub fn add_header<H>(&mut self, header: H) -> &mut Builder<'a>
    where H: hyper::header::Header + hyper::header::HeaderFormat
  {
    self.headers.set(header);
    self
  }


  pub fn add_query_param<K, S>(&mut self, key: K, val: S) -> &mut Builder<'a>
    where K: Into<String>,
          S: Into<std::borrow::Cow<'a, str>>
  {
    self.query_params.insert(key.into(), val.into());
    self
  }


  pub fn add_path<S>(&mut self, path: S) -> &mut Builder<'a>
    where S: Into<std::borrow::Cow<'a, str>>
  {
    self.path_segments.push(path.into());
    self
  }


  pub fn basic_auth<U, P>(&mut self,
                          user: U,
                          pass: Option<P>)
                          -> &mut Builder<'a>
    where U: Into<std::borrow::Cow<'a, str>>,
          P: Into<std::borrow::Cow<'a, str>>
  {
    let auth = hyper::header::Basic {
      username: user.into().into_owned(),
      password: pass.map(|s| s.into().into_owned()),
    };

    self.add_header(hyper::header::Authorization(auth))
  }


  pub fn set_bearer_token<T>(&mut self, token: T) -> &mut Builder<'a>
    where T: Into<std::borrow::Cow<'a, str>>
  {
    let auth = hyper::header::Bearer { token: token.into().into_owned() };

    self.add_header(hyper::header::Authorization(auth))
  }


  pub fn build(self) -> std::result::Result<client::Client<'a>, error::Error> {

    let mut url = match self.base_url {
      UrlType::UrlString(ref url) => try!(url::Url::from_str(url)),
      UrlType::UrlObject(url) => url,
    };
    url.add_query_params(&self.query_params);
    url.add_path_segments(&self.path_segments);

    let mut req = request::Request::new();
    req.set_url(url).set_headers(self.headers);

    Ok(client::Client::new(req))
  }
}
