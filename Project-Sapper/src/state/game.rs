use macroquad::prelude::*;

use crate::field::PlayingField;
use crate::field::cell::CellType;
use crate::state::State;

const CELL_SIZE:   f32 = 32.0;
const CELL_OFFSET: f32 =  1.0;
const BORDER_X:	f32 = 56.0;
const BORDER_Y:	f32 = 60.0;



/// Состояние игрового процесса.
pub struct Game {
	/// Игровое поле.
	field:  PlayingField,
	f_game: u8,
}

impl Default for Game {
	/// Логика создания новой `Game`.
	fn default() -> Self {
		Self::new(32, 20) // Инициализация игры с полем 32x20.
	}
}

impl Game {
	/// Создает новое состояние игрового процесса.
	/// rows - длина строки, cols - высота столбца.
	fn new(rows: usize, cols: usize) -> Self {
		Self {
			field:  PlayingField::new(rows, cols),
			f_game: 0,
		}
	}

	/// Логика обновления приложения.
	/// Возвращает новое состояние, если требуется переключение.
	pub fn update(&mut self) -> Option<State> {
		if self.f_game == 0 && is_mouse_button_pressed(MouseButton::Left) {
			let (mx, my) = mouse_position();
			let x = ((mx - BORDER_X) / (CELL_SIZE + 2 as f32 * CELL_OFFSET)) as usize;
			let y = ((my - BORDER_Y) / (CELL_SIZE + 2 as f32 * CELL_OFFSET)) as usize;

			let p_type = self.field.open(x, y);

			if self.field.open + self.field.mines == self.field.volume {
				self.f_game = 1;
			}
			if let CellType::Mine = p_type {
				self.f_game = 2;
			}
		}

		if self.f_game == 0 && is_mouse_button_pressed(MouseButton::Right) {
			let (mx, my) = mouse_position();
			let x = ((mx - BORDER_X) / (CELL_SIZE + 2 as f32 * CELL_OFFSET)) as usize;
			let y = ((my - BORDER_Y) / (CELL_SIZE + 2 as f32 * CELL_OFFSET)) as usize;

			self.field.mark(x, y);
		}

		if is_key_pressed(KeyCode::Enter) {
			Some(State::default()) // Переход в меню.
		} else {
			None
		}
	}

	/// Отображение приложения.
	pub fn draw(&self) {
		clear_background(DARKGRAY); // Очищаем фон.

		if self.f_game == 0 {
			draw_text("Play! Press Enter to return to the menu.", 20.0, 20.0, 30.0, BLACK);
			draw_text(&format!("Mines: {}", self.field.mines as isize - self.field.flag as isize), 20.0, 40.0, 30.0, BLACK);
		}
		if self.f_game == 1 {
			draw_text("Win! Press Enter to return to the menu.", 20.0, 20.0, 30.0, GREEN);
		}
		if self.f_game == 2 {
			draw_text("Loss! Press Enter to return to the menu.", 20.0, 20.0, 30.0, RED);
		}

		for x in 0..self.field.width {
			for y in 0..self.field.height {
				let screen_x = (x as f32) * (CELL_SIZE + 2 as f32 * CELL_OFFSET) + BORDER_X;
				let screen_y = (y as f32) * (CELL_SIZE + 2 as f32 * CELL_OFFSET) + BORDER_Y;
				let cell = &self.field.field[x][y];

				if cell.p_type == CellType::Null {
					continue;
				}

				if cell.f_open {
					draw_rectangle(
						screen_x + CELL_OFFSET, screen_y + CELL_OFFSET, CELL_SIZE, CELL_SIZE, WHITE,
						);
					match cell.p_type {
						CellType::Null => todo!(),
						CellType::Number(n) => {
							if n > 0 {
								draw_text(&n.to_string(), screen_x + 10.0, screen_y + 25.0, 30.0, DARKGREEN);
							}
						}
						CellType::Mine => {
							draw_text("M", screen_x + 10.0, screen_y + 25.0, 30.0, RED);
						},
					}
				}
				else {
					draw_rectangle(
						screen_x + CELL_OFFSET, screen_y + CELL_OFFSET, CELL_SIZE, CELL_SIZE, GRAY,
						);
					if cell.f_mark {
						draw_text("F", screen_x + 10.0, screen_y + 25.0, 30.0, DARKBLUE);
					}
				}

			}
		}
	}
}