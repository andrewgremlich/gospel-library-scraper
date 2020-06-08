use mkdirp::mkdirp;
use scraper::Html;
use std::fs::File;
use std::io::prelude::*;

const CHURCH_ROOT: &str = "https://www.churchofjesuschrist.org";

#[derive(Debug)]
pub struct UrlReference {
  pub file: String,
  pub dir: String,
  pub original: String,
  pub no_params: String,
}

pub fn urls_of_first_chapter(url: &str) -> UrlReference {
  let no_query_param: Vec<&str> = url.split("?").collect();

  let mut file_name_vector: Vec<&str> = no_query_param[0].split("/").collect();
  file_name_vector.pop();
  let dir_append: String = file_name_vector.join("/");

  let mut file_to_write: String = ".".to_owned();
  let mut dir_to_write: String = ".".to_owned();
  file_to_write.push_str(no_query_param[0]);
  file_to_write.push_str(".md");

  dir_to_write.push_str(&dir_append);

  let urls = UrlReference {
    file: String::from(file_to_write),
    dir: String::from(dir_to_write),
    no_params: String::from(no_query_param[0]),
    original: String::from(url),
  };

  return urls;
}

pub async fn navigate(url: &str) -> Result<Html, Box<dyn std::error::Error>> {
  let navigate_url: String = format!("{}{}", CHURCH_ROOT, url);
  let resp = reqwest::get(&navigate_url).await?.text().await?;

  let document = Html::parse_document(&resp);

  return Ok(document);
}

fn write_dir(dir: &str) {
  match mkdirp(dir) {
    Err(e) => println!("{:?}", e),
    _ => (),
  };
}

pub fn write_index_file(file_name: &str, contents: &str) -> std::io::Result<()> {
  let mut file = File::create(file_name)?;
  file.write_all(contents.as_bytes())?;
  Ok(())
}

pub fn write_section(file_name: &str, contents: &str) -> std::io::Result<()> {
  let mut file = File::create(file_name)?;
  file.write_all(contents.as_bytes())?;
  Ok(())
}
