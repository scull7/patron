
use hyper;
use std;
use types;


pub struct Builder<'a> {
  client: types::Client,
  headers: hyper::header::Headers,
  base_url: Option<std::borrow::Cow<'a, str>>,
  query_params: types::QueryParams<'a>,
  path_segments: Vec<std::borrow::Cow<'a, str>>,
  body: Option<types::Body>,
}


impl<'a> Builder<'a> {
  pub fn new(client: types::Client, method: types::Method) -> Builder<'a> {
    let mut headers = hyper::header::Headers::new();
    headers.set(hyper::header::ContentType::json());

    Builder {
      client: client,
      headers: headers,
      base_url: None,
      query_params: std::collections::HashMap::new(),
      path_segments: std::vec::Vec::new(),
      body: None,
    }
  }


  pub fn set_base_url<S>(&mut self, url: S) -> &mut Builder<'a>
    where S: Into<std::borrow::Cow<'a, str>>
  {
    self.base_url = Some(url.into());
    self
  }

  // @TODO Implement a set path function.


  pub fn set_headers(&mut self,
                     headers: hyper::header::Headers)
                     -> &mut Builder<'a> {
    self.headers = headers;
    self
  }


  pub fn set_query_params(&mut self,
                          params: types::QueryParams<'a>)
                          -> &mut Builder<'a> {
    self.query_params = params;
    self
  }


  pub fn set_body(&mut self, body: types::Body) -> &mut Builder<'a> {
    self.body = Some(body);
    self
  }


  pub fn add_path<S>(&mut self, path: S) -> &mut Builder<'a>
    where S: Into<std::borrow::Cow<'a, str>>
  {
    self.path_segments.push(path.into());
    self
  }


  pub fn add_query_param<S>(&mut self, key: &'a str, val: S) -> &mut Builder<'a>
    where S: Into<std::borrow::Cow<'a, str>>
  {
    self.query_params.insert(key, val.into());
    self
  }
}
