// coord
#[derive(Default, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    // todo - remove this and replace with just == impl
    //#define ce(a,b)		((a).x == (b).x && (a).y == (b).y)
    pub fn ce(&self, other: &Coord) -> bool {
	    self.x == other.x && self.y == other.y
    }
}

