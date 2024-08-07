mod game;
mod board;
mod enums;

use game::Game;

fn main() {
	println!("Welcome to Rust Tic-Tac-Toe with Bot!");
	let mut game = Game::new();
	game.play();
}