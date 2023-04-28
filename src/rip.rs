use crate::{rogue::*, player::Player, utils::vowelstr, screen::{Screen, SCREEN_HEIGHT}, io::wait_for, constants::PRESS_SPACE_TO_CONTINUE};

const RIP: &str = "
                       __________
                      /          \\
                     /            \\
                    /  here lies   \\
                   / one whose name \\
                  / was writ in water\\
                  |                  |
                  |                  |
                  |   killed by a    |
                  |                  |
                  |                  |
                 *|     *  *  *      | *
         ________)/\\\\_//(\\/(/\\)/\\//\\/|_)_______
";

/*
// Figure score and post it.
void
score(int amount, int flags, char monst, bool noscore)
{
    SCORE *scp;
    int i;
    SCORE *sc2;
    SCORE *top_ten, *endp;
    void (*fp)(int);
    unsigned int uid;
    static char *reason[] = {
	"killed",
	"quit",
	"A total winner",
	"killed with Amulet"
    };

    start_score();

 if (flags >= 0)
    {
	mvaddstr(LINES - 1, 0 , "[Press return to continue]");
        refresh();
        wgetnstr(stdscr,prbuf,80);
 	endwin();
        printf("\n");
	/*
	 * free up space to "guarantee" there is space for the top_ten
	 */
	delwin(stdscr);
	delwin(curscr);
	if (hw != NULL)
	    delwin(hw);
    }

    top_ten = (SCORE *) malloc(numscores * sizeof (SCORE));
    endp = &top_ten[numscores];
    for (scp = top_ten; scp < endp; scp++)
    {
	scp->sc_score = 0;
	for (i = 0; i < MAXSTR; i++)
	    scp->sc_name[i] = (unsigned char) rnd(255);
	scp->sc_flags = RN;
	scp->sc_level = RN;
	scp->sc_monster = (unsigned short) RN;
	scp->sc_uid = RN;
    }

    signal(SIGINT, SIG_DFL);

    rd_score(top_ten);
    /*
     * Insert her in list if need be
     */
    sc2 = NULL;
    if (!noscore)
    {
	uid = md_getuid();
	for (scp = top_ten; scp < endp; scp++)
	    if (amount > scp->sc_score)
		break;
	    else if (!allscore &&	/* only one score per nowin uid */
		flags != 2 && scp->sc_uid == uid && scp->sc_flags != 2)
		    scp = endp;
	if (scp < endp)
	{
	    if (flags != 2 && !allscore)
	    {
		for (sc2 = scp; sc2 < endp; sc2++)
		{
		    if (sc2->sc_uid == uid && sc2->sc_flags != 2)
			break;
		}
		if (sc2 >= endp)
		    sc2 = endp - 1;
	    }
	    else
		sc2 = endp - 1;
	    while (sc2 > scp)
	    {
		*sc2 = sc2[-1];
		sc2--;
	    }
	    scp->sc_score = amount;
	    strncpy(scp->sc_name, ADVENTURER_NAME, MAXSTR);
	    scp->sc_flags = flags;
	    if (flags == 2)
		scp->sc_level = max_level;
	    else
		scp->sc_level = level;
	    scp->sc_monster = monst;
	    scp->sc_uid = uid;
	    sc2 = scp;
	}
    }
    /*
     * Print the list
     */
    if (flags != -1)
	putchar('\n');
    printf("Top %s %s:\n", Numname, allscore ? "Scores" : "Rogueists");
    printf("   Score Name\n");
    for (scp = top_ten; scp < endp; scp++)
    {
	if (scp->sc_score) {
	    if (sc2 == scp)
            md_raw_standout();
	    printf("%2d %5d %s: %s on level %d", (int) (scp - top_ten + 1),
		scp->sc_score, scp->sc_name, reason[scp->sc_flags],
		scp->sc_level);
	    if (scp->sc_flags == 0 || scp->sc_flags == 3)
		printf(" by %s", killname((char) scp->sc_monster, TRUE));
                printf(".");
	    if (sc2 == scp)
		    md_raw_standend();
            putchar('\n');
	}
	else
	    break;
    }
    /*
     * Update the list file
     */
    if (sc2 != NULL)
    {
	if (lock_sc())
	{
	    fp = signal(SIGINT, SIG_IGN);
	    wr_score(top_ten);
	    unlock_sc();
	    signal(SIGINT, fp);
	}
    }
}
*/

// Do something really fun when he dies
// death 
pub fn death(mut screen: Box<dyn Screen>, player: &Player, monster: char) {
	let purse = player.purse - player.purse / 10;
	// todo - score(purse, 0, monster, false);
	draw_death_screen(screen, purse, monster);
	wait_for(' ');
}

fn draw_death_screen(mut screen: Box<dyn Screen>, purse: usize, monster: char) {
	screen.clear();
	screen.move_cursor(8, 0);
	screen.write(RIP);
	let gold_string = format!("{} Au", purse);
	screen.move_cursor(16, center(&gold_string));
	screen.write(&gold_string);
	let killer_name = killer_name(monster, false);
	screen.move_cursor(18, center(&killer_name));
	screen.write(killer_name.as_str());
	if monster == 's' || monster == 'h' {
		screen.move_cursor(17, 32);
		screen.writ_char(' ');
	} else {
		screen.move_cursor(17, 33);
		screen.write(vowelstr(&killer_name));
	}
	screen.move_cursor(SCREEN_HEIGHT - 1, 0);
	screen.write(PRESS_SPACE_TO_CONTINUE);
	screen.draw();
}


// Return the index to center the given string
fn center(str: &str) -> usize {
	28 - ((str.len() + 1) / 2)
}

/*
// Code for a winner
void
total_winner()
{
    THING *obj;
    struct obj_info *op;
    int worth = 0;
    int oldpurse;

    clear();
    standout();
    addstr("     Congratulations, you have made it to the light of day!    \n");
    standend();
    addstr("\nYou have joined the elite ranks of those who have escaped the\n");
    addstr("Dungeons of Doom alive.  You journey home and sell all your loot at\n");
    addstr("a great profit and are admitted to the Fighters' Guild.\n");
    mvaddstr(LINES - 1, 0, "--Press space to continue--");
    refresh();
    wait_for(' ');
    clear();
    mvaddstr(0, 0, "   Worth  Item\n");
    oldpurse = purse;
    for (obj = pack; obj != NULL; obj = next(obj))
    {
	switch (obj->o_type)
	{
	    case FOOD:
		worth = 2 * obj->o_count;
	    when WEAPON:
		worth = weap_info[obj->o_which].oi_worth;
		worth *= 3 * (obj->o_hplus + obj->o_dplus) + obj->o_count;
		obj->o_flags |= ISKNOW;
	    when ARMOR:
		worth = arm_info[obj->o_which].oi_worth;
		worth += (9 - obj->o_arm) * 100;
		worth += (10 * (a_class[obj->o_which] - obj->o_arm));
		obj->o_flags |= ISKNOW;
	    when SCROLL:
		worth = scr_info[obj->o_which].oi_worth;
		worth *= obj->o_count;
		op = &scr_info[obj->o_which];
		if (!op->oi_know)
		    worth /= 2;
		op->oi_know = TRUE;
	    when POTION:
		worth = pot_info[obj->o_which].oi_worth;
		worth *= obj->o_count;
		op = &pot_info[obj->o_which];
		if (!op->oi_know)
		    worth /= 2;
		op->oi_know = TRUE;
	    when RING:
		op = &ring_info[obj->o_which];
		worth = op->oi_worth;
		if (obj->o_which == R_ADDSTR || obj->o_which == R_ADDDAM ||
		    obj->o_which == R_PROTECT || obj->o_which == R_ADDHIT)
		{
			if (obj->o_arm > 0)
			    worth += obj->o_arm * 100;
			else
			    worth = 10;
		}
		if (!(obj->o_flags & ISKNOW))
		    worth /= 2;
		obj->o_flags |= ISKNOW;
		op->oi_know = TRUE;
	    when STICK:
		op = &ws_info[obj->o_which];
		worth = op->oi_worth;
		worth += 20 * obj->o_charges;
		if (!(obj->o_flags & ISKNOW))
		    worth /= 2;
		obj->o_flags |= ISKNOW;
		op->oi_know = TRUE;
	    when AMULET:
		worth = 1000;
	}
	if (worth < 0)
	    worth = 0;
	printw("%c) %5d  %s\n", obj->o_packch, worth, inv_name(obj, FALSE));
	purse += worth;
    }
    printw("   %5d  Gold Pieces          ", oldpurse);
    refresh();
    score(purse, 2, ' ');
}
*/

const UNKOWN_KILLER_NAME: &str = "Wally the Wonder Badger";
// Convert a code to a monster name
// killname
fn killer_name(monster: char, doart: bool) -> String {
	let death_causes = [
		('a', "arrow", true),
		('b', "bolt", true),
		('d', "dart", true),
		('h', "hypothermia", false),
		('s', "starvation", false),
	];
	
	let (killer_name, article) = if monster.is_ascii_uppercase() {
		(String::from(monsters[monster as usize - 'A' as usize].name.as_str()), true)
	} else {
		let mut sp = String::from(UNKOWN_KILLER_NAME);
		let mut article = false;
		for hp in death_causes.iter() {
			if hp.0 == monster {
				sp = String::from(hp.1);
				article = hp.2;
				break;
			}
		}
		(sp, article)
	};

	if doart && article {
		format!("a{} {}", vowelstr(&killer_name), killer_name)
	} else {
		killer_name
	}
}

#[cfg(test)]
mod tests {
	use serial_test::serial;
	use crate::test_helpers::tests::MockScreen;

    use super::{draw_death_screen};

    #[test]
	#[serial]
    fn death_by_arrow() {
		draw_death_screen(Box::new(MockScreen::new()), 100, 'a');
    }
    
	
	#[test]
	#[serial]
    fn death_by_starvation() {
		draw_death_screen(Box::new(MockScreen::new()), 0, 's');
    }
}