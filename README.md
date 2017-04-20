# patron
A wrapper around the hyper.rs library to allow for targeted clients to specific remote APIs.

## The dream interface

### Exports
We want to give out an owned `std::sync::Arc` wrapped client so that the user can
utilize the client between threads if ncessary.

```rust
pub type Client = std::sync::Arc<patron::Client>;

```

### Creating an HTTPs client with an OAuth2 token.
```rust
use patron;

let client: patron::Client = try!(patron::from_url("https://api.github.com")
           .set_oauth_token("0b79bab50daca910b000d4f1a2b675d604257e42")
           .build());
```

### Creating an HTTPs client using query based tokens.
```rust
use patron;

let client: patron::Client = try!(patron::from_url("https://api.github.com")
            .add_query_param("client_id", "ABCDEFGHIKLMNOP")
            .add_query_param("client_secret", "QRSTUVWXYZABCDE")
            .build());
```

### Creating an HTTP client for CouchDB.
```rust
use patron;

let client: patron::Client = try!(patron::from_props(patron::HTTPS, "localhost", 5984)
            .add_path("somedatabase")
            .basic_auth("anna", "secret")
            .build());
```
