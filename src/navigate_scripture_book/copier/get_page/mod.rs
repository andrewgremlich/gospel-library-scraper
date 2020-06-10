mod text_transform;

use scraper::{Html, Selector};
use gospellibraryscraper::{navigate};
use text_transform::transform_source_scripture_text;

#[derive(Debug)]
pub struct ScrapedPage {
  pub contents: String,
  pub summary: String,
}

pub async fn get_page(urls_of_first_chapter: &str) -> ScrapedPage {
  let section_of_book: Html = navigate(urls_of_first_chapter).await.unwrap();

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