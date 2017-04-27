
use patron;

let client_id: &'static str = "39515280290e3b9c67eb"
let client_secret: &'static str = "53742fb491d5c17170c60c3330553f226c0fc469"


#[test]
fn get_commits() {

  let client: patron::Client = patron::from_url("https://api.github.com")
    .add_query_param("client_id", client_id)
    .add_query_param("client_secret", client_secret)
    .build()
    .unwrap();

}
