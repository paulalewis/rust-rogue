// use crate::{constants::PRESS_SPACE_TO_CONTINUE, core::{direction::Direction, rogue_action::RogueAction, rogue_state::RogueState, screen::Screen}, rogue::*, ui::input_handler::{read_character, wait_for_character}};

// Process the user commands
/*void command() {
    register char ch;
    register int ntimes = 1;			/* Number of player moves */
    char *fp;
    THING *mp;
    static char countch, direction, newcount = FALSE;

    if (on(player, ISHASTE))
	ntimes++;
    /*
     * Let the daemons start up
     */
    do_daemons(BEFORE);
    do_fuses(BEFORE);
    while (ntimes--)
    {
	again = FALSE;
	if (has_hit)
	{
	    endmsg();
	    has_hit = FALSE;
	}
	/*
	 * these are illegal things for the player to be, so if any are
	 * set, someone's been poking in memeory
	 */
	if (on(player, ISSLOW|ISGREED|ISINVIS|ISREGEN|ISTARGET))
	    exit(1);

	look(TRUE);
	status();
	lastscore = purse;
	move(hero.y, hero.x);
	refresh();			/* Draw screen */
	take = 0;
	after = TRUE;
	/*
	 * Read command or continue run
	 */
	if (!no_command)
	{
	    if (running || to_death)
		ch = runch;
	    else if (count)
		ch = countch;
	    else
	    {
		ch = readchar();
		move_on = FALSE;
		if (mpos != 0)		/* Erase message if its there */
		    msg("");
	    }
	}
	else
	    ch = '.';
	if (no_command)
	{
	    if (--no_command == 0)
	    {
		player.t_flags |= ISRUN;
		msg("you can move again");
	    }
	}
	else
	{
	    /*
	     * check for prefixes
	     */
	    newcount = FALSE;
	    if (isdigit(ch))
	    {
		count = 0;
		newcount = TRUE;
		while (isdigit(ch))
		{
		    count = count * 10 + (ch - '0');
		    if (count > 255)
			count = 255;
		    ch = readchar();
		}
		countch = ch;
		/*
		 * turn off count for commands which don't make sense
		 * to repeat
		 */
		switch (ch)
		{
		    case '.': case 'a': case 'b': case 'h': case 'j':
		    case 'k': case 'l': case 'm': case 'n': case 'q':
		    case 'r': case 's': case 't': case 'u': case 'y':
		    case 'z': case 'B': case 'C': case 'H': case 'I':
		    case 'J': case 'K': case 'L': case 'N': case 'U':
		    case 'Y':
			break;
		    default:
			count = 0;
		}
	    }
	    /*
	     * execute a command
	     */
	    if (count && !running)
		count--;
	    if (ch != 'a' && ch != ESCAPE && !(running || count || to_death))
	    {
		l_last_comm = last_comm;
		l_last_dir = last_dir;
		l_last_pick = last_pick;
		last_comm = ch;
		last_dir = '\0';
		last_pick = NULL;
	    }
over:
	    switch (ch)
	    {
		case ',': {
		    THING *obj = NULL;
		    int found = 0;
		    for (obj = lvl_obj; obj != NULL; obj = next(obj))
    			{
			    if (obj->o_pos.y == hero.y && obj->o_pos.x == hero.x)
			    {
				found=1;
				break;
			    }
    			}

		    if (found) {
			if (levit_check())
			    ;
			else
			    pick_up((char)obj->o_type);
		    }
		    else {
				addmsg("there is nothing here to pick up");
                endmsg();
		    }
		}
		when 'h': do_move(0, -1);
		when 'j': do_move(1, 0);
		when 'k': do_move(-1, 0);
		when 'l': do_move(0, 1);
		when 'y': do_move(-1, -1);
		when 'u': do_move(-1, 1);
		when 'b': do_move(1, -1);
		when 'n': do_move(1, 1);
		when 'H': do_run('h');
		when 'J': do_run('j');
		when 'K': do_run('k');
		when 'L': do_run('l');
		when 'Y': do_run('y');
		when 'U': do_run('u');
		when 'B': do_run('b');
		when 'N': do_run('n');
		when 'f':
		    if (!get_dir())
		    {
			after = FALSE;
			break;
		    }
		    delta.y += hero.y;
		    delta.x += hero.x;
		    if ( ((mp = moat(delta.y, delta.x)) == NULL)
			|| ((!see_monst(mp)) && !on(player, SEEMONST)))
		    {
			msg("I see no monster there");
			after = FALSE;
		    }
		    else if (diag_ok(&hero, &delta))
		    {
			to_death = TRUE;
			mp->t_flags |= ISTARGET;
			runch = ch = dir_ch;
			goto over;
		    }
		when 't':
		    if (!get_dir())
			after = FALSE;
		    else
			missile(delta.y, delta.x);
		when 'a':
		    if (last_comm == '\0')
		    {
			msg("you haven't typed a command yet");
			after = FALSE;
		    }
		    else
		    {
			ch = last_comm;
			again = TRUE;
			goto over;
		    }
		when 'q': quaff();
		when 'Q':
		    after = FALSE;
		    q_comm = TRUE;
		    quit(0);
		    q_comm = FALSE;
		when 'i': after = FALSE; inventory(pack, 0);
		when 'd': drop();
		when 'r': read_scroll();
		when 'e': eat();
		when 'w': wield();
		when 'W': wear();
		when 'T': take_off();
		when 'P': ring_on();
		when 'R': ring_off();
		when 'c': call(); after = FALSE;
		when '>': after = FALSE; d_level();
		when '<': after = FALSE; u_level();
		when '?': after = FALSE; help();
		when '/': after = FALSE; identify();
		when 's': search();
		when 'z':
		    if (get_dir())
			do_zap();
		    else
			after = FALSE;
		when 'D': after = FALSE; discovered();
		when 'S': 
		    after = FALSE;
		    save_game();
		when '.': ;			/* Rest command */
		when ' ': after = FALSE;	/* "Legal" illegal command */
		when '^':
		    after = FALSE;
		    if (get_dir()) {
			delta.y += hero.y;
			delta.x += hero.x;
			fp = &flat(delta.y, delta.x);
                            addmsg("You have found ");
			if (chat(delta.y, delta.x) != TRAP)
			    msg("no trap there");
			else if (on(player, ISHALU))
			    msg(tr_name[rnd(NTRAPS)]);
			else {
			    msg(tr_name[*fp & F_TMASK]);
			    *fp |= F_SEEN;
			}
		    }
		when ESCAPE:	/* Escape */
		    count = 0;
		    after = FALSE;
		    again = FALSE;
		when 'm':
		    move_on = TRUE;
		    if (!get_dir())
			after = FALSE;
		    else
		    {
			ch = dir_ch;
			countch = dir_ch;
			goto over;
		    }
		when ')': current(cur_weapon, "wielding", NULL);
		when ']': current(cur_armor, "wearing", NULL);
		when '=':
		    current(cur_ring[LEFT], "wearing", "on left hand");
		    current(cur_ring[RIGHT], "wearing", "on right hand");
		otherwise:
		    after = FALSE;
			illegal_command(ch);
	    }
	}
	/*
	 * If he ran into something to take, let him pick it up.
	 */
	if (take != 0)
	    pick_up(take);
	if (!after)
	    ntimes++;
    }
    do_daemons(AFTER);
    do_fuses(AFTER);
    if (ISRING(LEFT, R_SEARCH))
	search();
    else if (ISRING(LEFT, R_TELEPORT) && rnd(50) == 0)
	teleport();
    if (ISRING(RIGHT, R_SEARCH))
	search();
    else if (ISRING(RIGHT, R_TELEPORT) && rnd(50) == 0)
	teleport();
}
*/

/*
// player gropes about him to find hidden things.
void
search()
{
    register int y, x;
    register char *fp;
    register int ey, ex;
    int probinc;
    bool found;

    ey = hero.y + 1;
    ex = hero.x + 1;
    probinc = (on(player, ISHALU) ? 3 : 0);
    probinc += (on(player, ISBLIND) ? 2 : 0);
    found = FALSE;
    for (y = hero.y - 1; y <= ey; y++) 
	for (x = hero.x - 1; x <= ex; x++)
	{
	    if (y == hero.y && x == hero.x)
		continue;
	    fp = &flat(y, x);
	    if (!(*fp & F_REAL))
		switch (chat(y, x))
		{
		    case '|':
		    case '-':
			if (rnd(5 + probinc) != 0)
			    break;
			chat(y, x) = DOOR;
                        msg("a secret door");
foundone:
			found = TRUE;
			*fp |= F_REAL;
			count = FALSE;
			running = FALSE;
			break;
		    case FLOOR:
			if (rnd(2 + probinc) != 0)
			    break;
			chat(y, x) = TRAP;
			    addmsg("you found ");
			if (on(player, ISHALU))
			    msg(tr_name[rnd(NTRAPS)]);
			else {
			    msg(tr_name[*fp & F_TMASK]);
			    *fp |= F_SEEN;
			}
			goto foundone;
			break;
		    case ' ':
			if (rnd(3 + probinc) != 0)
			    break;
			chat(y, x) = PASSAGE;
			goto foundone;
		}
	}
    if (found)
	look(FALSE);
}
*/

// Give single character help, or the whole mess if he wants it
/*void
help()
{
    register struct h_list *strp;
    register int numprint, cnt;
    mpos = 0;
    /*
     * Here we print help for everything.
     * Then wait before we return to command mode
     */
    numprint = 0;
    for (strp = help_items; strp->h_desc != NULL; strp++)
	if (strp->h_print)
	    numprint++;
    if (numprint & 01)		/* round odd numbers up */
	numprint++;
    numprint /= 2;
    if (numprint > LINES - 1)
	numprint = LINES - 1;

    wclear(hw);
    cnt = 0;
    for (strp = help_items; strp->h_desc != NULL; strp++)
	if (strp->h_print)
	{
	    wmove(hw, cnt % numprint, cnt >= numprint ? COLS / 2 : 0);
	    if (strp->h_ch)
		waddstr(hw, unctrl(strp->h_ch));
	    waddstr(hw, strp->h_desc);
	    if (++cnt >= numprint * 2)
		break;
	}
    wmove(hw, LINES - 1, 0);
    waddstr(hw, "--Press space to continue--");
    wrefresh(hw);
    wait_for(' ');
    clearok(stdscr, TRUE);
/*
    refresh();
*/
    msg("");
    touchwin(stdscr);
    wrefresh(stdscr);
}*/

/*
// Tell the player what a certain thing is.
void
identify()
{
    register int ch;
    register struct h_list *hp;
    register char *str;
    static struct h_list ident_list[] = {
	{'|',		"wall of a room",		FALSE},
	{'-',		"wall of a room",		FALSE},
	{GOLD,		"gold",				FALSE},
	{STAIRS,	"a staircase",			FALSE},
	{DOOR,		"door",				FALSE},
	{FLOOR,		"room floor",			FALSE},
	{PLAYER,	"you",				FALSE},
	{PASSAGE,	"passage",			FALSE},
	{TRAP,		"trap",				FALSE},
	{POTION,	"potion",			FALSE},
	{SCROLL,	"scroll",			FALSE},
	{FOOD,		"food",				FALSE},
	{WEAPON,	"weapon",			FALSE},
	{' ',		"solid rock",			FALSE},
	{ARMOR,		"armor",			FALSE},
	{AMULET,	"the Amulet of Yendor",		FALSE},
	{RING,		"ring",				FALSE},
	{STICK,		"wand or staff",		FALSE},
	{'\0'}
    };

    msg("what do you want identified? ");
    ch = readchar();
    mpos = 0;
    if (ch == ESCAPE)
    {
	msg("");
	return;
    }
    if (isupper(ch))
	str = monsters[ch-'A'].m_name;
    else
    {
	str = "unknown character";
	for (hp = ident_list; hp->h_ch != '\0'; hp++)
	    if (hp->h_ch == ch)
	    {
		str = hp->h_desc;
		break;
	    }
    }
    msg("'%s': %s", unctrl(ch), str);
}

/*
 * d_level:
 *	He wants to go down a level
 */
void
d_level()
{
    if (levit_check())
	return;
    if (chat(hero.y, hero.x) != STAIRS)
	msg("I see no way down");
    else
    {
	level++;
	seenstairs = FALSE;
	new_level();
    }
}

/*
 * u_level:
 *	He wants to go up a level
 */
void
u_level()
{
    if (levit_check())
	return;
    if (chat(hero.y, hero.x) == STAIRS)
	if (amulet)
	{
	    level--;
	    if (level == 0)
		total_winner();
	    new_level();
	    msg("you feel a wrenching sensation in your gut");
	}
	else
	    msg("your way is magically blocked");
    else
	msg("I see no way up");
}

/*
 * levit_check:
 *	Check to see if she's levitating, and if she is, print an
 *	appropriate message.
 */
bool
levit_check()
{
    if (!on(player, ISLEVIT))
	return FALSE;
    msg("You can't.  You're floating off the ground!");
    return TRUE;
}

/*
 * call:
 *	Allow a user to call a potion, scroll, or ring something
 */
void
call()
{
    register THING *obj;
    register struct obj_info *op = NULL;
    register char **guess, *elsewise = NULL;
    register bool *know;

    obj = get_item("call", CALLABLE);
    /*
     * Make certain that it is somethings that we want to wear
     */
    if (obj == NULL)
	return;
    switch (obj->o_type)
    {
	case RING:
	    op = &ring_info[obj->o_which];
	    elsewise = r_stones[obj->o_which];
	    goto norm;
	when POTION:
	    op = &pot_info[obj->o_which];
	    elsewise = p_colors[obj->o_which];
	    goto norm;
	when SCROLL:
	    op = &scr_info[obj->o_which];
	    elsewise = s_names[obj->o_which];
	    goto norm;
	when STICK:
	    op = &ws_info[obj->o_which];
	    elsewise = ws_made[obj->o_which];
norm:
	    know = &op->oi_know;
	    guess = &op->oi_guess;
	    if (*guess != NULL)
		elsewise = *guess;
	when FOOD:
	    msg("you can't call that anything");
	    return;
	otherwise:
	    guess = &obj->o_label;
	    know = NULL;
	    elsewise = obj->o_label;
    }
    if (know != NULL && *know)
    {
	msg("that has already been identified");
	return;
    }
    if (elsewise != NULL && elsewise == *guess)
    {
	msg("Was called \"%s\"", elsewise);
    }
	msg("what do you want to call it? ");

    if (elsewise == NULL)
	strcpy(prbuf, "");
    else
	strcpy(prbuf, elsewise);
    if (get_str(prbuf, stdscr) == NORM)
    {
	if (*guess != NULL)
	    free(*guess);
	*guess = malloc((unsigned int) strlen(prbuf) + 1);
	strcpy(*guess, prbuf);
    }
}

// Print the current weapon/armor
void current(THING *cur, char *how, char *where) {
    after = FALSE;
    if (cur != NULL)
    {
	    addmsg("you are %s (", how);
	inv_describe = FALSE;
	addmsg("%c) %s", cur->o_packch, inv_name(cur, TRUE));
	inv_describe = TRUE;
	if (where)
	    addmsg(" %s", where);
	endmsg();
    }
    else
    {
	addmsg("you are %s nothing", how);
	if (where)
	    addmsg(" %s", where);
	endmsg();
    }
}
*/
