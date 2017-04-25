

pub trait Body {
  //fn blob(&self) -> Blob;

  //fn formData(&self) -> FormData;

  fn json(&self) -> serde_json::Value;


  fn text(&self) -> String;
}
