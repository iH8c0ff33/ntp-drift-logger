use std::fmt;
use std::io;
use std::net::ToSocketAddrs;

use chrono;
use chrono::TimeZone;
use ntp;

pub fn local_time(timestamp: ntp::protocol::TimestampFormat) -> chrono::DateTime<chrono::Local> {
    let unix_time = ntp::unix_time::Instant::from(timestamp);
    chrono::Local.timestamp(unix_time.secs(), unix_time.subsec_nanos() as _)
}

pub trait Average {
    fn add_sample(&mut self, &Self);
    fn average(&mut self, i32);
}

#[derive(Debug)]
pub struct Stats {
    delay: chrono::Duration,
    offset: chrono::Duration,
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{},{}",
            self.offset.num_milliseconds(),
            self.delay.num_milliseconds()
        )
    }
}

impl Average for Stats {
    fn add_sample(&mut self, sample: &Stats) {
        self.delay = self.delay + sample.delay;
        self.offset = self.offset + sample.offset;
    }

    fn average(&mut self, iterations: i32) {
        self.delay = self.delay / iterations;
        self.offset = self.offset / iterations;
    }
}

pub fn get_stats<T: ToSocketAddrs>(address: T) -> io::Result<Stats> {
    let ntp_response = ntp::request(address)?;

    let t4 = chrono::Local::now();
    let t1 = local_time(ntp_response.origin_timestamp);
    let t2 = local_time(ntp_response.receive_timestamp);
    let t3 = local_time(ntp_response.transmit_timestamp);

    let delay = (t4 - t1) - (t3 - t2);
    let offset = ((t2 - t1) + (t3 - t4)) / 2;

    Ok(Stats { delay, offset })
}
