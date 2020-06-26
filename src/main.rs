mod navigate_scriptures;

use gospellibraryscraper::navigate;
use scraper::Html;

#[tokio::main]
async fn main() {
    let scriptures_page_html_data: Html = navigate("/study/scriptures?lang=eng").await.unwrap();

    navigate_scriptures::get_scripture_books_url(&scriptures_page_html_data).await;
}
