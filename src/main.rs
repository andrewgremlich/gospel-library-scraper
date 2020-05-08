mod navigate_scripture_book;

use gospellibraryscraper::navigate;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() {
    let scriptures_page_html_data: (Html, Selector) =
        navigate("/study/scriptures?lang=eng", ".tile-3KqhL")
            .await
            .unwrap();

    navigate_scripture_book::main(&scriptures_page_html_data).await;
}
