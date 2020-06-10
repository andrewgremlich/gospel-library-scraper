mod get_page;
mod get_urls;

use scraper::{Html, Selector};

use get_page::{get_page, ScrapedPage};
use get_urls::urls_of_first_chapter;
use gospellibraryscraper::{navigate, write_dir, write_section, UrlReference};

#[derive(Debug)]
pub struct OrderedScrapedPage {
  pub order_number: u8,
  pub title: String,
  pub scraped_page: ScrapedPage,
}

// fn parse_books_with_chapters(section_of_book: &Html, list_with_book: &Selector) {
//   let mut order_number: u8 = 1;

//   for element in section_of_book.select(&list_with_book) {
//     let text = element.text().collect::<Vec<_>>()[0];
//     println!("{:?}", text);
//   }
// }

async fn parse_first_chapters(title_text: &str, section_of_book: &Html, active_link: &Selector) {
  for element in section_of_book.select(&active_link) {
    if let Some(url) = element.value().attr("href") {
      let urls_of_first_chapter: UrlReference = urls_of_first_chapter(url);
      let page_data: ScrapedPage = get_page(&urls_of_first_chapter.original).await;

      let ordered_scraped_page: OrderedScrapedPage = OrderedScrapedPage {
        order_number: 1,
        title: String::from(title_text),
        scraped_page: page_data,
      };

      write_dir(&urls_of_first_chapter.dir);

      if let Ok(d) = write_section(
        &urls_of_first_chapter.file,
        &ordered_scraped_page.scraped_page.contents,
      ) {
        println!("{:?}", d);
      };
    }
  }
}

pub async fn copy(title_text: &str, url: &str) {
  let section_of_book: Html = navigate(url).await.unwrap();
  let active_link: Selector = Selector::parse("a.active-mDRbE").unwrap();
  // let list_with_book: Selector = Selector::parse("ul.active-mDRbE a.item-3cCP7").unwrap();
  parse_first_chapters(title_text, &section_of_book, &active_link).await;
  // parse_books_with_chapters(&section_of_book, &list_with_book);
}
