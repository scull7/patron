use request;
use types;


pub struct Client<'a> {
  request: request::Request<'a>,
}


impl<'a> Client<'a> {
  pub fn new(req: request::Request<'a>) -> Client<'a> {
    Client { request: req }

  }


  pub fn request<S>(&self, method: types::Method, path: S) -> request::Request
    where S: Into<String>
  {
    let mut req = self.request.clone();
    req.set_method(method).add_path(path.into());

    req
  }


  pub fn post<S>(&self, path: S) -> request::Request
    where S: Into<String>
  {
    self.request(types::Method::Post, path)
  }


  pub fn get<S>(&self, path: S) -> request::Request
    where S: Into<String>
  {
    self.request(types::Method::Get, path)
  }
}
