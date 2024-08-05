use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;
use std::env;

fn is_port_open<A: ToSocketAddrs>(address: A) -> bool {
	match TcpStream::connect_timeout(&address.to_socket_addrs().unwrap().next().unwrap(), Duration::from_secs(1)) {
		Ok(_) => true,
		Err(_) => false,
	}
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <host>", args[0]);
        std::process::exit(1);
    }
	let host = &args[1];
	let ports = [52, 80, 8000];

	for port in &ports {
		let address = format!("{}:{}", host, port);
		if is_port_open(&address) {
			println!("Port {} is open", port);
		} else {
			println!("Port {} is closed", port);
		}
	}
}