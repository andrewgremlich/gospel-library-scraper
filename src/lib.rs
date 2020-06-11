use std::fs::File;
use std::io::prelude::*;

use mkdirp::mkdirp;
use scraper::Html;

const CHURCH_ROOT: &str = "https://www.churchofjesuschrist.org";

#[derive(Debug)]
pub struct UrlReference {
  pub file: String,
  pub dir: String,
  pub original: String,
  pub no_params: String,
}

pub async fn navigate(url: &str) -> Result<Html, Box<dyn std::error::Error>> {
  let navigate_url: String = format!("{}{}", CHURCH_ROOT, url);
  let resp = reqwest::get(&navigate_url).await?.text().await?;

  let document = Html::parse_document(&resp);

  return Ok(document);
}

pub fn write_dir(dir: &str) {
  match mkdirp(dir) {
    Err(e) => println!("{:?}", e),
    _ => (),
  };
}

pub fn write_index_file(file_name: &str, contents: &str) -> std::io::Result<String> {
  let mut file = File::create(file_name)?;
  file.write_all(contents.as_bytes())?;
  Ok(format!("{} index file has been written", file_name))
}
