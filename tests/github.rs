extern crate hyper;
extern crate patron;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

static CLIENT_ID: &'static str = "39515280290e3b9c67eb";
static CLIENT_SECRET: &'static str = "53742fb491d5c17170c60c3330553f226c0fc469";

#[derive(Debug, Deserialize)]
struct User {
  name: String,
  email: String,
  date: String,
}


#[derive(Debug, Deserialize)]
struct CommitInfo {
  comment_count: u64,
  message: String,
  author: User,
  committer: User,
}


#[derive(Debug, Deserialize)]
struct Commit {
  url: String,
  sha: String,
  commit: CommitInfo,
}

#[test]
fn get_commits() {

  let client: patron::Client = patron::from_str("https://api.github.com")
    .add_header(hyper::header::UserAgent("patron".to_string()))
    .add_query_param("client_id", CLIENT_ID)
    .add_query_param("client_secret", CLIENT_SECRET)
    .build()
    .unwrap();


  let commits: Vec<Commit> = client
    .get("/repos/scull7/patron/commits")
    .add_query_param("until", "2017-04-27T23:20:00Z")
    .send()
    .expect("Failed to retrieve commits")
    .deserialize()
    .unwrap();


  println!("Commits: {:?}", commits);

  assert_eq!(commits.len(), 13);
  assert!(
    commits
      .iter()
      .all(|c| c.commit.author.name == "Nathan Sculli")
  );

}
