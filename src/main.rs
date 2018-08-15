extern crate chrono;
extern crate ntp;
extern crate ntp_drift_logger;

use std::env;
use std::process;

use ntp_drift_logger::program::{run, Config};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Args parse error: {}", err);
        process::exit(1);
    });

    println!("Querying {}, {} iterations", config.url, config.iterations);

    if let Err(err) = run(config) {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}
