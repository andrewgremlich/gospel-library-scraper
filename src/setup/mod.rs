use clap::ArgMatches;

use std::env;
use std::str::FromStr;

pub fn set_env(matches: &ArgMatches) {

    if let Some(f) = matches.value_of("format") {
        println!("{:?}", f);
        env::set_var("OUTPUT_FORMAT", f);
    }

    if let Some(ref matches) = matches.subcommand_matches("verses") {
        if matches.is_present("notes") {
            env::set_var("NOTES", "true");
            println!("Attaching footnotes...");
        }

        env::set_var("VERSES", "true");
        println!("Running verses...");
    }
}

pub fn get_env_var(var_name: &str) -> String {
    let env_value: String = env::var(var_name).unwrap();
    // maybe do something to parse numbers if number or bool if bool.
    // let hugo_files_bool: bool = FromStr::from_str(&write_hugo_files).unwrap();
    return env_value;
}
