use scraper::Html;

const CHURCH_ROOT: &str = "https://www.churchofjesuschrist.org";

pub async fn navigate(url: &str) -> Result<Html, Box<dyn std::error::Error>> {
    let navigate_url: String = format!("{}{}", CHURCH_ROOT, url);
    let resp = reqwest::get(&navigate_url).await?.text().await?;

    let document = Html::parse_document(&resp);

    return Ok(document);
}
