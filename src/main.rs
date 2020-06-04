mod navigate_scripture_book;

use gospellibraryscraper::navigate;
use scraper::Html;

#[tokio::main]
async fn main() {
    let scriptures_page_html_data: Html =
        navigate("/study/scriptures?lang=eng")
            .await
            .unwrap();

    navigate_scripture_book::main(&scriptures_page_html_data).await;
}
