mod get_page;

use std::path::Path;

use scraper::{Html, Selector};

use get_page::{get_page, OrderedScrapedPage, ScrapedPage};
use gospellibraryscraper::{
    get_env_var, navigate, urls_of_chapter, write_dir, UrlReference,
};

async fn handle_write_file(urls_of_chapter: UrlReference, title_text: &str, order_number: u16) {
    let page_data: ScrapedPage = get_page(&urls_of_chapter.metadata.original).await;

    let ordered_scraped_page: OrderedScrapedPage = OrderedScrapedPage {
        order_number: order_number,
        title: String::from(title_text),
        scraped_page: page_data,
    };

    let output_format: String = get_env_var("OUTPUT_FORMAT");

    write_dir(&urls_of_chapter.dir);

    if output_format == "hugo" {
        if let Ok(d) = ordered_scraped_page.write_with_hugo_header(&urls_of_chapter.file) {
            println!("{:?}", d);
        };
    } else if output_format == "md" {
        if let Ok(d) = ordered_scraped_page.write_md_section(&urls_of_chapter.file) {
            println!("{:?}", d);
        };
    }
}

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
                handle_write_file(urls_of_chapter, title_text, order_number).await;
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
