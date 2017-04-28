
#[derive(Debug, Clone)]
pub enum Types {
  // Normal, same origin response, with all headers exposed except
  // Set-Cookie` and `Set-Cookie2`.
  Basic,

  // Response was received from a valid cross-origin request.  Certain
  // headers and the body may be accessed.
  Cors,

  // Network error.  No useful information describing the error is
  // available.  The Response's status is `0`, headers are empty and
  // immutable.  This is the type for a Response obtained from
  // `Response.error()`
  Error,

  // Response for `no-cors` request to cross-origin resource.
  // Severely restricted
  Opaque,
}
