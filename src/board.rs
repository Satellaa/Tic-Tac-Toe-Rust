use crate::enums::{Cell, Symbol};

#[derive(Clone)]
pub struct Board {
	pub cells: [Cell; 9],
}

impl Board {
	pub fn new() -> Self {
		Board {
			cells: [Cell::Empty; 9],
		}
	}

	pub fn make_move(&mut self, position: usize, symbol: Symbol) -> bool {
		if position < 9 && self.cells[position] == Cell::Empty {
			self.cells[position] = Cell::Occupied(symbol);
			true
		} else {
			false
		}
	}

	pub fn is_full(&self) -> bool {
		self.cells.iter().all(|&cell| cell != Cell::Empty)
	}

	pub fn check_winner(&self) -> Option<Symbol> {
		const WINNING_COMBINATIONS: [[usize; 3]; 8] = [
			[0, 1, 2], [3, 4, 5], [6, 7, 8], // Rows
			[0, 3, 6], [1, 4, 7], [2, 5, 8], // Columns
			[0, 4, 8], [2, 4, 6],		 // Diagonals
		];

		for combo in WINNING_COMBINATIONS.iter() {
			if let (Cell::Occupied(symbol), Cell::Occupied(b), Cell::Occupied(c)) =
				(self.cells[combo[0]], self.cells[combo[1]], self.cells[combo[2]])
			{
				if b == symbol && c == symbol {
					return Some(symbol);
				}
			}
		}

		None
	}

	pub fn get_empty_cells(&self) -> Vec<usize> {
		self.cells
			.iter()
			.enumerate()
			.filter(|(_, &cell)| cell == Cell::Empty)
			.map(|(index, _)| index)
			.collect()
	}
}
