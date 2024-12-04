use std::io::{Read, Write};
use std::net::TcpStream;
use std::env;

fn main() -> std::io::Result<()> {
	let args : Vec<String> = env::args().collect();

	if args.len() != 4
	{
		panic!("Use: [ip_address] [port_number] [message]");
	}

	let ip_address : &str = &args[1];
	let port_number : &str = &args[2];
	let message : &str = &args[3];

	let mut stream = TcpStream::connect("{ip_address}:{port_number}")?;

	stream.write_all(message.as_bytes());

	Ok(())
}
