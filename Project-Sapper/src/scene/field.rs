pub mod cell;       // Экспортируем модуль cell.
pub mod generatore; // Экспортируем модуль cell.

use rand::prelude::*;

use crate::field::cell::CellType;
use crate::field::cell::Cell;
use crate::field::generatore::Generatore;
use crate::field::generatore::Tile;



pub struct PlayingField {
	pub width:  usize,
	pub height: usize,
	pub field:  Vec<Vec<Cell>>,

	// pub open:   usize,
	pub flag:   usize,

	// pub volume: usize,
	pub mines:  usize,
}

impl PlayingField {
	pub fn new(width: usize, height: usize) -> Self {
		let mut field = PlayingField {
			width:  width,
			height: height,
			field:  vec![
				vec![
					Cell {
						p_type: CellType::Number(0),
						f_open: false, // Флаг, открытость ячейки
						f_mark: false, // Флаг, маркировка ячейки
					};
					height

				];
				width
			],
			// open:   0,
			flag:   0,
			// volume: 0,
			mines:  0,
		};

		field.mines  = (0.15 * field.generation() as f32) as usize;

		field.set_mines();
		field.set_numbers();
		field
	}

	pub fn open(&mut self, x: usize, y: usize) -> CellType {
		if x >= self.width || y >= self.height || self.field[x][y].f_open || self.field[x][y].f_mark {
			return CellType::Null;
		}
		self.field[x][y].f_open = true;
		// self.open += 1;

		if let CellType::Number(0) = self.field[x][y].p_type {
			for dx in -1..=1 {
				for dy in -1..=1 {
					if dx == 0 && dy == 0 {
						continue;
					}

					let nx = x as isize + dx;
					let ny = y as isize + dy;

					if nx >= 0 && ny >= 0 && (nx as usize) < self.width && (ny as usize) < self.height {
						self.open(nx as usize, ny as usize);
					}
				}
			}
		}

		self.field[x][y].p_type
	}

	pub fn mark(&mut self, x: usize, y: usize) {
		if x >= self.width || y >= self.height || self.field[x][y].f_open {
			return;
		}

		if self.field[x][y].f_mark {
			self.field[x][y].f_mark = false;
			self.flag -= 1;
		}
		else {
			self.field[x][y].f_mark = true;
			self.flag += 1;
		}
	}

	fn generation(&mut self) -> usize {
		let mut generatore_field = Generatore::new(self.width, self.height);
		let mut count = 0;

		generatore_field.generation();
		for x in 0..self.width {
			for y in 0..self.height {
				if generatore_field.grid[x][y] == Tile::Wall {
					self.field[x][y].p_type = CellType::Null;
				}
				if generatore_field.grid[x][y] == Tile::Empty {
					count += 1;
				}
			}
		}

		count
	}

	fn set_mines(&mut self) {
		let mut rng = thread_rng();
		let mut mines_count = 0;

		while self.mines > mines_count {
			let x = rng.gen_range(0..self.width);
			let y = rng.gen_range(0..self.height);

			if let CellType::Number(_) = self.field[x][y].p_type {
				self.field[x][y].p_type = CellType::Mine;
				mines_count += 1;
			}
		}
	}

	fn set_numbers(&mut self) {
		for x in 0..self.width {
			for y in 0..self.height {
				if let CellType::Null = self.field[x][y].p_type {
					continue;
				}
				if let CellType::Mine = self.field[x][y].p_type {
					continue;
				}

				let mut count = 0;
				for dx in -1..=1 {
					for dy in -1..=1 {
						if dx == 0 && dy == 0 {
							continue;
						}

						let nx = x as isize + dx;
						let ny = y as isize + dy;

						if nx >= 0 && ny >= 0 && (nx as usize) < self.width && (ny as usize) < self.height {
							if let CellType::Mine = self.field[nx as usize][ny as usize].p_type {
								count += 1;
							}
						}
					}
				}

				self.field[x][y].p_type = CellType::Number(count);
			}
		}
	}
}