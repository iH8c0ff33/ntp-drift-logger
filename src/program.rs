use std::env::Args;
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{stdout, Write};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use chrono::offset::Local;

use time::{get_stats, Average, Stats};
#[derive(Clone)]
pub struct Config {
    pub url: String,
    pub samples: i32,
    pub filename: String,
}

impl Config {
    pub fn new(mut args: Args) -> Result<Config, &'static str> {
        args.next();

        let url = match args.next() {
            Some(arg) => arg,
            None => String::from("ntp1.inrim.it:123"),
        };

        let samples = match args.next() {
            Some(arg) => arg.parse().unwrap_or(1),
            None => 1,
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => format!("drift.{}.csv", Local::now().timestamp_millis()),
        };

        Ok(Config {
            url,
            samples,
            filename,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let (file_tx, rx) = mpsc::channel::<Stats>();
    let filename = config.filename;
    thread::spawn(move || {
        let mut file = OpenOptions::new()
            .append(true)
            .open(&filename)
            .unwrap_or_else(|_| {
                let mut file = File::create(&filename).expect("file: couldn't create file");

                file.write(b"timestamp,offset,delay\n")
                    .expect("file: couldn't write header in file");

                file
            });

        for data in rx {
            file.write(format!("{},{}\n", Local::now(), data).as_bytes())
                .expect("file: couldn't write to file");
        }
    });

    let samples = config.samples;
    loop {
        let mut stats = get_stats(&config.url)?;

        let (tx, rx) = mpsc::channel();
        let url = config.url.clone();
        thread::spawn(move || {
            println!("ntp: starting, {} iters", samples);
            for _ in 1..samples {
                if let Ok(current) = get_stats(&url) {
                    tx.send(current).expect("ntp: couldn't send sample");
                }
            }
        });

        let mut iterations = 1;
        let mut output = stdout();
        for sample in rx {
            stats.add_sample(&sample);
            print!("\rmain: sample {}/{}", iterations, config.samples);
            output.flush()?;
            iterations += 1;
        }
        println!();

        stats.average(iterations);

        println!("Stats: {:?}, actual samples: {}", stats, iterations);
        file_tx.send(stats)?;

        println!("main: done, sleeping 30 secs...");
        thread::sleep(Duration::from_secs(30));
    }
}
