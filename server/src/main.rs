use std::io::{self, Write, BufRead};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::env;

const BOARD_SIZE : usize = 3;
const EMPTY_SPACE : char = '-';

struct Game
{
	board: [ [char; BOARD_SIZE]; BOARD_SIZE ],
	current_player: char,
	round_number: usize,
}

impl Game
{
	fn new() -> Self
	{
		Game
		{
			board: [[EMPTY_SPACE; BOARD_SIZE]; BOARD_SIZE],
			current_player: 'X',
			round_number: 0,
		}
	}

	fn print_board(&self)
	{
		println!("Round {}", self.round_number % 2);
		for row in &self.board
		{
			println!("{}", row.iter().collect::<String>());
		}
		println!("\n\n");
	}

	fn make_move(&mut self, row: usize, col: usize) -> bool
	{
		if self.board[row][col] == EMPTY_SPACE
		{
			self.board[row][col] = self.current_player;
			self.current_player = if self.current_player == 'X' { 'O' } else { 'X' };

			self.round_number += 1;

			return true
		}
		
		return false
	}

	fn check_winner(&self) -> Option<char>
	{
		for i in 0..BOARD_SIZE
		{
			if self.board[i][0] != EMPTY_SPACE && self.board[i][0] == self.board[i][1] && self.board[i][1] == self.board[i][2]
			{
				return Some(self.board[i][0]);
			}
			if self.board[0][i] != EMPTY_SPACE && self.board[0][i] == self.board[1][i] && self.board[1][i] == self.board[2][i]
			{
				return Some(self.board[0][i]);
			}
		}

		if self.board[0][0] != EMPTY_SPACE && self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2]
		{
			return Some(self.board[0][0]);
		}
		if self.board[0][2] != EMPTY_SPACE && self.board[0][2] == self.board[1][1] && self.board[1][1] == self.board[2][0]
		{
			return Some(self.board[0][2]);
		}

		return None
	}
}

fn handle_client(stream: TcpStream, game: Arc<Mutex<Game>>, player: char)
{
	println!("[Server] Player {player} connected");

	let mut reader = io::BufReader::new(stream.try_clone().unwrap());
	let mut writer = stream;

	loop
	{
		{
			let game = game.lock().unwrap();
			game.print_board();
			writeln!(writer, "Player {}, enter your move (row and column): ", player).unwrap();
			writer.flush().unwrap();
		}

		let mut input = String::new();
		reader.read_line(&mut input).unwrap();
		let coords: Vec<usize> = input.trim().split_whitespace()
			.filter_map(|s| s.parse().ok()).collect();

		if coords.len() != 2
		{
			writeln!(writer, "Invalid input. Please enter row and column.").unwrap();
			continue;
		}

		let (row, col) = (coords[0], coords[1]);
		let mut game = game.lock().unwrap();
		if row >= BOARD_SIZE || col >= BOARD_SIZE || !game.make_move(row, col)
		{
			writeln!(writer, "Invalid move. Try again.").unwrap();
			continue;
		}

		if let Some(winner) = game.check_winner()
		{
			game.print_board();
			writeln!(writer, "Player {} wins!", winner).unwrap();
			println!("Player {} wins!", winner);
			std::process::exit(0);
		}
	}
}

fn main()
{
	let args : Vec<String> = env::args().collect();
	if args.len() != 3
	{
		panic!("Invalid arguments!\n\tUse [ip address] [port number]");
	}

	let ip_address : &str = &args[1];
	let port_number : &str = &args[2];

	let listener = TcpListener::bind(format!("{}:{}", ip_address, port_number)).unwrap();
	let game = Arc::new(Mutex::new(Game::new()));
	let mut player_count = 0;

	for stream in listener.incoming()
	{
		let stream = stream.unwrap();
		let player = if player_count % 2 == 0 { 'X' } else { 'O' };
		player_count += 1;

		let game_clone = Arc::clone(&game);
		thread::spawn(move || {
			handle_client(stream, game_clone, player);
		});

		if player_count > 2
		{
			println!("Only 2 players can play...");
			break;
		}
	}
}