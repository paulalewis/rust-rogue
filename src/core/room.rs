use super::{constants::{ISDARK, ISGONE}, coord::Coord};

/*struct room {
    coord r_pos;			/* Upper left corner */
    coord r_max;			/* Size of room */
    coord r_gold;			/* Where the gold is */
    int r_goldval;			/* How much the gold is worth */
    short r_flags;			/* info about the room */
    int r_nexits;			/* Number of exits */
    coord r_exit[12];			/* Where the exits are */
};*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Room {
    // upper left corner
    pub pos: Coord, // r_pos
    pub size: Coord, // r_max
    pub gold_location: Coord,
    pub gold_value: usize,
    // info about the room
    pub flags: usize,
    pub nexits: usize,
    // where the exits are
    pub exit: [Coord; 12],
}

impl Room {
    pub fn new() -> Self {
        Room {
            pos: Coord { x: 0, y: 0 },
            size: Coord { x: 0, y: 0 },
            gold_location: Coord { x: 0, y: 0 },
            gold_value: 0,
            flags: ISGONE | ISDARK,
            nexits: 0,
            exit: [Coord { x: 0, y: 0 }; 12],
        }
    }
}
