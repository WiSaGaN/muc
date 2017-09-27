extern crate chrono;
extern crate net2;

use net2::UdpBuilder;
use std::net::Ipv4Addr;
use std::str::FromStr;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 3 {
        println!("Usage: {} group_ip port [NIC]", args[0]);
        return;
    }
    let mc_ip = Ipv4Addr::from_str(&args[1]).expect("cannot parse group_ip");
    let mc_port = args[2].parse::<u16>().expect("cannot parse port");
    let nic_string = args.get(3).cloned().unwrap_or("0.0.0.0".into());
    let nic = Ipv4Addr::from_str(&nic_string).expect("cannot parse nic");
    let socket = UdpBuilder::new_v4().expect("cannot create UDP socket")
        .reuse_address(true).expect("cannot set reuse address")
        .bind((mc_ip, mc_port)).expect("cannot bind");
    socket.join_multicast_v4(&mc_ip, &nic).expect("cannot join");
    let mut buffer = [0u8; 1600];
    loop {
        let (size, addr) = socket.recv_from(&mut buffer).expect("cannot recv");
        let now = chrono::Local::now();
        println!("{}, {} bytes from {}, {:?}", now, size, addr, &buffer[..size]);
    }
}
