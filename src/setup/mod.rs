use clap::ArgMatches;

use std::env;

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
    match env::var(var_name) {
        Ok(d) => {
            return d;
        }
        Err(_) => {
            return format!("Env var not set for {}", var_name);
        }
    }
}
