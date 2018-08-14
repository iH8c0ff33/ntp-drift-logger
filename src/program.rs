use std::env::Args;

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
