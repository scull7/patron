
extern crate hyper;
extern crate patron;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Debug, Deserialize)]
struct Person {
  name: String,
  birth_year: String,
  eye_color: String,
  height: String,
  mass: String,
}


#[test]
fn https_get_test() {

  let client = patron::from_str("https://swapi.co/api/")
    .build()
    .unwrap();
  // Should get "Han Solo"
  let response = client.get("/people/14").send().unwrap();
  let han_solo: Person = serde_json::from_value(response.json().unwrap())
    .unwrap();

  assert_eq!(han_solo.birth_year, "29BBY");
  assert_eq!(han_solo.eye_color, "brown");
  assert_eq!(han_solo.height, "180");
  assert_eq!(han_solo.mass, "80");

}
