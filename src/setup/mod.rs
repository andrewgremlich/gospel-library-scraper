use clap::ArgMatches;

use std::env;

pub fn set_env(matches: &ArgMatches) {
    if let Some(f) = matches.value_of("output") {
        env::set_var("OUTPUT_FORMAT", f);
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
