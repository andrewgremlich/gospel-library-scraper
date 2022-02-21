mod navigate_scriptures;

use scraper::Html;

use clap::{Arg, ArgMatches, Command};

use gospellibraryscraper::{navigate, set_env};
use navigate_scriptures::get_scripture_books_url;

#[tokio::main]
async fn main() {
    let matches: ArgMatches = Command::new("Gospel Library Scraper")
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("What file extension should this go to.")
                .default_value("txt"),
        )
        .arg(
            Arg::new("lang")
                .short('l')
                .long("lang")
                .help("Which gospel library language to scrape.")
                .default_value("eng"),
        )
        .get_matches();

    set_env(&matches);

    if let Some(l) = matches.value_of("lang") {
        let url: String = format!("/study/scriptures?lang={}", l);
        let scriptures_page_html_data: Html = navigate(&url).await.unwrap();
        get_scripture_books_url(&scriptures_page_html_data).await;
    }
}
