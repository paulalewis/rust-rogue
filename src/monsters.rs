use crate::{constants::{AMULETLEVEL, LEFT, RIGHT, R_PROTECT, VS_MAGIC}, core::{coord::Coord, creature::Creature, monster::{Monster, MONSTERS, NUMBER_OF_MONSTERS}, rogue_state::RogueState}, rogue::*, utils::*};

use std::cmp;

// List of monsters in rough order of vorpalness
static lvl_mons: [char; NUMBER_OF_MONSTERS] = [
    'K', 'E', 'B', 'S', 'H', 'I', 'R', 'O', 'Z', 'L', 'C', 'Q', 'A', 'N', 'Y', 'F', 'T', 'W', 'P', 'X', 'U', 'M', 'V', 'G', 'J', 'D'
];

static wand_mons: [char; NUMBER_OF_MONSTERS] = [
    'K', 'E', 'B', 'S', 'H', ' ', 'R', 'O', 'Z', ' ', 'C', 'Q', 'A', ' ', 'Y', ' ', 'T', 'W', 'P', ' ', 'U', 'M', 'V', 'G', 'J', ' '
];

// Pick a monster to show up.  The lower the level,
// the meaner the monster.
//char randmonster(bool wander) {}
pub fn randmonster(level: usize, wander: bool) -> char {
    let mut selected_monster: usize;
    let mons: [char; NUMBER_OF_MONSTERS] = if wander { wand_mons } else { lvl_mons };
    loop {
        selected_monster = level + (rnd(10) - 6);
        if selected_monster < 0 {
            selected_monster = rnd(5);
        }
        if selected_monster > 25 {
            selected_monster = rnd(5) + 21;
        }
        if mons[selected_monster] != ' ' {
            break;
        }
    }
    mons[selected_monster]
}

// Pick a new monster and add it to the list
/*void
new_monster(THING *tp, char type, coord *cp)
{
    struct monster *mp;
    int lev_add;

    if ((lev_add = level - AMULETLEVEL) < 0)
	lev_add = 0;
    attach(mlist, tp);
    tp->t_type = type;
    tp->t_disguise = type;
    tp->t_pos = *cp;
    move(cp->y, cp->x);
    tp->t_oldch = CCHAR( inch() );
    tp->t_room = roomin(cp);
    moat(cp->y, cp->x) = tp;
    mp = &monsters[tp->t_type-'A'];
    tp->t_stats.s_lvl = mp->m_stats.s_lvl + lev_add;
    tp->t_stats.s_maxhp = tp->t_stats.s_hpt = roll(tp->t_stats.s_lvl, 8);
    tp->t_stats.s_arm = mp->m_stats.s_arm - lev_add;
    strcpy(tp->t_stats.s_dmg,mp->m_stats.s_dmg);
    tp->t_stats.s_str = mp->m_stats.s_str;
    tp->t_stats.s_exp = mp->m_stats.s_exp + lev_add * 10 + exp_add(tp);
    tp->t_flags = mp->m_flags;
    if (level > 29)
	tp->t_flags |= ISHASTE;
    tp->t_turn = TRUE;
    tp->t_pack = NULL;
    if (ISWEARING(R_AGGR))
	runto(cp);
    if (type == 'X')
	tp->t_disguise = rnd_thing();
}*/
pub fn new_monster(state: &RogueState, tp: Box<Creature>, m_type: char, cp: Option<Coord>) {
    let mut mp: Monster;
    let mut tp: Creature;
    let mut cp: Coord;
    // let mut mlist: Vec<Thing>;

    let lev_add = cmp::max(0, state.dungeon.level as isize - AMULETLEVEL as isize);

    /*attach(mlist, tp);
    tp.t_type = m_type;
    tp.t_disguise = m_type;
    tp.t_pos = cp;
    move(cp.y, cp.x);
    tp.t_oldch = inch();
    tp.t_room = roomin(cp);
    moat(cp.y, cp.x) = tp;
    mp = &monsters[tp.t_type - 'A'];
    tp.t_stats.s_lvl = mp.m_stats.s_lvl + lev_add;
    tp.t_stats.s_maxhp = tp.t_stats.s_hpt = roll(tp.t_stats.s_lvl, 8);
    tp.t_stats.s_arm = mp.m_stats.s_arm - lev_add;
    strcpy(tp.t_stats.s_dmg, mp.m_stats.s_dmg);
    tp.t_stats.s_str = mp.m_stats.s_str;
    tp.t_stats.s_exp = mp.m_stats.s_exp + lev_add * 10 + exp_add(tp);
    tp.t_flags = mp.m_flags;
    if (level > 29) {
        tp.t_flags |= ISHASTE;
    }
    tp.t_turn = TRUE;
    tp.t_pack = NULL;
    if (ISWEARING(R_AGGR)) {
        runto(cp);
    }
    if (type == 'X') {
        tp.t_disguise = rnd_thing();
    }*/
}

// Experience to add for this monster's level/hit points
//int exp_add(THING *tp)
pub fn exp_add(monster: &Creature) -> usize {
    let mut xp: usize;
    xp = monster.stats.max_hp / if monster.stats.lvl == 1 { 8 } else { 6 };
    xp *= if monster.stats.lvl > 9 { 20 } else if monster.stats.lvl > 6 { 4 } else { 1 };
    xp
}

/*
// Create a new wandering monster and aim it at the player
void
wanderer()
{
    THING *tp;
    static coord cp;

    tp = new_item();
    do
    {
	find_floor((struct room *) NULL, &cp, FALSE, TRUE);
    } while (roomin(&cp) == proom);
    new_monster(tp, randmonster(TRUE), &cp);
    if (on(player, SEEMONST))
    {
	standout();
	if (!on(player, ISHALU))
	    addch(tp->t_type);
	else
	    addch(rnd(26) + 'A');
	standend();
    }
    runto(&tp->t_pos);
}

/*
 * wake_monster:
 *	What to do when the hero steps next to a monster
 */
THING *
wake_monster(int y, int x)
{
    THING *tp;
    struct room *rp;
    char ch, *mname;

    tp = moat(y, x);
    if (tp == NULL) 	 	 
	endwin(), abort(); 
    ch = tp->t_type;
    /*
     * Every time he sees mean monster, it might start chasing him
     */
    if (!on(*tp, ISRUN) && rnd(3) != 0 && on(*tp, ISMEAN) && !on(*tp, ISHELD)
	&& !ISWEARING(R_STEALTH) && !on(player, ISLEVIT))
    {
	tp->t_dest = &hero;
	tp->t_flags |= ISRUN;
    }
    if (ch == 'M' && !on(player, ISBLIND) && !on(player, ISHALU)
	&& !on(*tp, ISFOUND) && !on(*tp, ISCANC) && on(*tp, ISRUN))
    {
        rp = proom;
	if ((rp != NULL && !(rp->r_flags & ISDARK))
	    || dist(y, x, hero.y, hero.x) < LAMPDIST)
	{
	    tp->t_flags |= ISFOUND;
	    if (!save(VS_MAGIC))
	    {
		if (on(player, ISHUH))
		    lengthen(unconfuse, spread(HUHDURATION));
		else
		    fuse(unconfuse, 0, spread(HUHDURATION), AFTER);
		player.t_flags |= ISHUH;
		mname = set_mname(tp);
		addmsg("%s", mname);
		if (strcmp(mname, "it") != 0)
		    addmsg("'");
		msg("s gaze has confused you");
	    }
	}
    }
    /*
     * Let greedy ones guard gold
     */
    if (on(*tp, ISGREED) && !on(*tp, ISRUN))
    {
	tp->t_flags |= ISRUN;
	if (proom->r_goldval)
	    tp->t_dest = &proom->r_gold;
	else
	    tp->t_dest = &hero;
    }
    return tp;
}
*/

// Give a pack to a monster if it deserves one
// void give_pack(THING *tp)
pub fn give_pack(state: &RogueState, creature: Creature) {
    if state.dungeon.level >= state.max_level {
        let index = (creature.creature_type as u8 - b'A') as usize;
        if rnd(100) < MONSTERS[index].carry {
            // todo - attach(pack, new_thing());
        }
    }
}

// See if a creature save against something
//int save_throw(int which, THING *tp)
pub fn save_throw(which: usize, creature: &Creature) -> bool {
    let need: usize = 14 + which - creature.stats.lvl / 2;
    roll(1, 20) >= need
}
