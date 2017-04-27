
use hyper;
use response;
use serde_json;
use std;


#[derive(Debug)]
pub enum Error {
  HttpNetwork(hyper::Error),
  JsonParse(serde_json::Error),
  InvalidUrl(UrlError),
  InvalidMethod(String),
  InvalidScheme(String),
  CallFailure(response::Response),
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub enum UrlError {
  Invalid(String),
  ParseError(hyper::error::ParseError),
}


impl std::convert::From<hyper::error::ParseError> for Error {
  fn from(err: hyper::error::ParseError) -> Error {
    Error::InvalidUrl(UrlError::ParseError(err))
  }
}


impl std::convert::From<hyper::Url> for Error {
  fn from(url: hyper::Url) -> Error {
    Error::InvalidUrl(UrlError::Invalid(url.to_string()))
  }
}


impl std::convert::From<hyper::Error> for Error {
  fn from(err: hyper::Error) -> Error {
    Error::HttpNetwork(err)
  }
}


impl std::convert::From<serde_json::Error> for Error {
  fn from(err: serde_json::Error) -> Error {
    Error::JsonParse(err)
  }
}


impl std::convert::From<UrlError> for Error {
  fn from(err: UrlError) -> Error {
    Error::InvalidUrl(err)
  }
}


impl std::convert::From<response::Response> for Error {
  fn from(err: response::Response) -> Error {
    Error::CallFailure(err)
  }
}
