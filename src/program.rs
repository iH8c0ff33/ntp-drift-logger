use std::env::Args;
use std::error::Error;
use std::sync::mpsc;
use std::thread;

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
    let mut stats = get_stats(&config.url)?;

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        println!("thread: starting, {} iters", config.iterations);
        for _ in 1..config.iterations {
            if let Ok(current) = get_stats(&config.url) {
                tx.send(current).expect("couldn't send value");
            }
        }
    });

    let mut iterations = 0;
    for sample in rx {
        stats.add_sample(&sample);
        println!("recv: {:?}", sample);
        iterations += 1;
    }

    stats.average(iterations);

    println!("Stats: {:?}, actual iters: {}", stats, iterations);

    Ok(())
}
