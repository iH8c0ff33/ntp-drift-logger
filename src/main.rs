extern crate chrono;
extern crate ntp;

use std::io;
use std::net::ToSocketAddrs;

use chrono::TimeZone;

#[derive(Debug)]
struct Stats {
    delay: chrono::Duration,
    offset: chrono::Duration,
}

fn local_time(timestamp: ntp::protocol::TimestampFormat) -> chrono::DateTime<chrono::Local> {
    let unix_time = ntp::unix_time::Instant::from(timestamp);
    chrono::Local.timestamp(unix_time.secs(), unix_time.subsec_nanos() as _)
}

fn get_stats<T: ToSocketAddrs>(address: T) -> io::Result<Stats> {
    let ntp_response = ntp::request(address)?;

    let t4 = chrono::Local::now();
    let t1 = local_time(ntp_response.origin_timestamp);
    let t2 = local_time(ntp_response.receive_timestamp);
    let t3 = local_time(ntp_response.transmit_timestamp);

    let delay = (t4 - t1) - (t3 - t2);
    let offset = ((t2 - t1) + (t3 - t4)) / 2;

    Ok(Stats { delay, offset })
}

fn main() {
    let address = "ntp1.inrim.it:123";

    let stats = get_stats(address).unwrap();

    println!("stats: {:?}", stats);
}
