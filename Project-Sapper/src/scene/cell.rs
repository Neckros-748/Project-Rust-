#[derive(Copy, Clone, PartialEq)]
pub enum CellType {
	Null,
	Number(u8),
	Mine,
}

#[derive(Copy, Clone)]
pub struct Cell {
	pub p_type: CellType,
	pub f_open: bool, // Флаг, открытость ячейки
	pub f_mark: bool, // Флаг, маркировка ячейки
}