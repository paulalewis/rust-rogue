/*struct stats {
    uint s_str;			/* Strength */
    int s_exp;				/* Experience */
    int s_lvl;				/* level of mastery */
    int s_arm;				/* Armor class */
    int s_hpt;			/* Hit points */
    char s_dmg[13];			/* String describing damage done */
    int  s_maxhp;			/* Max hit points */
};*/
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Stats {
    pub str: usize,
    pub exp: usize,
    pub lvl: usize,
    pub arm: isize,
    pub hpt: usize,
    pub dmg: DmgStats,
    pub max_hp: usize,
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            str: 0,
            exp: 0,
            lvl: 0,
            arm: 0,
            hpt: 0,
            dmg: DmgStats::Single(Attack(0, 0)),
            max_hp: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum DmgStats {
    FlyTrap,
    Single(Attack),
    Double(Attack, Attack),
    Triple(Attack, Attack, Attack),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Attack(pub isize, pub isize);

