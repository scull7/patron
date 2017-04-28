use error;
use hyper;
use hyper_rustls;
use response;
use std;
use types;
use url;


#[derive(Debug, Clone)]
pub struct Request<'a> {
  client: std::sync::Arc<hyper::Client>,
  method: types::Method,
  url: url::Url<'a>,
  headers: hyper::header::Headers,
  body: String,
}


impl<'a> Request<'a> {
  pub fn new() -> Request<'a> {
    Request {
      client: std::sync::Arc::new(hyper::Client::new()),
      method: types::Method::Get,
      url: url::Url::new(),
      headers: hyper::header::Headers::new(),
      body: String::new(),
    }
  }


  pub fn get_scheme(&self) -> types::Scheme {
    self.url.scheme.clone()
  }


  pub fn set_url(&mut self, url: url::Url<'a>) -> &mut Request<'a> {
    let client = match url.scheme {
      types::Scheme::Http => hyper::Client::new(),
      types::Scheme::Https => {
        let tls = hyper_rustls::TlsClient::new();
        let conn = hyper::net::HttpsConnector::new(tls);

        hyper::Client::with_connector(conn)
      }
    };

    self.client = std::sync::Arc::new(client);
    self.url = url;

    self
  }


  pub fn set_method(&mut self, method: types::Method) -> &mut Request<'a> {
    self.method = method;
    self
  }


  pub fn set_headers(
    &mut self,
    headers: hyper::header::Headers,
  ) -> &mut Request<'a> {
    self.headers = headers;
    self
  }


  pub fn set_body<S>(&mut self, body: S) -> &mut Request<'a>
    where S: Into<String>
  {
    self.body = body.into();
    self
  }


  pub fn add_header<H>(&mut self, header: H) -> &mut Request<'a>
    where H: hyper::header::Header + hyper::header::HeaderFormat
  {
    self.headers.set(header);
    self
  }



  pub fn add_path<S>(&mut self, path: S) -> &mut Request<'a>
    where S: Into<String>
  {
    self.url.add_path(path.into());
    self
  }


  pub fn add_query_param<K, S>(&mut self, key: K, val: S) -> &mut Request<'a>
    where K: Into<String>,
          S: Into<std::borrow::Cow<'a, str>>
  {
    self.url.add_query_param(key, val);
    self
  }


  pub fn basic_auth<U, P>(&mut self, u: U, p: Option<P>) -> &mut Request<'a>
    where U: Into<String>,
          P: Into<String>
  {
    let auth = hyper::header::Basic {
      username: u.into(),
      password: p.map(|s| s.into()),
    };
    self.add_header(hyper::header::Authorization(auth))
  }


  pub fn set_bearer_token<T>(&mut self, token: T) -> &mut Request<'a>
    where T: Into<String>
  {
    let auth = hyper::header::Bearer { token: token.into() };

    self.add_header(hyper::header::Authorization(auth))
  }


  pub fn send(&self) -> std::result::Result<response::Response, error::Error> {

    let url = try!(self.url.to_url());

    let request = match self.method {
      types::Method::Get => self.client.get(url),
      types::Method::Post => self.client.post(url),
      types::Method::Put => self.client.put(url),
    };

    request
      .headers(self.headers.clone())
      .body(&self.body)
      .send()
      .map_err(error::Error::from)
      .map(response::Response::from)
      .and_then(
        |res| match res.ok {
          true => Ok(res),
          false => Err(error::Error::from(res)),
        },
      )

  }
}
