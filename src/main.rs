use std::env;
use std::process;

use srt2vtt::{Config, run};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Error: {}", err);
        process::exit(1);
    });

    if let Err(err) = run(config) {
        println!("Error: {}", err);
        process::exit(1);
    }
}