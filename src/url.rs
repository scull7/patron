
use error;
use hyper;
use lib_url;
use std;
use types;


#[derive(Debug, Clone)]
pub struct Url<'a> {
  pub scheme: types::Scheme,
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


  pub fn to_url(&self) -> std::result::Result<hyper::Url, error::Error> {
    hyper::Url::parse(&self.to_string()).map_err(error::Error::from)
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


  pub fn add_path_segments(&mut self, segments: &types::PathSegments) {
    for segment in segments {
      let path = std::borrow::Cow::from(segment.clone().into_owned());
      self.path_segments.push(path);
    }
  }


  pub fn add_query_params(&mut self, params: &types::QueryParams) {

    for (key, value) in params {
      let value = std::borrow::Cow::from(value.clone().into_owned());
      self.query_params.insert(key.clone(), value);
    }

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

    write!(f,
           "{scheme}{host}:{port}/{paths}?{query}",
           scheme = &self.scheme.to_string(),
           host = &*self.host,
           port = self.port,
           paths = join_paths(&self.path_segments),
           query = encode_query(&self.query_params))
  }
}


fn encode_query(params: &types::QueryParams) -> String {
  let mut serializer = lib_url::form_urlencoded::Serializer::new(String::new());

  for (key, value) in params {
    serializer.append_pair(&key, &*value);
  }

  serializer.finish()
}


fn join_paths(segments: &types::PathSegments) -> String {
  let s = String::new();
  segments
    .iter()
    .fold(s, |i, j| i.to_string() + "/" + &j.clone().into_owned())
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
