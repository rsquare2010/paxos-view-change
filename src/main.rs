use std::net::UdpSocket;
use std::time::Duration;
use dns_lookup::getnameinfo;
use std::net::{IpAddr, SocketAddr};
use std::thread;
fn main() {
    println!("Hello, world!");

    // gethostname::gethostname
    // let mut addrs_iter = "udptestsend:21233".to_socket_addrs().unwrap();
    // let ips: Vec<std::net::IpAddr> = lookup_host(hostname).unwrap();
    thread::sleep(Duration::from_millis(4000));
    // let mut addrs_iter = "udptestsend:21233".to_socket_addrs().unwrap();
    // let socket = UdpSocket::bind(addrs_iter).expect("couldn't bind to address");
    let hostname = "udptestsend";
    let ips: Vec<std::net::IpAddr> = lookup_host(hostname).unwrap();
    let port = 21233;
    let socketadds: SocketAddr = (ips.get(0), port).into();
    let socket = UdpSocket::bind(socketadds).expect("couldn't bind to address");
    // let socket = UdpSocket::bind("udptestsend:21233").expect("couldn't bind to address");
    println!("looks like bind was successful");
    // let mut buf = [0; 10];
    // let (number_of_bytes, src_addr) = socket.recv_from(&mut buf)
    //                                     .expect("Didn't receive data");
}
