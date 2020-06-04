mod copier;

use gospellibraryscraper::navigate;
use scraper::{Html, Selector};

//async
async fn navigate_manifest(resp: &Html) {
  let contents_html: &Html = &resp;
  let contents_selector: Selector = Selector::parse(".manifest a").unwrap();

  let mut index_count: u8 = 1;

  for link in contents_html.select(&contents_selector) {
    match link.value().attr("href") {
      Some(url) => {
        copier::main(url, index_count).await;
        index_count = index_count + 1;
      }
      _ => {
        // println!("No URL!");
      }
    }
  }
}

pub async fn main(main_resp: &Html) {
  let document: &Html = &main_resp;
  let selector: Selector = Selector::parse(".tile-3KqhL").unwrap();

  for element in document.select(&selector) {
    match element.value().attr("href") {
      Some(url) => {
        let contents_of_scripture_book_html_data: Html = navigate(url).await.unwrap();

        navigate_manifest(&contents_of_scripture_book_html_data).await;
      }
      _ => {
        println!("Nothing to see here");
      }
    }
  }
}
