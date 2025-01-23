pub mod game; // Экспортируем модуль game.
pub mod menu; // Экспортируем модуль menu.

use crate::state::game::Game; // Подключаем локальный модуль game.
use crate::state::menu::Menu; // Подключаем локальный модуль menu.



/// Состояние приложения.
pub enum State {
	Game(Game),
	Menu(Menu),
}

impl Default for State {
	fn default() -> Self {
		Self::Menu(Menu::default()) // Изначально находимся в меню.
	}
}

impl State {
	/// Логика обновления приложения.
	pub fn update(&mut self) {
		match self {
			State::Game(game) => {
				// Логика обновления для игры.
				if let Some(new_state) = game.update() {
					*self = new_state; // Переключение состояния.
				}
				// println!("Обновляем игру...");
			}
			State::Menu(menu) => {
				// Логика обновления для меню.
				if let Some(new_state) = menu.update() {
					*self = new_state; // Переключение состояния.
				}
				// println!("Обновляем меню...");
			}
		}
	}

	/// Отображение приложения.
	pub fn draw(&self) {
		match self {
			State::Game(game) => {
				game.draw(); // Логика отображения для игры.
				// println!("Отображаем игру...");
			}
			State::Menu(menu) => {
				menu.draw(); // Логика отображения для меню.
				// println!("Отображаем меню...");
			}
		}
	}

	// /// Переключение состояния на `Menu`.
	// pub fn switch_to_menu(&mut self, rows: usize, cols: usize) {
	// 	*self = State::Menu(Menu::new(rows, cols));
	// 	println!("Переключение на меню.");
	// }

	// /// Переключение состояния на `Game`.
	// pub fn switch_to_game(&mut self, rows: usize, cols: usize) {
	// 	*self = State::Game(Game::new(rows, cols));
	// 	println!("Переключение на игру.");
	// }

	// /// Универсальный метод для изменения состояния.
	// pub fn set_state(&mut self, new_state: State) {
	// 	*self = new_state;
	// 	println!("Состояние изменено.");
	// }
}