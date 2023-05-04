/*struct stats {
    uint s_str;			/* Strength */
    int s_exp;				/* Experience */
    int s_lvl;				/* level of mastery */
    int s_arm;				/* Armor class */
    int s_hpt;			/* Hit points */
    char s_dmg[13];			/* String describing damage done */
    int  s_maxhp;			/* Max hit points */
};*/
#[derive(Debug, Clone, PartialEq)]
pub struct Stats {
    pub str: usize,
    pub exp: usize,
    pub lvl: usize,
    pub arm: isize,
    pub hpt: usize,
    pub dmg: String,
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
            dmg: String::new(),
            max_hp: 0,
        }
    }
}

