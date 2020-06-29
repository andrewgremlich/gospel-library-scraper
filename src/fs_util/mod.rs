use std::fs::File;
use std::io::prelude::*;

use mkdirp::mkdirp;

pub fn write_dir(dir: &str) {
    match mkdirp(dir) {
        Err(e) => println!("{:?}", e),
        _ => (),
    };
}

pub fn write_index_file(file_name: &str, contents: &str) -> std::io::Result<String> {
    let mut file = File::create(file_name)?;
    file.write_all(contents.as_bytes())?;
    Ok(format!("{} index file has been written", file_name))
}
