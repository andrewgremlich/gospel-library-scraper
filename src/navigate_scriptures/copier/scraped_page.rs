mod text_transform;

use std::fs::File;
use std::io::prelude::*;
use std::process;

use gospellibraryscraper::{get_env_var, navigate, write_dir, UrlReference};
use scraper::{Html, Selector};
use text_transform::transform_source_scripture_text;

#[derive(Debug)]
struct Page {
    pub contents: String,
    pub summary: String,
}

impl Page {
    async fn new(original_url: &str) -> Page {
        let section_of_book: Html = navigate(original_url).await.unwrap();
        let chapter_text_selector = Selector::parse(".renderFrame-1yHgQ .body-block").unwrap();
        let chapter_summary_selector = Selector::parse("#study_summary1").unwrap();
        let mut page: Page = Page {
            contents: String::from(""),
            summary: String::from(""),
        };
        if let Some(d) = section_of_book.select(&chapter_summary_selector).next() {
            let chapter_summary: &str = d.text().collect::<Vec<_>>()[0];
            page.summary = String::from(chapter_summary);
        }
        if let Some(d) = section_of_book.select(&chapter_text_selector).next() {
            let chapter_text: Vec<&str> = d.text().collect::<Vec<_>>();
            let transformed_text: String = transform_source_scripture_text(chapter_text);
            page.contents = transformed_text;
        }
        return page;
    }
}

#[derive(Debug)]
pub struct ScrapedPage {
    order_number: u16,
    title: String,
    page: Page,
    file_name: String,
    directory_name: String,
    output_format: String,
}

impl ScrapedPage {
    pub async fn new(
        urls_of_chapter: UrlReference,
        title_text: &str,
        order_number: u16,
    ) -> ScrapedPage {
        let page_data: Page = Page::new(&urls_of_chapter.metadata.original).await;

        let ordered_scraped_page: ScrapedPage = ScrapedPage {
            order_number: order_number,
            title: String::from(title_text),
            page: page_data,
            file_name: urls_of_chapter.file,
            directory_name: urls_of_chapter.dir,
            output_format: get_env_var("OUTPUT_FORMAT"),
        };

        return ordered_scraped_page;
    }

    pub fn write_handler(&self) {
        write_dir(&self.directory_name);

        match self.output_format.as_ref() {
            "hugo" => {
                self.write_hugo_handler();
            }
            "md" => {
                if let Ok(d) = self.write_md_section() {
                    println!("{:?}", d);
                };
            }
            _ => {
                println!("No output format detected!");
                process::exit(-1);
            }
        }
    }

    fn write_hugo_handler(&self) {
        let verses_env = get_env_var("VERSES");
        let notes_env = get_env_var("NOTES");

        if verses_env == "true" {
            if notes_env == "true" {
                //write with footnotes
            } else {
                //write without footnotes
            }
        } else {
            if let Ok(d) = self.write_regular_files() {
                println!("{:?}", d);
            };
        }
    }

    fn write_md_section(&self) -> std::io::Result<String> {
        let mut file = File::create(&self.file_name)?;
        file.write_all(&self.page.contents.as_bytes())?;
        return Ok(format!("wrote {}", &self.file_name));
    }

    fn write_regular_files(&self) -> std::io::Result<String> {
        let mut file = File::create(&self.file_name)?;
        file.write(b"---\n")?;
        file.write(format!("title: {}\n", &self.title).as_bytes())?;
        file.write(format!("description: {}\n", &self.page.summary).as_bytes())?;
        file.write(format!("order: {}\n", &self.order_number).as_bytes())?;
        file.write(b"---\n")?;
        file.write_all(&self.page.contents.as_bytes())?;

        return Ok(format!("wrote {}", &self.file_name));
    }
}
