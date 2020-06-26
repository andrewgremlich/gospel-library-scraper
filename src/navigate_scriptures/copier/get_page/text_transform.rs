use regex::Regex;

pub fn transform_source_scripture_text(chapter_text: Vec<&str>) -> String {
  let digit_regex = Regex::new(r"\d").unwrap();
  let footnote_regex = Regex::new(r"^[a-zA-Z]{1}$").unwrap();

  let mut transformed_chapter_text: String = String::from("");

  for text in chapter_text {
    let trimmed_text = text.trim();

    let is_digit: bool = digit_regex.is_match(trimmed_text);
    let is_char: bool = footnote_regex.is_match(trimmed_text);

    if is_char {
      continue;
    }

    if is_digit {
      transformed_chapter_text.push_str(&format!("\n{}. ", trimmed_text));
    } else {
      transformed_chapter_text.push_str(&format!("{}", text));
    }
  }

  transformed_chapter_text.push_str("\n");

  return transformed_chapter_text;
}
