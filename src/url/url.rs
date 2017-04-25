
use error;
use hyper;
use std;
use types;


pub struct Url<'a> {
  scheme: types::Scheme,
  host: std::borrow::Cow<'a, str>,
  port: u16,
  path_segments: types::PathSegments<'a>,
  query_params: types::QueryParams<'a>,
}


impl<'a> Url<'a> {
  pub fn new() -> Url<'a> {
    Url {
      scheme: types::Scheme::Http,
      host: std::borrow::Cow::from("localhost"),
      port: 80,
      path_segments: std::vec::Vec::new(),
      query_params: std::collections::HashMap::new(),
    }
  }


  pub fn to_url(&self) -> hyper::Url {
    let mut url = hyper::Url::parse("http://localhost").unwrap();
    url.set_scheme(&self.scheme.to_string());
    url.set_host(Some(&*self.host));
    url.set_port(Some(self.port));

    append_segments(&mut url, &self.path_segments);
    append_query_pairs(&mut url, &self.query_params);

    url
  }


  pub fn set_scheme(&mut self, scheme: types::Scheme) {
    self.scheme = scheme;
  }


  pub fn set_host<S>(&mut self, host: S)
    where S: Into<std::borrow::Cow<'a, str>>
  {
    self.host = host.into();
  }


  pub fn set_port(&mut self, port: u16) {
    self.port = port;
  }


  pub fn add_path<S>(&mut self, path: S)
    where S: Into<std::borrow::Cow<'a, str>>
  {
    self.path_segments.push(path.into());
  }


  pub fn add_query_param<K, V>(&mut self, key: K, val: V)
    where K: Into<String>,
          V: Into<std::borrow::Cow<'a, str>>
  {
    self.query_params.insert(key.into(), val.into());
  }
}


impl<'a> std::str::FromStr for Url<'a> {
  type Err = error::Error;


  fn from_str(s: &str) -> std::result::Result<self::Url<'a>, Self::Err> {
    let url = try!(hyper::Url::parse(s));

    Ok(Url {
         scheme: try!(types::Scheme::from_str(url.scheme())),
         host: host_from_url(&url),
         port: url.port().unwrap_or(80),
         path_segments: path_segments_from_url(&url),
         query_params: query_params_from_url(&url),
       })

  }
}


impl<'a> std::fmt::Display for Url<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    f.write_str(self.to_url().as_str())
  }
}


fn append_segments(url: &mut hyper::Url, segments: &types::PathSegments) {

  let mut url_segments =
    url.path_segments_mut().expect("unexpected invalid URL.");

  for segment in segments {
    url_segments.push(&*segment);
  }
}


fn append_query_pairs(url: &mut hyper::Url, params: &types::QueryParams) {

  let mut url_params = url.query_pairs_mut();

  for (key, value) in params {
    url_params.append_pair(key.as_ref(), &*value);
  }

}


fn host_from_url<'a>(url: &hyper::Url) -> std::borrow::Cow<'a, str> {
  let host = match url.host_str() {
    None => String::from("localhost"),
    Some(h) => h.to_owned(),
  };

  std::borrow::Cow::from(host)
}


fn path_segments_from_url<'a>(url: &hyper::Url) -> types::PathSegments<'a> {

  let mut paths = std::vec::Vec::new();

  match url.path_segments() {
    None => paths,
    Some(segments) => {

      for segment in segments {
        paths.push(std::borrow::Cow::from(segment.to_string()));
      }

      paths
    }
  }

}


fn query_params_from_url<'a>(url: &hyper::Url) -> types::QueryParams<'a> {

  let mut params: types::QueryParams<'a> = std::collections::HashMap::new();

  for (key, value) in url.query_pairs().into_owned() {
    params.insert(key, std::borrow::Cow::from(value));
  }

  params

}
