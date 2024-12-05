use core::str;
use std::io::{self, BufRead, Read, Write};
use std::net::TcpStream;
use std::env;
use std::time::Duration;

const BUFFER_SIZE : usize = 4096;

fn main()
{
	let args : Vec<String> = env::args().collect();

	if args.len() != 3
	{
		panic!("Invalid Args:\n\tUse: [Server IP Address] [Port Number]");
	}

	let ip_address : &str = &args[1];
	let port_number : &str = &args[2];

	let server_address : String = format!("{}:{}", ip_address, port_number);
	let mut stream : TcpStream = TcpStream::connect(&server_address).expect("Could not connect to server");

	println!("Connected to the server at {}", server_address);
	let mut reader : io::BufReader<TcpStream> = io::BufReader::new(stream.try_clone().unwrap());
	let stdin : io::Stdin = io::stdin();

	loop
	{
		let mut buffer : [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
		reader.read(&mut buffer).expect("Unable to read server message...");

		println!("[Server] {}", str::from_utf8(&buffer).expect("Invalid UTF-8 String received..."));
		buffer = [0; BUFFER_SIZE];

		let mut input : String = String::new();
		print!("Input: ");
		stdin.read_line(&mut input).expect("Failed to read from stdin");

		// Send the user's input to the server
		stream.write_all(input.as_bytes()).expect("Failed to send data to server");
	}
}