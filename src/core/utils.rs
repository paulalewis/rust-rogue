use rand::RngCore;

use crate::{core::constants::{POTION, SCROLL, FOOD, RING, STICK, WEAPON, ARMOR, STAIRS, GOLD, AMULET, AMULET_LEVEL}, core::coord::Coord};

/// Pick a random number
pub fn rnd(range: usize) -> usize {
    let mut rng = rand::thread_rng();
    return if range == 0 { 0 } else { rng.next_u64() as usize % range };
}

/// Roll a number of dice
pub fn roll(number: usize, sides: usize) -> usize {
    let mut total = 0;
    for _ in 0..number {
        total += rnd(sides) + 1;
    }
    return total;
}

// Returns true if it is ok to step on ch
/// int step_ok(int ch)
pub fn step_ok(character: char) -> bool {
    match character {
        ' ' | '|' | '-' => false,
        _ => !character.is_alphabetic()
    }
}

//#define hero		player.t_pos
//#define pstats		player.t_stats
//#define pack		player.t_pack
//#define proom		player.t_room
//#define max_hp		player.t_stats.s_maxhp

//#define GOLDCALC
pub fn goldcalc(lvl: usize) -> usize {
	rnd(50 + 10 * lvl) + 2
}

//#define ISMULT(type)
fn is_mult(item_type: char) -> bool {
	match item_type {
		POTION | SCROLL | FOOD => true,
		_ => false
	}
}

//#define unc(cp)		(cp).y, (cp).x
pub fn unc(coord: &Coord) -> (usize, usize) {
	(coord.y, coord.x)
}

/*
// A quick glance all around the player
void
look(bool wakeup)
{
    int x, y;
    int ch;
    THING *tp;
    PLACE *pp;
    struct room *rp;
    int ey, ex;
    int passcount;
    char pfl, *fp, pch;
    int sy, sx, sumhero = 0, diffhero = 0;
    passcount = 0;
    rp = proom;
    if (!ce(oldpos, hero))
    {
	erase_lamp(&oldpos, oldrp);
	oldpos = hero;
	oldrp = rp;
    }
    ey = hero.y + 1;
    ex = hero.x + 1;
    sx = hero.x - 1;
    sy = hero.y - 1;
    pp = INDEX(hero.y, hero.x);
    pch = pp->p_ch;
    pfl = pp->p_flags;

    for (y = sy; y <= ey; y++)
	if (y > 0 && y < NUMLINES - 1) for (x = sx; x <= ex; x++)
	{
	    if (x < 0 || x >= NUMCOLS)
		continue;
	    if (!on(player, ISBLIND))
	    {
		if (y == hero.y && x == hero.x)
		    continue;
	    }

	    pp = INDEX(y, x);
	    ch = pp->p_ch;
	    if (ch == ' ')		/* nothing need be done with a ' ' */
		    continue;
	    fp = &pp->p_flags;
	    if (pch != DOOR && ch != DOOR)
		if ((pfl & F_PASS) != (*fp & F_PASS))
		    continue;
	    if (((*fp & F_PASS) || ch == DOOR) && 
		 ((pfl & F_PASS) || pch == DOOR))
	    {
		if (hero.x != x && hero.y != y &&
		    !step_ok(chat(y, hero.x)) && !step_ok(chat(hero.y, x)))
			continue;
	    }

	    if ((tp = pp->p_monst) == NULL)
		ch = trip_ch(y, x, ch);
	    else
		if (on(player, SEEMONST) && on(*tp, ISINVIS))
		{
		    continue;
		}
		else
		{
		    if (wakeup)
			wake_monster(y, x);
		    if (see_monst(tp))
		    {
			if (on(player, ISHALU))
			    ch = rnd(26) + 'A';
			else
			    ch = tp->t_disguise;
		    }
		}
	    if (on(player, ISBLIND) && (y != hero.y || x != hero.x))
		continue;

	    move(y, x);

	    if (tp != NULL || ch != CCHAR( inch() ))
		addch(ch);
	}
    if (!running)
	mvaddch(hero.y, hero.x, PLAYER);
}

// Return the character appropriate for this space, taking into
// account whether or not the player is tripping.
int
trip_ch(int y, int x, int ch)
{
    if (on(player, ISHALU) && after)
	switch (ch)
	{
	    case FLOOR:
	    case ' ':
	    case PASSAGE:
	    case '-':
	    case '|':
	    case DOOR:
	    case TRAP:
		break;
	    default:
		if (y != stairs.y || x != stairs.x || !seenstairs)
		    ch = rnd_thing();
		break;
	}
    return ch;
}

// Erase the area shown by a lamp in a dark room.
void
erase_lamp(coord *pos, struct room *rp)
{
    int y, x, ey, sy, ex;

    if (!((rp->r_flags & (ISGONE|ISDARK)) == ISDARK && !on(player,ISBLIND)))
	    return;

    ey = pos->y + 1;
    ex = pos->x + 1;
    sy = pos->y - 1;
    for (x = pos->x - 1; x <= ex; x++)
	for (y = sy; y <= ey; y++)
	{
	    if (y == hero.y && x == hero.x)
		continue;
	    move(y, x);
	    if (inch() == FLOOR)
		addch(' ');
	}
}

// Find the unclaimed object at y, x
THING *
find_obj(int y, int x)
{
    THING *obj;

    for (obj = lvl_obj; obj != NULL; obj = next(obj))
    {
	if (obj->o_pos.y == y && obj->o_pos.x == x)
		return obj;
    }
    /* NOTREACHED */
    return NULL;
}

void
eat()
{
    THING *obj;

    if ((obj = get_item("eat", FOOD)) == NULL)
	return;
    if (obj->o_type != FOOD)
    {
	    msg("ugh, you would get ill if you ate that");
	return;
    }
    if (food_left < 0)
	food_left = 0;
    if ((food_left += HUNGERTIME - 200 + rnd(400)) > STOMACHSIZE)
	food_left = STOMACHSIZE;
    hungry_state = 0;
    if (obj == cur_weapon)
	cur_weapon = NULL;
    if (obj->o_which == 1)
	msg("my, that was a yummy %s", fruit);
    else
	if (rnd(100) > 70)
	{
	    pstats.s_exp++;
	    msg("%s, this food tastes awful", choose_str("bummer", "yuk"));
	    check_level();
	}
	else
	    msg("%s, that tasted good", choose_str("oh, wow", "yum"));
    leave_pack(obj, FALSE, FALSE);
}

// used to modify the playes strength.  It keeps track of the
// highest it has been, just in case
void
chg_str(int amt)
{
    auto uint comp;

    if (amt == 0)
	return;
    add_str(&pstats.s_str, amt);
    comp = pstats.s_str;
    if (ISRING(LEFT, R_ADDSTR))
	add_str(&comp, -cur_ring[LEFT]->o_arm);
    if (ISRING(RIGHT, R_ADDSTR))
	add_str(&comp, -cur_ring[RIGHT]->o_arm);
    if (comp > max_stats.s_str)
	max_stats.s_str = comp;
}

// Perform the actual add, checking upper and lower bound limits
void
add_str(uint *sp, int amt)
{
    if ((*sp += amt) < 3)
	*sp = 3;
    else if (*sp > 31)
	*sp = 31;
}

// Add a haste to the player
bool
add_haste(bool potion)
{
    if (on(player, ISHASTE))
    {
	no_command += rnd(8);
	player.t_flags &= ~(ISRUN|ISHASTE);
	extinguish(nohaste);
	msg("you faint from exhaustion");
	return FALSE;
    }
    else
    {
	player.t_flags |= ISHASTE;
	if (potion)
	    fuse(nohaste, 0, rnd(4)+4, AFTER);
	return TRUE;
    }
}

// Aggravate all the monsters on this level
void
aggravate()
{
    THING *mp;

    for (mp = mlist; mp != NULL; mp = next(mp))
	runto(&mp->t_pos);
}
*/

/*
// Set up the direction co_ordinate for use in varios "prefix" commands
bool
get_dir()
{
    char *prompt;
    bool gotit;
    static coord last_delt= {0,0};

    if (again && last_dir != '\0')
    {
	delta.y = last_delt.y;
	delta.x = last_delt.x;
	dir_ch = last_dir;
    }
    else
    {
	    msg(prompt = "which direction? ");
	do
	{
	    gotit = TRUE;
	    switch (dir_ch = readchar())
	    {
		case 'h': case'H': delta.y =  0; delta.x = -1;
		when 'j': case'J': delta.y =  1; delta.x =  0;
		when 'k': case'K': delta.y = -1; delta.x =  0;
		when 'l': case'L': delta.y =  0; delta.x =  1;
		when 'y': case'Y': delta.y = -1; delta.x = -1;
		when 'u': case'U': delta.y = -1; delta.x =  1;
		when 'b': case'B': delta.y =  1; delta.x = -1;
		when 'n': case'N': delta.y =  1; delta.x =  1;
		when ESCAPE: last_dir = '\0'; reset_last(); return FALSE;
		otherwise:
		    mpos = 0;
		    msg(prompt);
		    gotit = FALSE;
	    }
	} until (gotit);
	if (isupper(dir_ch))
	    dir_ch = (char) tolower(dir_ch);
	last_dir = dir_ch;
	last_delt.y = delta.y;
	last_delt.x = delta.x;
    }
    if (on(player, ISHUH) && rnd(5) == 0)
	do
	{
	    delta.y = rnd(3) - 1;
	    delta.x = rnd(3) - 1;
	} while (delta.y == 0 && delta.x == 0);
    mpos = 0;
    return TRUE;
}
*/

// Return the sign of the number
// int sign(int nm)
pub fn sign(value: i32) -> i32 {
	if value < 0 { -1 } else if value > 0{ 1 } else { 0 }
}

// Give a spread around a given number (+/- 20%)
pub fn spread(value: usize) -> usize {
	value - value / 20 + rnd(value / 10)
}

/*
// Call an object something after use.
void
call_it(struct obj_info *info)
{
    if (info->oi_know)
    {
	if (info->oi_guess)
	{
	    free(info->oi_guess);
	    info->oi_guess = NULL;
	}
    }
    else if (!info->oi_guess)
    {
	msg("what do you want to call it? ");
	if (get_str(prbuf, stdscr) == NORM)
	{
	    if (info->oi_guess != NULL)
		free(info->oi_guess);
	    info->oi_guess = malloc((unsigned int) strlen(prbuf) + 1);
	    strcpy(info->oi_guess, prbuf);
	}
    }
}
*/

// Pick a random thing appropriate for this level
// rnd_thing()
pub fn rnd_thing(level: usize) -> char {
	let thing_list = [POTION, SCROLL, RING, STICK, FOOD, WEAPON, ARMOR, STAIRS, GOLD, AMULET];
	let i = if level >= AMULET_LEVEL {
		rnd(thing_list.len())
	} else {
		rnd(thing_list.len() - 1)
	};
	thing_list[i]
}

pub fn has_flag(flags: usize, flag: usize) -> bool {
	(flags & flag) == flag
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sign_is_negative() {
		assert_eq!(sign(-100), -1);
    }

    #[test]
	fn sign_is_positive() {
		assert_eq!(sign(100), 1);
	}

    #[test]
	fn sign_is_zero() {
		assert_eq!(sign(0), 0);
	}

	#[test]
	fn has_flag_false() {
		assert_eq!(has_flag(10, 1), false);
	}

	#[test]
	fn has_flag_true() {
		assert_eq!(has_flag(11, 10), true);
	}
}