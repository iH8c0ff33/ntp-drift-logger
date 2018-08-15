use std::env::Args;
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::mpsc;
use std::thread;

use chrono::offset::Local;

use time::{get_stats, Average, Stats};
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

    let (file_tx, rx) = mpsc::channel::<Stats>();
    thread::spawn(|| {
        let mut file = OpenOptions::new()
            .append(true)
            .open("drift.csv")
            .unwrap_or_else(|_| {
                let mut file = File::create("drift.csv").expect("file: couldn't create file");

                file.write(b"timestamp,offset,delay\n")
                    .expect("file: couldn't write header in file");

                file
            });

        for data in rx {
            file.write(format!("{},{}\n", Local::now(), data).as_bytes())
                .expect("file: couldn't write to file");
        }
    });

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        println!("ntp: starting, {} iters", config.iterations);
        for _ in 1..config.iterations {
            if let Ok(current) = get_stats(&config.url) {
                tx.send(current).expect("ntp: couldn't send sample");
            }
        }
    });

    let mut iterations = 0;
    for sample in rx {
        stats.add_sample(&sample);
        println!("recv: {:?}", sample);
        file_tx.send(sample).expect("main: couldn't send sample");
        iterations += 1;
    }

    stats.average(iterations);

    println!("Stats: {:?}, actual iters: {}", stats, iterations);

    Ok(())
}
