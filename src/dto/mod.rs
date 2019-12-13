#[derive(Serialize, Deserialize, Debug,)]
pub struct NewBookDto {
  pub title: String,
  pub author: String,
  #[serde(default)]
  pub is_published: bool,
}

#[derive(Serialize, Deserialize, Debug,)]
pub struct IdDto {
    id: i32
}