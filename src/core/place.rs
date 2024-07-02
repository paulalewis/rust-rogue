use super::creature::Creature;

/*typedef struct {
    char p_ch;
    char p_flags;
    THING *p_monst;
} PLACE;*/
/// describe a place on the level map
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Place {
    pub ch: char,
    pub flags: usize,
    pub monst: Option<Creature>,
}

impl Place {
    pub fn new() -> Self {
        Place {
            ch: '\0',
            flags: 0,
            monst: None,
        }
    }
}
