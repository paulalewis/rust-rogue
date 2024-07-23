use super::{constants::{ISGONE, MAXROOMS}, coord::Coord, creature::Creature, object::Object, place::Place, room::Room, utils::rnd};

pub const MAP_HEIGHT: usize = 24; // NUMLINES
pub const MAP_WIDTH: usize = 80; // NUMCOLS

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Dungeon {
    // dungeon level
    pub level: usize,
    // lvl_obj List of objects on this level
    pub objects: Vec<Object>,
    //mlist List of monsters on the level
    pub monsters: Vec<Object>,
    // places[MAXLINES*MAXCOLS] level map 
    pub places: Vec<Vec<Place>>,
    // rooms[MAXROOMS] One for each room -- A level
    pub rooms: [Room; MAXROOMS],
}

impl Dungeon {
    pub fn new() -> Dungeon {
        Dungeon {
            level: 1,
            objects: Vec::new(),
            monsters: Vec::new(),
            places: vec![vec![Place::new(); MAP_HEIGHT]; MAP_WIDTH],
            rooms: [Room::new(); MAXROOMS],
        }
    }

    //#define INDEX(y,x)	(&places[((x) << 5) + (y)])
    pub fn place_at(&mut self, coord: Coord) -> &mut Place {
        &mut self.places[coord.x][coord.y]
    }

    //#define chat(y,x)	(places[((x) << 5) + (y)].p_ch)
    pub fn character_at(&self, coord: Coord) -> char {
        self.places[coord.x][coord.y].ch
    }

    //#define flat(y,x)	(places[((x) << 5) + (y)].p_flags)
    pub fn flag_at(&self, coord: Coord) -> usize {
        self.places[coord.x][coord.y].flags
    }

    //#define moat(y,x)	(places[((x) << 5) + (y)].p_monst)
    pub fn monster_at(&self, coord: Coord) -> Option<Creature> {
        self.places[coord.x][coord.y].monst.clone()
    }

    //#define winat(y,x)	(moat(y,x) != NULL ? moat(y,x)->t_disguise : chat(y,x))
    pub fn winat(&self, coord: Coord) -> char {
	    match self.monster_at(coord) {
		    Some(monster) => monster.disguise,
		    None => self.character_at(coord)
	    }
    }

    // Pick a room that is really there
    // int rnd_room()
    pub fn rnd_room_index(&self) -> usize {
        loop {
            let index = rnd(self.rooms.len());
            if self.rooms[index].flags & ISGONE == 0 {
                break index;
            }
        }
    }
}