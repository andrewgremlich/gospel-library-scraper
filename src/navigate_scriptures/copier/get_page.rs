mod text_transform;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use gospellibraryscraper::navigate;
use scraper::{Html, Selector};
use text_transform::transform_source_scripture_text;

#[derive(Debug)]
pub struct ScrapedPage {
  pub contents: String,
  pub summary: String,
}

#[derive(Debug)]
pub struct OrderedScrapedPage {
  pub order_number: u16,
  pub title: String,
  pub scraped_page: ScrapedPage,
}

impl OrderedScrapedPage {
  pub fn write_section(&self, file_name: &str) -> std::io::Result<String> {
    let mut file = File::create(file_name)?;
    file.write(b"---\n")?;
    file.write(format!("title: {}\n", &self.title).as_bytes())?;
    file.write(format!("description: {}\n", &self.scraped_page.summary).as_bytes())?;
    file.write(format!("order: {}\n", &self.order_number).as_bytes())?;
    file.write(b"---\n")?;
    file.write_all(&self.scraped_page.contents.as_bytes())?;
    return Ok(format!("wrote {}", file_name));
  }
}

#[derive(Debug)]
pub struct IndexPage {
  pub index_number: u16,
  pub title: String,
  pub url: String,
}

impl IndexPage {
  pub fn write_index_file(&self) -> std::io::Result<String> {
    let path_exist: bool = Path::new(&self.url).exists();

    if path_exist {
      return Ok(format!("already exists {}", &self.title));
    } else {
      let mut file = File::create(&self.url)?;

      file.write(b"---\n")?;
      file.write(format!("title: {}\n", &self.title).as_bytes())?;
      file.write(format!("order: {}\n", &self.index_number).as_bytes())?;
      file.write(b"---\n")?;
      file.write_all(b"Index page.\n")?;
      
      return Ok(format!("wrote {}", &self.url));
    }
  }
}

pub async fn get_page(original_url: &str) -> ScrapedPage {
  let section_of_book: Html = navigate(original_url).await.unwrap();

  let chapter_text_selector = Selector::parse(".renderFrame-1yHgQ .body-block").unwrap();
  let chapter_summary_selector = Selector::parse("#study_summary1").unwrap();

  let mut page: ScrapedPage = ScrapedPage {
    contents: String::from(""),
    summary: String::from(""),
  };

  if let Some(d) = section_of_book.select(&chapter_summary_selector).next() {
    let chapter_summary: &str = d.text().collect::<Vec<_>>()[0];
    page.summary = String::from(chapter_summary);
  }
  if let Some(d) = section_of_book.select(&chapter_text_selector).next() {
    let chapter_text: Vec<&str> = d.text().collect::<Vec<_>>();
    let transformed_text: String = transform_source_scripture_text(chapter_text);
    page.contents = transformed_text;
  }

  return page;
}