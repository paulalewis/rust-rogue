use super::coord::Coord;

/*
 * Room structure
 */
/*struct room {
    coord r_pos;			/* Upper left corner */
    coord r_max;			/* Size of room */
    coord r_gold;			/* Where the gold is */
    int r_goldval;			/* How much the gold is worth */
    short r_flags;			/* info about the room */
    int r_nexits;			/* Number of exits */
    coord r_exit[12];			/* Where the exits are */
};*/
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Room {
    pub pos: Coord,
    pub max: Coord,
    pub gold: Coord,
    pub goldval: i32,
    pub flags: usize,
    pub nexits: i32,
    pub exit: [Coord; 12],
}

impl Room {
    pub fn new() -> Self {
        Room {
            pos: Coord { x: 0, y: 0 },
            max: Coord { x: 0, y: 0 },
            gold: Coord { x: 0, y: 0 },
            goldval: 0,
            flags: 0,
            nexits: 0,
            exit: [Coord { x: 0, y: 0 }; 12],
        }
    }
}
