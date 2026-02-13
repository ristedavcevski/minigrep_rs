use std::{env, process};

use minigrep_rd::Config;

fn main() {
    let ignore_case = env::var("IGNORE_CASE").is_ok();

    let config = Config::build(env::args(), ignore_case).unwrap_or_else(|err| {
        eprintln!("Error while processing required arguments: {err}");
        process::exit(1);
    });

    if let Err(err) = minigrep_rd::run(&config) {
        eprintln!("Error while running the app: {err}");
        process::exit(1);
    }
}
