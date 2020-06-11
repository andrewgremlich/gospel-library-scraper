use gospellibraryscraper::UrlReference;

pub fn urls_of_chapter(url: &str) -> UrlReference {
  let no_query_param: Vec<&str> = url.split("?").collect();

  let mut file_name_vector: Vec<&str> = no_query_param[0].split("/").collect();
  file_name_vector.pop();
  let dir_append: String = file_name_vector.join("/");

  let mut file_to_write: String = ".".to_owned();
  let mut dir_to_write: String = ".".to_owned();
  file_to_write.push_str(no_query_param[0]);
  file_to_write.push_str(".md");

  dir_to_write.push_str(&dir_append);

  let urls = UrlReference {
    file: String::from(file_to_write),
    dir: String::from(dir_to_write),
    no_params: String::from(no_query_param[0]),
    original: String::from(url),
  };

  return urls;
}
