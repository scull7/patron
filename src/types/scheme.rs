
use error;
use hyper;
use std;

use std::str::FromStr;


pub enum Scheme {
  Https,
  Http,
}


impl Scheme {
  pub fn from_url(url: hyper::Url)
                  -> std::result::Result<self::Scheme, error::Error> {
    self::Scheme::from_str(url.scheme())
  }
}


impl std::str::FromStr for Scheme {
  type Err = error::Error;


  fn from_str(s: &str) -> std::result::Result<self::Scheme, Self::Err> {
    Ok(match s.to_uppercase().as_str() {
         "HTTPS" => self::Scheme::Https,
         "HTTP" => self::Scheme::Http,
         invalid => {
           return Err(error::Error::InvalidScheme(invalid.to_string()))
         }
       })
  }
}


impl std::fmt::Display for Scheme {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    f.write_str(match *self {
                  self::Scheme::Https => "https://",
                  self::Scheme::Http => "http://",
                })
  }
}
