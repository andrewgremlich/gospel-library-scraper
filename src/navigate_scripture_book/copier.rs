use gospellibraryscraper::navigate;
use scraper::{Html, Selector};

use gospellibraryscraper::no_query_params;

async fn get_page() {
  println!("Get page");
}

fn parse_books_with_chapters(section_of_book: &Html, list_with_book: &Selector) {
  let mut order_number: u8 = 1;

  for element in section_of_book.select(&list_with_book) {
    let text = element.text().collect::<Vec<_>>()[0];
    println!("{:?}", text);
  }
}

fn parse_first_chapters(section_of_book: &Html, active_link: &Selector) {
  let mut order_number: u8 = 1;

  for element in section_of_book.select(&active_link) {
    let page_title = element.text().collect::<Vec<_>>()[0];

    match element.value().attr("href") {
      Some(url) => {
        //get page code
        let straight_url: &str = no_query_params(url);
        // get_page().await;
        order_number = order_number + 1;
        println!("{:?}", straight_url);
      }
      _ => {
        // println!("No URL!");
      }
    }
  }
}

pub async fn main(url: &str, index: u8) {
  let section_of_book: Html = navigate(url).await.unwrap();

  let active_link: Selector = Selector::parse("a.active-mDRbE").unwrap();
  let list_with_book: Selector = Selector::parse("ul.active-mDRbE a.item-3cCP7").unwrap();

  parse_first_chapters(&section_of_book, &active_link);
  parse_books_with_chapters(&section_of_book, &list_with_book);
}
