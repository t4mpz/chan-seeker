use serde::{Deserialize, Serialize};
pub struct ThreadReply{
  pub reply: String,
  pub subject: String,
  pub image_response: String,
  pub is_op: bool,
  // pub datetime: String // maybe work with dates later
}

pub struct Thread{
  pub url: String,
  pub title: String,
  pub image_href: String, // i'll get some image rendering here, not on the terminal ofc
  pub reply: Vec<ThreadReply>
}

#[derive(Deserialize, Serialize)]
pub struct Conf{
  pub host: String,
  pub user: String,
  pub password: String,
  pub db: String
}
// this one is another major problem
// I'll rewrite it soon just let me get some sleep
