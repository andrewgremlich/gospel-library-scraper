use clap::ArgMatches;
use std::env;

pub fn env(matches: &ArgMatches) {
    match matches.occurrences_of("go-hugo") {
        1 => {
            env::set_var("GO_HUGO", "true");
            println!("Writing files to be hugo compatible.")
        }
        _ => {
            env::set_var("GO_HUGO", "false");
        }
    }

    if let Some(ref matches) = matches.subcommand_matches("verses") {
        if matches.is_present("footnotes") {
            env::set_var("FOOTNOTES", "true");
            println!("Attaching footnotes...");
        }

        env::set_var("VERSES", "true");
        println!("Running verses...");
    }
}
