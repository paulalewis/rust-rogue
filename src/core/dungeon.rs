use super::object::Object;

#[derive(Debug, Clone, PartialEq)]
pub struct Dungeon {
    // level what level the player is on
    pub level: usize,
    // lvl_obj List of objects on this level
    pub objects: Vec<Object>,
    //mlist List of monsters on the level
    pub monsters: Vec<Object>,
}

impl Dungeon {
    pub fn new() -> Dungeon {
        Dungeon {
            level: 1,
            objects: Vec::new(),
            monsters: Vec::new(),
        }
    }
}