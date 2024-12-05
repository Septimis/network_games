use std::io::{self, Write, BufRead};
use std::net::TcpStream;
use std::env;

fn main()
{
	let args : Vec<String> = env::args().collect();
	if args.len() != 3
	{
		panic!("Invalid arguments!\n\tUse [ip address] [port number]");
	}

	let ip_address : &str = &args[1];
	let port_number : &str = &args[2];

	let stream = TcpStream::connect(format!("{}:{}", ip_address, port_number)).unwrap();
	let mut reader = io::BufReader::new(stream.try_clone().unwrap());
	let mut writer = stream;

	loop
	{
		let mut input = String::new();
		reader.read_line(&mut input).unwrap();
		println!("{}", input.trim());

		if input.contains("wins")
		{
			break;
		}

		let mut move_input = String::new();
		io::stdin().read_line(&mut move_input).unwrap();
		writer.write_all(move_input.as_bytes()).unwrap();
		writer.flush().unwrap();
	}
}