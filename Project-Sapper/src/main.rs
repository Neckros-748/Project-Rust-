#[path = "./scene/field.rs"]
mod field;
#[path = "./state/state.rs"]
mod state;

use macroquad::prelude::*;
use crate::miniquad::window::set_window_size;

use state::State;




// Точка входа в приложение.
// Макрос позволяет сделать функцию main асинхронной, а также иницилизирует окно.
#[macroquad::main("Project (Sapper)")]
async fn main() {
	set_window_size(1200, 800);

	// Инициализирум состояние наший игры по умолчанию.
	let mut state = State::default();

	// Запускаем игровой цикл.
	loop {
		// Обновляем состояние игры.
		state.update();

		// Отображаем игру в окне.
		state.draw();

		// Ожидаем возможности заняться следующим кадром.
		next_frame().await;
	}
}