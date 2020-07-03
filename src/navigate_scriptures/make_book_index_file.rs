use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use gospellibraryscraper::{no_params, remove_file_name, write_dir};

#[derive(Debug)]
pub struct IndexPage {
    pub index_number: u16,
    pub title: String,
    pub url: String,
}

//TODO write index page only in Hugo switch is on.
impl IndexPage {
    pub fn write_index_file(&self) -> std::io::Result<String> {
        let path_exist: bool = Path::new(&self.url).exists();

        if path_exist {
            return Ok(format!("already exists {}", &self.title));
        } else {
            let mut file = File::create(&self.url)?;

            file.write(b"---\n")?;
            file.write(format!("title: {}\n", &self.title).as_bytes())?;
            file.write(format!("order: {}\n", &self.index_number).as_bytes())?;
            file.write(b"---\n")?;
            file.write_all(b"Index page.\n")?;

            return Ok(format!("wrote {}", &self.url));
        }
    }
}

pub fn write_index(url: &str, book_title: &str, index_count: u16) {
    let file_path: &str = no_params(url);
    let dir_path: String = remove_file_name(file_path);

    let mut file_base: String = String::from(".");

    file_base.push_str(&dir_path);

    write_dir(&file_base);

    file_base.push_str("/_index.md");

    let index_page: IndexPage = IndexPage {
        index_number: index_count,
        title: String::from(book_title),
        url: file_base,
    };

    if let Ok(d) = index_page.write_index_file() {
        println!("{:?}", d);
    };
}
