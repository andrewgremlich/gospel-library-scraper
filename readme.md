# Gospel Library Scraper

An exploration of implementing a web scraper with Rust.

Also a way to find how to build static Markdown files that can be used with [Hugo](https://gohugo.io/).

Also a duplicate of [Node Gospel Library Scraper](https://github.com/andrewgremlich/node-gospel-library-scraper).

Currently only scrapes `/study/scriptures?lang=eng`.

## use

`cargo run` to get everything from scriptures of the gospel library.

Will output Markdown files to a `study/` folder in the present working directory.

## upgrade ideas

1. Provide command line option to parse into hugo compatible files.

2. Provide a command line option to get a language.

    - provide language codes.

3. Provide command line argument to go for verses.

    - provide grab foot notes as well.

## reference

- [Web scraping in rust](https://codeburst.io/web-scraping-in-rust-881b534a60f7)

## crates used

- [Tokio](https://tokio.rs/docs/getting-started/hello-world/)
- [scraper.rs](https://docs.rs/crate/scraper/0.12.0)
- [reqwest](https://docs.rs/reqwest/0.10.4/reqwest/index.html)
- [mkdirp](https://docs.rs/mkdirp/1.0.0/mkdirp/)
- [regex](https://docs.rs/regex/1.3.9/regex/)