use std::net::{TcpListener, TcpStream}; // Tcp functionality
use std::env; // Reading command-line arguments

fn handle_client(stream : TcpStream)
{
	println!("New connection found at {}", stream.peer_addr().unwrap());
}

fn main() -> std::io::Result<()>
{
	let args : Vec<String> = env::args().collect();

	if args.len() != 3
	{
		panic!("Use: [ip_address] [port number]");
	}
	let ip_address : &str = &args[1];
	let port_number : &str = &args[2];

	let listener = TcpListener::bind(ip_address)?;

	for stream in listener.incoming()
	{
		// Ensure a good connection before going to the 'handle_client' function
		let mut stream = match stream
		{
			Ok(s) => s,
			Err(e) => {
				eprintln!("Failed to accept connection: {}", e);
				continue;
			}
		};

		handle_client(stream);
	}

	Ok(())
}
