extern crate ntp;

fn main() {
    let address = "0.europe.pool.ntp.org:123";
    let response: ntp::packet::Packet = ntp::request(address).unwrap();
    println!("res: {:?}", response);
}
