#[derive(Debug)]
pub struct UrlMetaData {
  pub original: String,
  no_params: String,
}

#[derive(Debug)]
pub struct UrlReference {
  pub file: String,
  pub dir: String,
  pub metadata: UrlMetaData,
}

pub fn remove_file_name(url: &str) -> String {
  let mut file_name_vector: Vec<&str> = url.split("/").collect();

  file_name_vector.pop();

  let dir_append: String = file_name_vector.join("/");

  return dir_append;
}

fn dir_to_write(url: &str) -> String {
  let dir_append: String = remove_file_name(url);
  let mut dir_to_write: String = ".".to_owned();

  dir_to_write.push_str(&dir_append);

  return dir_to_write;
}

fn file_to_write(url: &str) -> String {
  let mut file_to_write: String = ".".to_owned();
  file_to_write.push_str(url);
  file_to_write.push_str(".md");

  return file_to_write;
}

pub fn no_params(url: &str) -> &str {
  let no_query_params: Vec<&str> = url.split("?").collect();
  return no_query_params[0];
}

pub fn urls_of_chapter(url: &str) -> UrlReference {
  let no_query_param: &str = no_params(url);

  let dir_to_write: String = dir_to_write(no_query_param);
  let file_to_write: String = file_to_write(no_query_param);

  let metadata: UrlMetaData = UrlMetaData {
    no_params: String::from(no_query_param),
    original: String::from(url),
  };

  let urls = UrlReference {
    file: String::from(file_to_write),
    dir: String::from(dir_to_write),
    metadata: metadata,
  };

  return urls;
}
