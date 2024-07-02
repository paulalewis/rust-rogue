use crate::constants::{ISFLY, ISGREED, ISINVIS, ISMEAN, ISREGEN};

use super::stats::{Attack, DmgStats, Stats};

/*struct monster {
    char *m_name;			/* What to call the monster */
    int m_carry;			/* Probability of carrying something */
    short m_flags;			/* things about the monster */
    struct stats m_stats;		/* Initial stats */
};*/
pub struct Monster {
    pub name: &'static str,
    pub carry: usize,
    pub flags: usize,
    pub stats: Stats,
}

pub const NUMBER_OF_MONSTERS: usize = 26;
/// monsters Array containing information on all the various types of monsters
pub static MONSTERS: [Monster; NUMBER_OF_MONSTERS] = [
    Monster { name: "aquator", carry: 0, flags: ISMEAN, stats: Stats { str: 10, exp: 20, lvl: 5, arm: 2, hpt: 1, dmg: DmgStats::Double(Attack(0, 0), Attack(0, 0)), max_hp: 0 } },
    Monster { name: "bat", carry: 0, flags: ISFLY, stats: Stats { str: 10, exp: 1, lvl: 1, arm: 3, hpt: 1, dmg: DmgStats::Single(Attack(1, 2)), max_hp: 0 } },
    Monster { name: "centaur", carry: 15, flags: 0, stats: Stats { str: 10, exp: 17, lvl: 4, arm: 4, hpt: 1, dmg: DmgStats::Triple(Attack(1, 2), Attack(1, 5), Attack(1, 5)), max_hp: 0 } },
    Monster { name: "dragon", carry: 100, flags: ISMEAN, stats: Stats { str: 10, exp: 5000, lvl: 10, arm: -1, hpt: 0, dmg: DmgStats::Triple(Attack(1, 8), Attack(1, 8), Attack(3, 10)), max_hp: 0 } },
    Monster { name: "emu", carry: 0, flags: ISMEAN, stats: Stats { str: 10, exp: 2, lvl: 1, arm: 7, hpt: 1, dmg: DmgStats::Single(Attack(1, 2)), max_hp: 0 } },
    Monster { name: "venus flytrap", carry: 0, flags: ISMEAN, stats: Stats { str: 10, exp: 80, lvl: 8, arm: 3, hpt: 1, dmg: DmgStats::FlyTrap, max_hp: 0 } },
    Monster { name: "griffin", carry: 20, flags: ISMEAN | ISFLY | ISREGEN, stats: Stats { str: 10, exp: 2000, lvl: 13, arm: 2, hpt: 1, dmg: DmgStats::Double(Attack(4, 3), Attack(3, 5)), max_hp: 0 } },
    Monster { name: "hobgablin", carry: 0, flags: ISMEAN, stats: Stats { str: 10, exp: 3, lvl: 1, arm: 5, hpt: 1, dmg: DmgStats::Single(Attack(1, 8)), max_hp: 0 } },
    Monster { name: "ice monster", carry: 0, flags: 0, stats: Stats { str: 10, exp: 5, lvl: 1, arm: 9, hpt: 1, dmg: DmgStats::Single(Attack(0, 0)), max_hp: 0 } },
    Monster { name: "jabberwock", carry: 70, flags: 0, stats: Stats { str: 10, exp: 3000, lvl: 15, arm: 6, hpt: 1, dmg: DmgStats::Double(Attack(2, 12), Attack(2, 4)), max_hp: 0 } },
    Monster { name: "kestrel", carry: 0, flags: ISMEAN | ISFLY, stats: Stats { str: 10, exp: 1, lvl: 1, arm: 7, hpt: 1, dmg: DmgStats::Single(Attack(1, 4)), max_hp: 0 } },
    Monster { name: "leprechaun", carry: 0, flags: 0, stats: Stats { str: 10, exp: 10, lvl: 3, arm: 8, hpt: 1, dmg: DmgStats::Single(Attack(1, 1)), max_hp: 0 } },
    Monster { name: "medusa", carry: 40, flags: ISMEAN, stats: Stats { str: 10, exp: 200, lvl: 8, arm: 2, hpt: 1, dmg: DmgStats::Triple(Attack(3, 4), Attack(3, 4), Attack(2, 5)), max_hp: 0 } },
    Monster { name: "nymph", carry: 100, flags: 0, stats: Stats { str: 10, exp: 37, lvl: 3, arm: 9, hpt: 1, dmg: DmgStats::Single(Attack(0, 0)), max_hp: 0 } },
    Monster { name: "orc", carry: 15, flags: ISGREED, stats: Stats { str: 10, exp: 5, lvl: 1, arm: 6, hpt: 1, dmg: DmgStats::Single(Attack(1, 8)), max_hp: 0 } },
    Monster { name: "phantom", carry: 0, flags: ISINVIS, stats: Stats { str: 10, exp: 120, lvl: 8, arm: 3, hpt: 1, dmg: DmgStats::Single(Attack(4, 4)), max_hp: 0 } },
    Monster { name: "quagga", carry: 0, flags: ISMEAN, stats: Stats { str: 10, exp: 15, lvl: 3, arm: 3, hpt: 1, dmg: DmgStats::Double(Attack(1, 5), Attack(1, 5)), max_hp: 0 } },
    Monster { name: "rattlesnake", carry: 0, flags: ISMEAN, stats: Stats { str: 10, exp: 9, lvl: 2, arm: 3, hpt: 1, dmg: DmgStats::Single(Attack(1, 6)), max_hp: 0 } },
    Monster { name: "snake", carry: 0, flags: ISMEAN, stats: Stats { str: 10, exp: 2, lvl: 1, arm: 5, hpt: 1, dmg: DmgStats::Single(Attack(1, 3)), max_hp: 0 } },
    Monster { name: "troll", carry: 50, flags: ISREGEN | ISMEAN, stats: Stats { str: 10, exp: 120, lvl: 6, arm: 4, hpt: 1, dmg: DmgStats::Triple(Attack(1, 8), Attack(1, 8), Attack(2, 6)), max_hp: 0 } },
    Monster { name: "black unicorn", carry: 0, flags: ISMEAN, stats: Stats { str: 10, exp: 190, lvl: 7, arm: -2, hpt: 1, dmg: DmgStats::Triple(Attack(1, 9), Attack(1, 9), Attack(2, 9)), max_hp: 0 } },
    Monster { name: "vampire", carry: 20, flags: ISREGEN | ISMEAN, stats: Stats { str: 10, exp: 350, lvl: 8, arm: 1, hpt: 1, dmg: DmgStats::Single(Attack(1, 10)), max_hp: 0 } },
    Monster { name: "wraith", carry: 0, flags: 0, stats: Stats { str: 10, exp: 55, lvl: 5, arm: 4, hpt: 1, dmg: DmgStats::Single(Attack(1, 6)), max_hp: 0 } },
    Monster { name: "xeroc", carry: 30, flags: 0, stats: Stats { str: 10, exp: 100, lvl: 7, arm: 7, hpt: 1, dmg: DmgStats::Single(Attack(4, 4)), max_hp: 0 } },
    Monster { name: "yeti", carry: 30, flags: 0, stats: Stats { str: 10, exp: 50, lvl: 4, arm: 6, hpt: 1, dmg: DmgStats::Double(Attack(1, 6), Attack(1, 6)), max_hp: 0 } },
    Monster { name: "zombie", carry: 0, flags: ISMEAN, stats: Stats { str: 10, exp: 6, lvl: 2, arm: 8, hpt: 1, dmg: DmgStats::Single(Attack(1, 8)), max_hp: 0 } },
];