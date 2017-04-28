use error;
use std;


#[derive(Debug, Clone)]
pub enum Method {
  Get,
  Post,
  Put,
}


impl Method {}


impl std::str::FromStr for Method {
  type Err = error::Error;


  fn from_str(s: &str) -> std::result::Result<self::Method, Self::Err> {
    Ok(
      match s.to_uppercase().as_str() {
        "GET" => self::Method::Get,
        "POST" => self::Method::Post,
        "PUT" => self::Method::Put,
        invalid => return Err(error::Error::InvalidMethod(invalid.to_string())),
      },
    )

  }
}


impl std::fmt::Display for Method {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    f.write_str(
      match *self {
        self::Method::Get => "GET",
        self::Method::Post => "POST",
        self::Method::Put => "PUT",
      },
    )
  }
}
