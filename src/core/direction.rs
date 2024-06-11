#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Direction {
	Left,
	Down,
	Up,
	Right,
	UpLeft,
	UpRight,
	DownLeft,
	DownRight,
}
