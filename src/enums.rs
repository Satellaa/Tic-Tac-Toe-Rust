#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Player {
	Human,
	Bot,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Symbol {
	X,
	O,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
	Empty,
	Occupied(Symbol),
}

pub enum GameState {
	Playing(Symbol),
	Won(Symbol),
	Draw,
}