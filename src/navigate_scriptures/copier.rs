mod scraped_page;

use std::path::Path;

use scraper::{Html, Selector};

use gospellibraryscraper::{navigate, urls_of_chapter, UrlReference};
use scraped_page::ScrapedPage;

async fn handle_html_and_selector(html_to_parse: &Html, selector_to_use: &Selector) {
    let mut order_number: u16 = 1;

    for element in html_to_parse.select(&selector_to_use) {
        let title_text = element.text().collect::<Vec<_>>()[0];

        if let Some(url) = element.value().attr("href") {
            let urls_of_chapter: UrlReference = urls_of_chapter(url);
            let path_exist: bool = Path::new(&urls_of_chapter.file).exists();

            //Keep this conditonal here so that `handle_write_file` could be skipped.
            if path_exist {
                println!("exists {}", &urls_of_chapter.file);
            } else {
                let scraped_page =
                    ScrapedPage::new(urls_of_chapter, title_text, order_number).await;

                scraped_page.write_handler();
            }
        }

        order_number = order_number + 1;
    }
}

pub async fn copy_book(url: &str) {
    let section_of_book: Html = navigate(url).await.unwrap();
    let list_with_book: Selector = Selector::parse("ul.active-mDRbE a.item-3cCP7").unwrap();

    // TODO: This might not be needed.
    // let active_link: Selector = Selector::parse("a.active-mDRbE").unwrap();
    // handle_html_and_selector(&section_of_book, &active_link).await;
    handle_html_and_selector(&section_of_book, &list_with_book).await;
}
