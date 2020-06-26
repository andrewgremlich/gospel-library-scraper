mod copier;

use copier::{copy_book, make_book_index_file};
use gospellibraryscraper::navigate;
use scraper::{Html, Selector};

async fn navigate_manifest(manifest: &Html) {
  let manifest_page: &Html = &manifest;
  let manifest_links_selector: Selector = Selector::parse(".manifest a").unwrap();

  let mut index_count: u16 = 1;

  for book_of_scripture_link in manifest_page.select(&manifest_links_selector) {
    let book_title: &str = book_of_scripture_link.text().collect::<Vec<_>>()[0];

    if let Some(href) = book_of_scripture_link.value().attr("href") {
      make_book_index_file(href, book_title, index_count);
      copy_book(href).await;
      index_count = index_count + 1;
    }
  }
}

pub async fn get_scripture_books_url(main_resp: &Html) {
  let document: &Html = &main_resp;
  let selector: Selector = Selector::parse(".tile-3KqhL").unwrap();

  for element in document.select(&selector) {
    if let Some(url) = element.value().attr("href") {
      let manifest_page: Html = navigate(url).await.unwrap();
      navigate_manifest(&manifest_page).await;
    }
  }
}
