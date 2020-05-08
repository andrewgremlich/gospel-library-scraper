use gospellibraryscraper::navigate;
use scraper::{Html, Selector};

pub async fn main(main_resp: &(Html, Selector)) {
  let document: &Html = &main_resp.0;
  let selector: &Selector = &main_resp.1;

  for element in document.select(&selector) {
    match element.value().attr("href") {
      Some(url) => {
        println!("{:?}", url);

        let contents_of_scripture_book_html_data: (Html, Selector) =
          navigate(url, "h1#title1").await.unwrap();

        let contents_html: Html = contents_of_scripture_book_html_data.0;
        let contents_selector: Selector = contents_of_scripture_book_html_data.1;

        for title in contents_html.select(&contents_selector) {
          let title: Vec<&str> = title.text().collect::<Vec<_>>();
          println!("{:?}", title[0]);
        }
      }
      _ => {
        println!("Nothing to see here");
      }
    }
  }
}
