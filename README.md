# patron
A wrapper around the hyper.rs library to allow for targeted clients to
specific remote APIs. This library should be useful on it's own or as a
building block for specific remote API wrappers.

## Design
I think that the design should move towards the [Fetch API](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API)  

## Notes
* Should the client be passed into the send method?
* I would like to only have one representation of a request configuration instead of the current 2.
* A more functional style design?
* I like how the fetch API returns a response object with the ability to retrieve the `Body` in 
  different formats. How can this be presented along with a nice `Result` based interface?
  ```rust
  client.get('/foo')
    .send()
    .json()
  ```
  `send()` will return a `Response` object which you can call `.json()`.  The `.json()` method will return
  a `Result<serde_json::Value, Error>`

## The Interface

### Exports
We want to give out an owned `std::sync::Arc` wrapped client so that the user can
utilize the client between threads if ncessary.

```rust
pub type Client = std::sync::Arc<patron::Client>;

```

### Creating an HTTPs GitHub Api client with an OAuth2 token.
```rust
use patron;

let client: patron::Client = try!(
  patron::from_url("https://api.github.com")
  .set_oauth_token("0b79bab50daca910b000d4f1a2b675d604257e42")
  .build()
);
```

### Creating an HTTPs GitHub Api client using query based tokens.
```rust
use patron;

let client: patron::Client = try!(
  patron::from_url("https://api.github.com")
  .add_query_param("client_id", "ABCDEFGHIKLMNOP")
  .add_query_param("client_secret", "QRSTUVWXYZABCDE")
  .build()
);
```

### Creating an HTTP client for CouchDB.
```rust
use patron;

let url: patron::Url = try!(
  patron::Url::new()
  .set_scheme(patron::Scheme::Http)
  .set_host('localhost')
  .set_port(5984)
  .build()
);

let client: patron::Client = try!(
  patron::from_url(url)
  .add_path("somedatabase")
  .basic_auth("anna", "secret")
  .build()
);
```

### Example usage for ArangoDB
```rust
use serde_json;
use patron;

#[derive(Debug, Deserialize)]
struct AuthRes {
  jwt: String,
  must_change_password: bool,
}

// Generally this would be built from some configuration object.
let url: patron::Url = try!(
  patron::Url::new()
  .set_scheme(patron::Scheme::Http)
  .set_host('localhost')
  .set_port(8529)
  .build()
);

let auth: AuthRes = try!(
  patron::Request::new(url)
  .post("/_open/auth")
  .add_body_param("username", "root")
  .add_body_param("password", "password")
  .send()
);

let mydb_client: patron::Client = try!(
  patron::from_url(url)
  .set_oauth_token(auth.jwt)
  .add_path("/_db/mydb/")
  .build()
);

#[derive(Debug, Deserialize)]
struct Version {
  version: String,
  server: String,
}

let version: Version = try!(client.get("/_api/version").send());

#[derive(Debug, Deserialize, Serialize)]
struct NewDoc {
  #[serde(rename = "_key")]
  key: String,
  #[serde(rename = "_id")]
  id: String,
  #[serde(rename = "_rev")]
  revision: String,
}

let newdoc: NewDoc = try!(
  client.post("/_api/demo")
  .add_body_param("message", serde_json::to_value("<replace_me>").unwrap())
  .add_query_param("waitForSync", "true")
  .send()
);

#[derive(Debug, Deserialize, Serialize)]
struct Document {
  #[serde(rename = "_key")]
  key: String,
  #[serde(rename = "_id")]
  id: String,
  #[serde(rename = "_rev")]
  revision: String,
  message: Option<String>
}
  
let mut hello: Document = try!(
  client.get("/_api/document/demo/")
  .add_path(newdoc.id)
  .send()
);

hello.message = Some("Hello, World!");

#[derive(Debug, Deserialize, Serialize)]
struct UpdateRes {
  #[serde(rename = "_key")]
  key: String,
  #[serde(rename = "_id")]
  id: String,
  #[serde(rename = "_rev")]
  revision: String,
  #[serde(rename = "_oldRev")]
  previous_revision: String,
}

let res: UpdateRes = try!(
  client.put("/api/document/demo")
  .add_path(hello.id)
  .set_body(hello)
  .send()
);
```
