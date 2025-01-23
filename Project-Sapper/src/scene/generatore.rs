use rand::prelude::*;

const ITERATIONS: usize = 3; // Итерации алгоритма клеточного автомата



#[derive(Clone, Copy, PartialEq)]
pub enum Tile {
	Wall,
	Empty,
}

pub struct Generatore {
	pub width:  usize,
	pub height: usize,
	pub grid:   Vec<Vec<Tile>>,

}

impl Generatore {
	pub fn new(width: usize, height: usize) -> Self {
		let mut grid = vec![vec![Tile::Empty; height]; width];
		let mut rng  = thread_rng();

		// Инициализируем поле случайными стенами
		for x in 0..width {
			for y in 0..height {
				if rng.gen_bool(0.45) { // 45% вероятности на стену
					grid[x][y] = Tile::Wall;
				}
			}
		}

		Self { width, height, grid }
	}

	// Генерация пещер (клеточный автомат)
	pub fn generation(&mut self) {
		for _ in 0..ITERATIONS {
			let mut new_grid = self.grid.clone();

			for x in 0..self.width {
				for y in 0..self.height {
					let neighbor_walls = self.count_wall_neighbors(x, y);

					// Если клетка пустая и вокруг больше 4-х стен, то превращаем её в стену
					if self.grid[x][y] == Tile::Empty && neighbor_walls >= 5 {
						new_grid[x][y] = Tile::Wall;
					}
					// Если клетка - стена и вокруг меньше 3-х стен, то превращаем её в пустую клетку
					else if self.grid[x][y] == Tile::Wall && neighbor_walls <= 3 {
						new_grid[x][y] = Tile::Empty;
					}
				}
			}

			self.grid = new_grid;
		}
	}

	// Подсчёт соседей стенок
	fn count_wall_neighbors(&self, x: usize, y: usize) -> usize {
		let mut count = 0;

		for dx in -1..=1 {
			for dy in -1..=1 {
				if dx == 0 && dy == 0 {
					continue; // Пропускаем саму клетку
				}

				let nx = (x as isize + dx) as usize;
				let ny = (y as isize + dy) as usize;

				if nx < self.width && ny < self.height {
					if self.grid[nx][ny] == Tile::Wall {
						count += 1;
					}
				}
				else {
					count += 1;
				}
			}
		}

		count
	}
}