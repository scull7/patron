
use error;
use request;
use std;
use types;


type Result<'a> = std::result::Result<request::Request<'a>, error::Error>;


pub trait Client<'a> {
  fn get_client(&self) -> types::Client;


  fn get_base_url(&self) -> std::borrow::Cow<'a, str>;


  fn request<S>(&self, method: types::Method, path: S) -> Result<'a>
    where S: Into<String>
  {
    let base_url: String = self.get_base_url().into();
    let mut req = try!(request::Request::from_url(base_url));

    req.set_method(method).add_path(path.into());

    Ok(req)
  }


  fn post<S>(&self, path: S) -> Result<'a>
    where S: Into<String>
  {
    self.request(types::Method::Post, path)
  }


  fn get<S>(&self, path: S) -> Result<'a>
    where S: Into<String>
  {
    self.request(types::Method::Get, path)
  }
}
