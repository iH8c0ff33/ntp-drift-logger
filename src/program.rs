use std::env::Args;
use std::error::Error;

use time::{get_stats, Average};
pub struct Config {
    pub url: String,
    pub iterations: i32,
}

impl Config {
    pub fn new(mut args: Args) -> Result<Config, &'static str> {
        args.next();

        let url = match args.next() {
            Some(arg) => arg,
            None => String::from("ntp1.inrim.it:123"),
        };

        let iterations = match args.next() {
            Some(arg) => arg.parse().unwrap_or(1),
            None => 1,
        };

        Ok(Config { url, iterations })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut iterations = config.iterations;
    let mut stats = get_stats(&config.url)?;

    for _ in 1..iterations {
        match get_stats(&config.url) {
            Ok(new) => stats.add_sample(new),
            _ => iterations -= 1,
        };
    }

    stats.average(iterations);

    println!("Stats: {:?}", stats);

    Ok(())
}
