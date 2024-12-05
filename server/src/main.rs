use std::collections::HashMap;
use std::io::{self, BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::env;

#[derive(Debug)]
struct GameRoom
{
	name: String,
	game_type: String,
	player1: Option<TcpStream>,
	player2: Option<TcpStream>,
}

#[derive(Clone)]
struct Server
{
	rooms: Arc<Mutex<HashMap<String, GameRoom>>>,
}

impl Server
{
	fn new() -> Self
	{
		Server
		{
			rooms: Arc::new(Mutex::new(HashMap::new())),
		}
	}

	fn handle_client(&self, mut stream : TcpStream)
	{
		log("Server", format!("Client connected\n\t{}", stream.peer_addr().unwrap().to_string()));

		let mut reader : BufReader<TcpStream> = io::BufReader::new(stream.try_clone().unwrap());
		let mut buffer = String::new();

		loop
		{
			buffer.clear();
			buffer.push_str("Choose from the following options:\n\t1) Create new room\n\t2) Join existing room\n\t3) Exit\n");
			stream.write_all(buffer.as_bytes()).unwrap();
			stream.flush().expect("msg");
			log("Server", format!("{}", &buffer));

			buffer.clear();
			reader.read_line(&mut buffer).unwrap();
			log("Client", format!("{}", &buffer));
			match buffer.trim()
			{
				"1" => self.create_game_room(&mut stream).unwrap(),
				"2" => self.join_game_room(&mut stream).unwrap(),
				"3" => {
					log("Server", "Ending connection".to_string());
					break;
				}
				_ => {
					stream.write_all(b"Invalid option. Please try again.\n").unwrap();
					log("Server", format!("Invalid option. Please try again."));
				}
			}
		}
	}

	fn create_game_room(&self, stream: &mut TcpStream) -> io::Result<()>
	{
		log("Server", "Starting the 'create game room' process...".to_string());

		let mut reader : BufReader<TcpStream> = io::BufReader::new(stream.try_clone().unwrap());
		let mut buffer : String = String::new();

		stream.write_all(b"Enter game room name: ")?;
		stream.flush()?;
		log("Server", "Enter game room name: ".to_string());

		reader.read_line(&mut buffer)?;
		let room_name = buffer.trim().to_string();
		buffer.clear();

		let mut game_type : String = String::new();
		while String::is_empty(&game_type)
		{
			stream.write_all(b"Choose game type (Tic Tac Toe / Connect 4): ")?;
			stream.flush()?;
			reader.read_line(&mut buffer)?;

			game_type = match buffer.trim()
			{
				"1" => "Tic Tac Toe".to_string(),
				"2" => "Connect 4".to_string(),
				_ => {
					stream.write_all(b"Invalid option... Please type the corresponding number...\n").unwrap();
					"".to_string()
				}
			}
		}

		let mut rooms : std::sync::MutexGuard<'_, HashMap<String, GameRoom>> = self.rooms.lock().unwrap();
		rooms.insert(
			room_name.clone(),
			GameRoom
			{
				name : room_name.clone(),
				game_type,
				player1 : Some(stream.try_clone().unwrap()),
				player2 : None,
			},
		);

		stream.write_all(b"Game room created. Waiting for a second player...\n")?;
		// Wait for a second player to join
		while let Some(room) = rooms.get_mut(&room_name)
		{
			if room.player2.is_none()
			{
				// Wait for another client to join
				// This part can be improved with a more sophisticated approach
				// For simplicity, we will just return here
				return Ok(());
			}
		}

		Ok(())
	}

	fn join_game_room(&self, stream: &mut TcpStream) -> io::Result<()>
	{
		let mut buffer : String = String::new();
		let mut reader : BufReader<TcpStream> = io::BufReader::new(stream.try_clone().unwrap());

		stream.write_all(b"Enter game room name: ")?;
		stream.flush()?;
		reader.read_line(&mut buffer)?;
		let room_name : String = buffer.trim().to_string();

		let mut rooms = self.rooms.lock().unwrap();
		if let Some(room) = rooms.get_mut(&room_name)
		{
			if room.player2.is_none()
			{
				room.player2 = Some(stream.try_clone().unwrap());
				stream.write_all(b"You have joined the game room.\n")?;
				// Start the game logic here
			}
			else
			{
				stream.write_all(b"Game room is full.\n")?;
			}
		}
		else
		{
			stream.write_all(b"Game room does not exist.\n")?;
		}

		Ok(())
	}
}

fn main()
{
	let args : Vec<String> = std::env::args().collect();
	if args.len() != 3
	{
		panic!("Invalid arguments!\n\tUse: [Server IP address] [Port Number]");
	}

	let ip_address : &str = &args[1];
	let port_number : &str = &args[2];

	let server : Server = Server::new();
	let listener : TcpListener = TcpListener::bind(format!("{}:{}", ip_address, port_number)).unwrap();
	println!("Server running on {}:{}", ip_address, port_number);

	for stream in listener.incoming()
	{
		match stream
		{
			Ok(stream) =>
			{
				let server : Server = server.clone();
				thread::spawn(move ||
				{
					server.handle_client(stream);
				});
			}
			Err(e) =>
			{
				eprintln!("Error : {}", e);
			}
		}
	}
}

fn log(owner : &str, log : String)
{
	println!("[{}] {}", owner, log);
}