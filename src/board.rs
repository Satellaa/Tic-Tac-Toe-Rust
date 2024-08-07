use crate::enums::{Cell, Symbol};

#[derive(Clone)]
pub struct Board {
	pub cells: Vec<Cell>,
	pub size: usize,
}

impl Board {
	pub fn new(size: usize) -> Self {
		Board {
			cells: vec![Cell::Empty; size * size],
			size,
		}
	}

	pub fn make_move(&mut self, position: usize, symbol: Symbol) -> bool {
		if position < self.cells.len() && self.cells[position] == Cell::Empty {
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
		let win_length = if self.size == 3 { 3 } else { 4 };

		// Check rows and columns
		for i in 0..self.size {
			if let Some(symbol) = self.check_line(i * self.size, 1, win_length) {
				return Some(symbol);
			}
			if let Some(symbol) = self.check_line(i, self.size, win_length) {
				return Some(symbol);
			}
		}

		// Check diagonals
		if let Some(symbol) = self.check_line(0, self.size + 1, win_length) {
			return Some(symbol);
		}
		if let Some(symbol) = self.check_line(self.size - 1, self.size - 1, win_length) {
			return Some(symbol);
		}

		// Check additional diagonals for 5x5
		if self.size == 5 {
			if let Some(symbol) = self.check_line(1, self.size + 1, win_length) {
				return Some(symbol);
			}
			if let Some(symbol) = self.check_line(self.size, self.size - 1, win_length) {
				return Some(symbol);
			}
		}

		None
	}

	fn check_line(&self, start: usize, step: usize, win_length: usize) -> Option<Symbol> {
		let mut count = 0;
		let mut current_symbol: Option<Symbol> = None;

		for i in 0..self.size {
			let index = start + i * step;
			if index >= self.cells.len() {
				break;
			}

			match self.get_symbol(index) {
				Some(symbol) => {
					if Some(symbol) == current_symbol {
						count += 1;
						if count == win_length {
							return current_symbol;
						}
					} else {
						current_symbol = Some(symbol);
						count = 1;
					}
				}
				None => {
					current_symbol = None;
					count = 0;
				}
			}
		}

		None
	}
	
	fn get_symbol(&self, index: usize) -> Option<Symbol> {
		if index < self.cells.len() {
			match self.cells[index] {
				Cell::Occupied(symbol) => Some(symbol),
				Cell::Empty => None,
			}
		} else {
			None
		}
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
