
use request;
use std;
use types;


pub trait Client<'a> {
  fn get_client(&self) -> types::Client;


  fn get_base_url(&self) -> std::borrow::Cow<'a, str>;


  fn request<S>(&self, method: types::Method, path: S) -> request::Builder
    where S: Into<std::borrow::Cow<'a, str>>
  {
    let base_url: String = self.get_base_url().into();
    let path: String = path.into().into_owned();
    let mut req = request::Builder::new(self.get_client().clone(), method);

    req.set_base_url(base_url).add_path(path);

    req
  }


  fn post<S>(&self, path: S) -> request::Builder
    where S: Into<std::borrow::Cow<'a, str>>
  {
    self.request(types::Method::Post, path)
  }


  fn get<S>(&self, path: S) -> request::Builder
    where S: Into<std::borrow::Cow<'a, str>>
  {
    self.request(types::Method::Get, path)
  }
}
