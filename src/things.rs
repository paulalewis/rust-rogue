use crate::core::{constants::ISCURSED, object::Object, object_type::{FoodType, ObjectType, RingType}, rogue_state::RogueState, stats::{Attack, DmgStats}, utils::rnd};

/*
/*
 * inv_name:
 *	Return the name of something as it would appear in an
 *	inventory.
 */
char *
inv_name(THING *obj, bool drop)
{
    char *pb;
    struct obj_info *op;
    char *sp;
    int which;

    pb = prbuf;
    which = obj->o_which;
    switch (obj->o_type)
    {
        case POTION:
	    nameit(obj, "potion", p_colors[which], &pot_info[which], nullstr);
	when RING:
	    nameit(obj, "ring", r_stones[which], &ring_info[which], ring_num);
	when STICK:
	    nameit(obj, ws_type[which], ws_made[which], &ws_info[which], charge_str);
	when SCROLL:
	    if (obj->o_count == 1)
	    {
		strcpy(pb, "A scroll ");
		pb = &prbuf[9];
	    }
	    else
	    {
		sprintf(pb, "%d scrolls ", obj->o_count);
		pb = &prbuf[strlen(prbuf)];
	    }
	    op = &scr_info[which];
	    if (op->oi_know)
		sprintf(pb, "of %s", op->oi_name);
	    else if (op->oi_guess)
		sprintf(pb, "called %s", op->oi_guess);
	    else
		sprintf(pb, "titled '%s'", s_names[which]);
	when FOOD:
	    if (which == 1)
		if (obj->o_count == 1)
		    sprintf(pb, "A%s %s", vowelstr(fruit), fruit);
		else
		    sprintf(pb, "%d %ss", obj->o_count, fruit);
	    else
		if (obj->o_count == 1)
		    strcpy(pb, "Some food");
		else
		    sprintf(pb, "%d rations of food", obj->o_count);
	when WEAPON:
	    sp = weap_info[which].oi_name;
	    if (obj->o_count > 1)
		sprintf(pb, "%d ", obj->o_count);
	    else
		sprintf(pb, "A%s ", vowelstr(sp));
	    pb = &prbuf[strlen(prbuf)];
	    if (obj->o_flags & ISKNOW)
		sprintf(pb, "%s %s", num(obj->o_hplus,obj->o_dplus,WEAPON), sp);
	    else
		sprintf(pb, "%s", sp);
	    if (obj->o_count > 1)
		strcat(pb, "s");
	    if (obj->o_label != NULL)
	    {
		pb = &prbuf[strlen(prbuf)];
		sprintf(pb, " called %s", obj->o_label);
	    }
	when ARMOR:
	    sp = arm_info[which].oi_name;
	    if (obj->o_flags & ISKNOW)
	    {
		sprintf(pb, "%s %s [",
		    num(a_class[which] - obj->o_arm, 0, ARMOR), sp);
		    strcat(pb, "protection ");
		pb = &prbuf[strlen(prbuf)];
		sprintf(pb, "%d]", 10 - obj->o_arm);
	    }
	    else
		sprintf(pb, "%s", sp);
	    if (obj->o_label != NULL)
	    {
		pb = &prbuf[strlen(prbuf)];
		sprintf(pb, " called %s", obj->o_label);
	    }
	when AMULET:
	    strcpy(pb, "The Amulet of Yendor");
	when GOLD:
	    sprintf(prbuf, "%d Gold pieces", obj->o_goldval);
    }
    if (inv_describe)
    {
	if (obj == cur_armor)
	    strcat(pb, " (being worn)");
	if (obj == cur_weapon)
	    strcat(pb, " (weapon in hand)");
	if (obj == cur_ring[LEFT])
	    strcat(pb, " (on left hand)");
	else if (obj == cur_ring[RIGHT])
	    strcat(pb, " (on right hand)");
    }
    if (drop && isupper(prbuf[0]))
	prbuf[0] = (char) tolower(prbuf[0]);
    else if (!drop && islower(*prbuf))
	*prbuf = (char) toupper(*prbuf);
    prbuf[MAXSTR-1] = '\0';
    return prbuf;
}

/*
 * drop:
 *	Put something down
 */

void
drop()
{
    char ch;
    THING *obj;

    ch = chat(hero.y, hero.x);
    if (ch != FLOOR && ch != PASSAGE)
    {
	after = FALSE;
	msg("there is something there already");
	return;
    }
    if ((obj = get_item("drop", 0)) == NULL)
	return;
    if (!dropcheck(obj))
	return;
    obj = leave_pack(obj, TRUE, (bool)!ISMULT(obj->o_type));
    /*
     * Link it into the level object list
     */
    attach(lvl_obj, obj);
    chat(hero.y, hero.x) = (char) obj->o_type;
    flat(hero.y, hero.x) |= F_DROPPED;
    obj->o_pos = hero;
    if (obj->o_type == AMULET)
	amulet = FALSE;
    msg("dropped %s", inv_name(obj, TRUE));
}

/*
 * dropcheck:
 *	Do special checks for dropping or unweilding|unwearing|unringing
 */
bool
dropcheck(THING *obj)
{
    if (obj == NULL)
	return TRUE;
    if (obj != cur_armor && obj != cur_weapon
	&& obj != cur_ring[LEFT] && obj != cur_ring[RIGHT])
	    return TRUE;
    if (obj->o_flags & ISCURSED)
    {
	msg("you can't.  It appears to be cursed");
	return FALSE;
    }
    if (obj == cur_weapon)
	cur_weapon = NULL;
    else if (obj == cur_armor)
    {
	waste_time();
	cur_armor = NULL;
    }
    else
    {
	cur_ring[obj == cur_ring[LEFT] ? LEFT : RIGHT] = NULL;
	switch (obj->o_which)
	{
	    case R_ADDSTR:
		chg_str(-obj->o_arm);
		break;
	    case R_SEEINVIS:
		unsee();
		extinguish(unsee);
		break;
	}
    }
    return TRUE;
}

THING *
new_thing()
{
    THING *cur;
    int r;

    cur = new_item();
    cur->o_hplus = 0;
    cur->o_dplus = 0;
    strncpy(cur->o_damage, "0x0", sizeof(cur->o_damage));
    strncpy(cur->o_hurldmg, "0x0", sizeof(cur->o_hurldmg));
    cur->o_arm = 11;
    cur->o_count = 1;
    cur->o_group = 0;
    cur->o_flags = 0;
    /*
     * Decide what kind of object it will be
     * If we haven't had food for a while, let it be food.
     */
    switch (no_food > 3 ? 2 : pick_one(things, NUMTHINGS))
    {
	case 0:
	    cur->o_type = POTION;
	    cur->o_which = pick_one(pot_info, MAXPOTIONS);
	when 1:
	    cur->o_type = SCROLL;
	    cur->o_which = pick_one(scr_info, MAXSCROLLS);
	when 2:
	    cur->o_type = FOOD;
	    no_food = 0;
	    if (rnd(10) != 0)
		cur->o_which = 0;
	    else
		cur->o_which = 1;
	when 3:
	    init_weapon(cur, pick_one(weap_info, MAXWEAPONS));
	    if ((r = rnd(100)) < 10)
	    {
		cur->o_flags |= ISCURSED;
		cur->o_hplus -= rnd(3) + 1;
	    }
	    else if (r < 15)
		cur->o_hplus += rnd(3) + 1;
	when 4:
	    cur->o_type = ARMOR;
	    cur->o_which = pick_one(arm_info, MAXARMORS);
	    cur->o_arm = a_class[cur->o_which];
	    if ((r = rnd(100)) < 20)
	    {
		cur->o_flags |= ISCURSED;
		cur->o_arm += rnd(3) + 1;
	    }
	    else if (r < 28)
		cur->o_arm -= rnd(3) + 1;
	when 5:
	    cur->o_type = RING;
	    cur->o_which = pick_one(ring_info, MAXRINGS);
	    switch (cur->o_which)
	    {
		case R_ADDSTR:
		case R_PROTECT:
		case R_ADDHIT:
		case R_ADDDAM:
		    if ((cur->o_arm = rnd(3)) == 0)
		    {
			cur->o_arm = -1;
			cur->o_flags |= ISCURSED;
		    }
		when R_AGGR:
		case R_TELEPORT:
		    cur->o_flags |= ISCURSED;
	    }
	when 6:
	    cur->o_type = STICK;
	    cur->o_which = pick_one(ws_info, MAXSTICKS);
	    fix_stick(cur);
    }
    return cur;
}
*/
/// new_thing
pub fn create_object(state: &mut RogueState) -> Object {
	let object_type: ObjectType = if state.no_food > 3 { ObjectType::generate_new_food() } else { ObjectType::generate_new() };
    let mut object = Object::new(object_type);
    // Decide what kind of object it will be
    // If we haven't had food for a while, let it be food.
	match object.object_type {
		ObjectType::Food(_) => {
			state.no_food = 0;
		}
		ObjectType::Weapon(_) => {
	    	//init_weapon(cur, pick_one(weap_info, MAXWEAPONS));
			match rnd(100) {
				0..=9 => {
					object.flags |= ISCURSED;
					object.hplus -= (rnd(3) + 1) as isize;
				}
				10..=14 => {
					object.hplus += (rnd(3) + 1) as isize;
				}
				_ => {}
			}
		}
		ObjectType::Armor(armor_type) => {
			object.arm = armor_type.armor_class() as isize;
			match rnd(100) {
				0..=19 => {
					object.flags |= ISCURSED;
					object.arm += (rnd(3) + 1) as isize;
				}
				20..=27 => {
					object.arm -= (rnd(3) + 1) as isize;
				}
				_ => {}
			}
		}
		ObjectType::Ring(ring_type) => {
			match ring_type {
				RingType::AddStrength |
				RingType::Protection |
				RingType::Dexterity |
				RingType::IncreaseDamage => {
					object.arm = rnd(3) as isize;
					if object.arm == 0 {
						object.arm = -1;
						object.flags |= ISCURSED;
					}
				}
				RingType::AggravateMonster |
				RingType::Teleportation => {
					object.flags |= ISCURSED;
				}
				_ => {}
			}
		}
		ObjectType::Stick(_) => {
		    // fix_stick(object);
		}
		_ => {}
	}
    return object;
}

/* 
// Pick an item out of a list of nitems possible objects
int
pick_one(struct obj_info *info, int nitems)
{
    struct obj_info *end;
    struct obj_info *start;
    int i;

    start = info;
    for (end = &info[nitems], i = rnd(100); info < end; info++)
	if (i < info->oi_prob)
	    break;
    if (info == end)
    {
	info = start;
    }
    return (int)(info - start);
}
*/
fn pick_one() {

}
/*

/*
 * discovered:
 *	list what the player has discovered in this game of a certain type
 */
static int line_cnt = 0;

static bool newpage = FALSE;

static char *lastfmt, *lastarg;


void
discovered()
{
    char ch;
    bool disc_list;

    do {
	disc_list = FALSE;
	addmsg("for what type of object do you want a list");
	msg("? (* for all)");
	ch = readchar();
	switch (ch)
	{
	    case ESCAPE:
		msg("");
		return;
	    case POTION:
	    case SCROLL:
	    case RING:
	    case STICK:
	    case '*':
		disc_list = TRUE;
		break;
	    default:
		    msg("Please type one of %c%c%c%c (ESCAPE to quit)", POTION, SCROLL, RING, STICK);
	}
    } while (!disc_list);
    if (ch == '*')
    {
	print_disc(POTION);
	add_line("", NULL);
	print_disc(SCROLL);
	add_line("", NULL);
	print_disc(RING);
	add_line("", NULL);
	print_disc(STICK);
	end_line();
    }
    else
    {
	print_disc(ch);
	end_line();
    }
}

/*
 * print_disc:
 *	Print what we've discovered of type 'type'
 */

#define MAX4(a,b,c,d)	(a > b ? (a > c ? (a > d ? a : d) : (c > d ? c : d)) : (b > c ? (b > d ? b : d) : (c > d ? c : d)))


void
print_disc(char type)
{
    struct obj_info *info = NULL;
    int i, maxnum = 0, num_found;
    static THING obj;
    static int order[MAX4(MAXSCROLLS, MAXPOTIONS, MAXRINGS, MAXSTICKS)];

    switch (type)
    {
	case SCROLL:
	    maxnum = MAXSCROLLS;
	    info = scr_info;
	    break;
	case POTION:
	    maxnum = MAXPOTIONS;
	    info = pot_info;
	    break;
	case RING:
	    maxnum = MAXRINGS;
	    info = ring_info;
	    break;
	case STICK:
	    maxnum = MAXSTICKS;
	    info = ws_info;
	    break;
    }
    set_order(order, maxnum);
    obj.o_count = 1;
    obj.o_flags = 0;
    num_found = 0;
    for (i = 0; i < maxnum; i++)
	if (info[order[i]].oi_know || info[order[i]].oi_guess)
	{
	    obj.o_type = type;
	    obj.o_which = order[i];
	    add_line("%s", inv_name(&obj, FALSE));
	    num_found++;
	}
    if (num_found == 0)
	add_line(nothing(type), NULL);
}

/*
 *	Set up order for list
 */
void
set_order(int *order, int numthings)
{
    int i, r, t;

    for (i = 0; i< numthings; i++)
	order[i] = i;

    for (i = numthings; i > 0; i--)
    {
	r = rnd(i);
	t = order[i - 1];
	order[i - 1] = order[r];
	order[r] = t;
    }
}

/*
 * add_line:
 *	Add a line to the list of discoveries
 */
/* VARARGS1 */
char
add_line(char *fmt, char *arg)
{
    WINDOW *tw, *sw;
    int x, y;
    char *prompt = "--Press space to continue--";
    static int maxlen = -1;

    if (line_cnt == 0) {
	    wclear(hw);
    }
	if (maxlen < 0)
	    maxlen = (int) strlen(prompt);
	if (line_cnt >= LINES - 1 || fmt == NULL) {
		wmove(hw, LINES - 1, 0);
		waddstr(hw, prompt);
		wrefresh(hw);
		wait_for(' ');
		clearok(curscr, TRUE);
		wclear(hw);
		touchwin(stdscr);
	    newpage = TRUE;
	    line_cnt = 0;
	    maxlen = (int) strlen(prompt);
	}
	if (fmt != NULL && !(line_cnt == 0 && *fmt == '\0'))
	{
	    mvwprintw(hw, line_cnt++, 0, fmt, arg);
	    getyx(hw, y, x);
	    if (maxlen < x)
		maxlen = x;
	    lastfmt = fmt;
	    lastarg = arg;
	}
    return ~ESCAPE;
}

/*
 * end_line:
 *	End the list of lines
 */

void
end_line()
{
	if (line_cnt == 1 && !newpage)
	{
	    mpos = 0;
	    msg(lastfmt, lastarg);
	}
	else
	    add_line((char *) NULL, NULL);
    line_cnt = 0;
    newpage = FALSE;
}

/*
 * nothing:
 *	Set up prbuf so that message for "nothing found" is there
 */
char *
nothing(char type)
{
    char *sp, *tystr = NULL;

	sprintf(prbuf, "Haven't discovered anything");
    if (type != '*')
    {
	sp = &prbuf[strlen(prbuf)];
	switch (type)
	{
	    case POTION: tystr = "potion";
	    when SCROLL: tystr = "scroll";
	    when RING: tystr = "ring";
	    when STICK: tystr = "stick";
	}
	sprintf(sp, " about any %ss", tystr);
    }
    return prbuf;
}

/*
 * nameit:
 *	Give the proper name to a potion, stick, or ring
 */

void
nameit(THING *obj, char *type, char *which, struct obj_info *op,
    char *(*prfunc)(THING *))
{
    char *pb;

    if (op->oi_know || op->oi_guess)
    {
	if (obj->o_count == 1)
	    sprintf(prbuf, "A %s ", type);
	else
	    sprintf(prbuf, "%d %ss ", obj->o_count, type);
	pb = &prbuf[strlen(prbuf)];
	if (op->oi_know)
	    sprintf(pb, "of %s%s(%s)", op->oi_name, (*prfunc)(obj), which);
	else if (op->oi_guess)
	    sprintf(pb, "called %s%s(%s)", op->oi_guess, (*prfunc)(obj), which);
    }
    else if (obj->o_count == 1)
	sprintf(prbuf, "A%s %s %s", vowelstr(which), which, type);
    else
	sprintf(prbuf, "%d %s %ss", obj->o_count, which, type);
}

/*
 * nullstr:
 *	Return a pointer to a null-length string
 */
char *
nullstr(THING *ignored)
{
    return "";
}
*/