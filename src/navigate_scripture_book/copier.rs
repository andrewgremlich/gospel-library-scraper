use gospellibraryscraper::{
  navigate, urls_of_first_chapter, write_index_file, write_section, UrlReference,
};
use scraper::{Html, Selector};

async fn get_page(urls_of_first_chapter: UrlReference, page_title: &str, order_number: u8) {
  let section_of_book: Html = navigate(&urls_of_first_chapter.original).await.unwrap();

  let dominant_sel = Selector::parse(".dominant").unwrap();
  let body_block_sel = Selector::parse(".renderFrame-1yHgQ .body-block").unwrap();
  let study_summary_sel = Selector::parse("#study_summary1").unwrap();

  if let Some(d) = section_of_book.select(&dominant_sel).next() {
    println!("{:?}", d.html());
  }

  // if let Some(d) = section_of_book.select(&body_block_sel).next() {
  //   println!("{:?}", d.inner_html());
  // }
  
  // if let Some(d) = section_of_book.select(&study_summary_sel).next() {
  //   println!("{:?}", d.inner_html());
  // }

  // println!("{:?}", urls_of_first_chapter);
}

// fn parse_books_with_chapters(section_of_book: &Html, list_with_book: &Selector) {
//   let mut order_number: u8 = 1;

//   for element in section_of_book.select(&list_with_book) {
//     let text = element.text().collect::<Vec<_>>()[0];
//     println!("{:?}", text);
//   }
// }

async fn parse_first_chapters(section_of_book: &Html, active_link: &Selector) {
  let mut order_number: u8 = 1;

  for element in section_of_book.select(&active_link) {
    let page_title = element.text().collect::<Vec<_>>()[0];

    if let Some(url) = element.value().attr("href") {
      // get page code
      let urls_of_first_chapter: UrlReference = urls_of_first_chapter(url);
      get_page(urls_of_first_chapter, page_title, order_number).await;
      order_number = order_number + 1;
    }
  }
}

pub async fn main(url: &str, index: u8) {
  let section_of_book: Html = navigate(url).await.unwrap();
  let active_link: Selector = Selector::parse("a.active-mDRbE").unwrap();
  // let list_with_book: Selector = Selector::parse("ul.active-mDRbE a.item-3cCP7").unwrap();
  parse_first_chapters(&section_of_book, &active_link).await;
  // parse_books_with_chapters(&section_of_book, &list_with_book);
}
