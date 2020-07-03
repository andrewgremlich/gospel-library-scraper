mod navigate_scriptures;

use scraper::Html;
use std::env;

use clap::{load_yaml, App, ArgMatches};
use gospellibraryscraper::{navigate, set_env};
use navigate_scriptures::get_scripture_books_url;

#[tokio::main]
async fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches: ArgMatches = App::from(yaml).get_matches();

    set_env(&matches);

    if let Some(l) = matches.value_of("lang") {
        let url: String = format!("/study/scriptures?lang={}", l);

        //TODO: implement these... this will take some time!
        println!("{:?}", env::var("NOTES"));
        println!("{:?}", env::var("VERSES"));

        let scriptures_page_html_data: Html = navigate(&url).await.unwrap();
        get_scripture_books_url(&scriptures_page_html_data).await;
    }
}
