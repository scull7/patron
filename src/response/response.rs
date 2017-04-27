
use error;
use hyper;
use serde_json;
use std;
use super::types;


#[derive(Debug)]
pub struct Response {
  pub headers: hyper::header::Headers,
  pub ok: bool,
  pub redirected: bool,
  pub status: u16,
  pub status_text: String,
  pub kind: types::Types,
  pub url: String,
  res: hyper::client::response::Response,
}


impl Response {

  pub fn json(self) -> std::result::Result<serde_json::Value, error::Error> {
    serde_json::from_reader(self.res).map_err(error::Error::from)
  }
}



impl std::convert::From<hyper::client::response::Response> for Response {

  fn from(res: hyper::client::response::Response) -> Response {
    let hyper::http::RawStatus(status, status_text) = res.status_raw().clone();

    Response {
      headers: res.headers.clone(),
      ok: res.status.is_success(),
      redirected: false, // @TODO figure out how to get this answer.
      status: status,
      status_text: status_text.into_owned(),
      kind: types::Types::Basic,
      url: res.url.to_string(),
      res:res,
    }
  }
}
