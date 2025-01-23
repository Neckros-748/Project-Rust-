use macroquad::prelude::*;

use crate::state::State;
use crate::state::Game;



/// Состояние меню.
pub struct Menu {
	/// Двумерный массив, представляющий поле.
	field: Vec<Vec<u8>>,
}

impl Default for Menu {
	/// Логика создания нового `Menu`.
	fn default() -> Self {
		Self::new(100, 100) // Инициализация меню с полем 100x100.
	}
}

impl Menu {
	/// Создает новое состояние меню.
	/// rows - длина строки, cols - высота столбца.
	fn new(rows: usize, cols: usize) -> Self {
		Self {
			field: vec![vec![0; cols]; rows], // Инициализация нулями.
		}
	}

	/// Логика обновления приложения.
	/// Возвращает новое состояние, если требуется переключение.
	pub fn update(&mut self) -> Option<State> {
		// Если нажат Enter - переключаемся на игровое состояние.
		if is_key_pressed(KeyCode::Enter) {
			Some(State::Game(Game::default())) // Переход в игру.
		} else {
			None
		}
	}

	/// Отображение приложения.
	pub fn draw(&self) {
		clear_background(DARKGRAY); // Очищаем фон.

		draw_text("Press Enter to start the game", 20.0, 20.0, 30.0, BLACK);
	}
}