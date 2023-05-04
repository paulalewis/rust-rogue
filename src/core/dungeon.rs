use crate::rogue::Place;

use super::{object::Object, screen::{SCREEN_HEIGHT, SCREEN_WIDTH}, creature::Creature, coord::Coord};

#[derive(Debug, Clone, PartialEq)]
pub struct Dungeon {
    // level what level the player is on
    pub level: usize,
    // lvl_obj List of objects on this level
    pub objects: Vec<Object>,
    //mlist List of monsters on the level
    pub monsters: Vec<Object>,
    // places[MAXLINES*MAXCOLS] level map 
    pub places: Vec<Place>,
}

impl Dungeon {
    pub fn new() -> Dungeon {
        Dungeon {
            level: 1,
            objects: Vec::new(),
            monsters: Vec::new(),
            places: Vec::new(),
        }
    }

    //#define INDEX(y,x)	(&places[((x) << 5) + (y)])
    fn index(&self, coord: &Coord) -> &Place {
        &self.places[coord.x << 5 + coord.y]
    }

    //#define chat(y,x)	(places[((x) << 5) + (y)].p_ch)
    fn character_at(&self, coord: &Coord) -> char {
	    self.places[coord.x << 5 + coord.y].ch
    }

    //#define flat(y,x)	(places[((x) << 5) + (y)].p_flags)
    fn flag_at(&self, coord: &Coord) -> usize {
        self.places[coord.x << 5 + coord.y].flags
    }

    //#define moat(y,x)	(places[((x) << 5) + (y)].p_monst)
    fn monster_at(&self, coord: &Coord) -> Option<Creature> {
        self.places[coord.x << 5 + coord.y].monst.clone()
    }

    //#define winat(y,x)	(moat(y,x) != NULL ? moat(y,x)->t_disguise : chat(y,x))
    pub fn winat(&self, coord: &Coord) -> char {
	    match self.monster_at(coord) {
		    Some(monster) => monster.disguise,
		    None => self.character_at(coord)
	    }
    }
}