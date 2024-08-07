use std::io::{self, Write};
use rand::prelude::SliceRandom;
use crate::board::Board;
use crate::enums::{Player, Symbol, GameState, Cell};

pub struct Game {
	board: Board,
	state: GameState,
	human_symbol: Symbol,
	bot_symbol: Symbol,
}

impl Game {
	pub fn new() -> Self {
		let size = Game::choose_board_size();
		let human_symbol = Game::choose_symbol();
		let bot_symbol = if human_symbol == Symbol::X { Symbol::O } else { Symbol::X };

		Game {
			board: Board::new(size),
			state: GameState::Playing(Symbol::X),
			human_symbol,
			bot_symbol,
		}
	}

	fn choose_board_size() -> usize {
		loop {
			println!("Choose board size (3 for 3x3, 5 for 5x5):");
			io::stdout().flush().unwrap();
			
			let mut input = String::new();
			io::stdin().read_line(&mut input).unwrap();
			
			match input.trim() {
				"3" => return 3,
				"5" => return 5,
				_ => println!("Invalid input. Please enter '3' or '5'."),
			}
		}
	}

	fn choose_symbol() -> Symbol {
		loop {
			println!("Do you want to play as X or O?");
			io::stdout().flush().unwrap();

			let mut input = String::new();
			io::stdin().read_line(&mut input).unwrap();

			match input.trim().to_uppercase().as_str() {
				"X" => return Symbol::X,
				"O" => return Symbol::O,
				_ => println!("Invalid input. Please enter 'X' or 'O'."),
			}
		}
	}

	pub fn play(&mut self) {
		println!("You are playing as {:?}", self.human_symbol);
		println!("The bot is playing as {:?}", self.bot_symbol);

		loop {
			self.display_board();

			match self.state {
				GameState::Playing(current_symbol) => {
					let current_player = if current_symbol == self.human_symbol {
						Player::Human
					} else {
						Player::Bot
					};

					match current_player {
						Player::Human => self.human_turn(),
						Player::Bot => self.bot_turn(),
					}

					if let Some(winner) = self.board.check_winner() {
						self.state = GameState::Won(winner);
					} else if self.board.is_full() {
						self.state = GameState::Draw;
					} else {
						self.state = GameState::Playing(if current_symbol == Symbol::X { Symbol::O } else { Symbol::X });
					}
				}
				GameState::Won(symbol) => {
					let winner = if symbol == self.human_symbol { "You win" } else { "Bot wins" };
					println!("{} ({:?})!", winner, symbol);
					break;
				}
				GameState::Draw => {
					println!("It's a draw!");
					break;
				}
			}
		}

		self.display_board();
	}

	fn human_turn(&mut self) {
		loop {
			println!("Your turn. Enter a position (0-{}):", self.board.cells.len() - 1);
			io::stdout().flush().unwrap();

			let mut input = String::new();
			io::stdin().read_line(&mut input).unwrap();

			match input.trim().parse::<usize>() {
				Ok(position) if position < self.board.cells.len() => {
					if self.board.make_move(position, self.human_symbol) {
						break;
					} else {
						println!("Invalid move. The cell is already occupied. Try again.");
					}
				}
				Ok(_) => println!("Invalid input. Please enter a number between 0 and {}.", self.board.cells.len() - 1),
				Err(_) => println!("Invalid input. Please enter a valid number."),
			}
		}
	}

	fn bot_turn(&mut self) {
		println!("Bot's turn...");
		std::thread::sleep(std::time::Duration::from_secs(1)); // Simulating "thinking" time

		let position = self.get_best_move();
		self.board.make_move(position, self.bot_symbol);
		println!("Bot chose position {}", position);
	}

	fn get_best_move(&self) -> usize {
		let empty_cells = self.board.get_empty_cells();
		let mut rng = rand::thread_rng();
		
		let mut best_moves = Vec::new();

		// Check for winning move
		for &pos in &empty_cells {
			let mut test_board = self.board.clone();
			if test_board.make_move(pos, self.bot_symbol) && test_board.check_winner() == Some(self.bot_symbol) {
				return pos;
			}
		}

		// Check for blocking move
		for &pos in &empty_cells {
			let mut test_board = self.board.clone();
			if test_board.make_move(pos, self.human_symbol) && test_board.check_winner() == Some(self.human_symbol) {
				best_moves.push(pos);
			}
		}
		
		if !best_moves.is_empty() {
			return *best_moves.choose(&mut rng).unwrap();
		}

		// Check for fork opportunity
		let fork_position = self.find_fork(self.bot_symbol);
		if let Some(pos) = fork_position {
			best_moves.push(pos);
		}
		
		if !best_moves.is_empty() {
			return *best_moves.choose(&mut rng).unwrap();
		}

		// Block opponent's fork
		let opponent_fork_position = self.find_fork(self.human_symbol);
		if let Some(pos) = opponent_fork_position {
			best_moves.push(pos);
		}
		
		if !best_moves.is_empty() {
			return *best_moves.choose(&mut rng).unwrap();
		}
		
		// If all else fails, choose randomly
		*empty_cells.choose(&mut rng).unwrap()
	}

	fn find_fork(&self, symbol: Symbol) -> Option<usize> {
		let empty_cells = self.board.get_empty_cells();

		for &pos in &empty_cells {
			let mut test_board = self.board.clone();
			if test_board.make_move(pos, symbol) {
				let winning_opportunities = self.count_winning_opportunities(&test_board, symbol);
				if winning_opportunities >= 2 {
					return Some(pos);
				}
			}
		}

		None
	}

	fn count_winning_opportunities(&self, board: &Board, symbol: Symbol) -> usize {
		let mut count = 0;
		let empty_cells = board.get_empty_cells();

		for &pos in &empty_cells {
			let mut test_board = board.clone();
			if test_board.make_move(pos, symbol) && test_board.check_winner() == Some(symbol) {
				count += 1;
			}
		}

		count
	}

	fn display_board(&self) {
		println!("\nCurrent board:");
		for i in 0..self.board.size {
			for j in 0..self.board.size {
				let index = i * self.board.size + j;
				match self.board.cells[index] {
					Cell::Empty => print!("[ ] "),
					Cell::Occupied(Symbol::X) => print!("[X] "),
					Cell::Occupied(Symbol::O) => print!("[O] "),
				}
			}
			println!();
		}
		println!();
	}
}