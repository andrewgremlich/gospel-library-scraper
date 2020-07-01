mod navigate_scriptures;

use clap::{App, load_yaml};

use gospellibraryscraper::navigate;
use scraper::Html;

#[tokio::main]
async fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();

    println!("{:?}", matches)

    // let scriptures_page_html_data: Html = navigate("/study/scriptures?lang=eng").await.unwrap();

    // navigate_scriptures::get_scripture_books_url(&scriptures_page_html_data).await;
}
