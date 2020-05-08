use scraper::{Html, Selector};

const CHURCH_ROOT: &str = "https://www.churchofjesuschrist.org";

pub async fn navigate(
  url: &str,
  selector_str: &str,
) -> Result<(Html, Selector), Box<dyn std::error::Error>> {
  let navigate_url: String = format!("{}{}", CHURCH_ROOT, url);
  let resp = reqwest::get(&navigate_url).await?.text().await?;

  let document = Html::parse_document(&resp);
  let selector = Selector::parse(selector_str).unwrap();

  return Ok((document, selector));
}
