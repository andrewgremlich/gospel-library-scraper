mod get_page;
mod get_urls;

use std::path::Path;

use scraper::{Html, Selector};

use get_page::{get_page, ScrapedPage, OrderedScrapedPage};
use get_urls::urls_of_chapter;
use gospellibraryscraper::{navigate, write_dir, UrlReference};

// #[derive(Debug)]
// pub struct IndexPage {
//   pub order_number: u16,
//   pub title: String,
//   pub contents: String,
// }

async fn handle_write_file(urls_of_chapter: UrlReference, title_text: &str, order_number: u16) {
  let page_data: ScrapedPage = get_page(&urls_of_chapter.original).await;

  let ordered_scraped_page: OrderedScrapedPage = OrderedScrapedPage {
    order_number: order_number,
    title: String::from(title_text),
    scraped_page: page_data,
  };

  write_dir(&urls_of_chapter.dir);

  if let Ok(d) = ordered_scraped_page.write_section(&urls_of_chapter.file) {
    println!("{:?}", d);
  };
}

async fn handle_html_and_selector(html_to_parse: &Html, selector_to_use: &Selector) {
  let mut order_number: u16 = 1;

  for element in html_to_parse.select(&selector_to_use) {
    let title_text = element.text().collect::<Vec<_>>()[0];

    if let Some(url) = element.value().attr("href") {
      let urls_of_chapter: UrlReference = urls_of_chapter(url);
      let path_exist: bool = Path::new(&urls_of_chapter.file).exists();

      if path_exist {
        println!("exists {}", &urls_of_chapter.file);
      } else {
        // INDEX PAGE WRITE.
        // println!("DIR {}", urls_of_chapter.dir);
        // println!("TITLE {}", book_title);

        handle_write_file(urls_of_chapter, title_text, order_number).await;
      }
    }

    order_number = order_number + 1;
  }
}

//TODO: TITLE TEXT IS FOR INDEX.MD
pub async fn copy(book_title: &str, url: &str) {
  let section_of_book: Html = navigate(url).await.unwrap();
  let active_link: Selector = Selector::parse("a.active-mDRbE").unwrap();
  let list_with_book: Selector = Selector::parse("ul.active-mDRbE a.item-3cCP7").unwrap();

  // handle_html_and_selector(&section_of_book, &active_link, book_title).await;
  handle_html_and_selector(&section_of_book, &list_with_book).await;
}
