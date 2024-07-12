/// an unsigend coordinate on map
/// coord
#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Coord {
    pub x: usize,
    pub y: usize,
}

impl Coord {
    // todo - remove this and replace with just == impl
    //#define ce(a,b)		((a).x == (b).x && (a).y == (b).y)
    // pub fn ce(&self, other: &Coord) -> bool {
	//     self.x == other.x && self.y == other.y
    // }
    
    pub fn inc_x(&self) -> Coord {
        Coord { x: self.x + 1, y: self.y }
    }
    
    pub fn inc_y(&self) -> Coord {
        Coord { x: self.x, y: self.y + 1 }
    }

    pub fn dec_x(&self) -> Coord {
        Coord { x: self.x - 1, y: self.y }
    }

    pub fn dec_y(&self) -> Coord {
        Coord { x: self.x, y: self.y - 1 }
    }
}

