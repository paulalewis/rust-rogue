use crate::core::monster::MONSTERS;
use crate::ui::constants::PRESS_ANY_KEY_TO_CONTINUE;
use crate::ui::game_screen::SCREEN_HEIGHT;
use crate::ui::utils::vowelstr;
// use crate::ui::input_handler::wait_for_character;

// Do something really fun when he dies
// death 
pub fn death(purse: usize, monster: char) {
	draw_death_screen(purse - purse / 10, monster);
	// wait_for_character(' ');
}

fn draw_death_screen(purse: usize, monster: char) {
	//screen.clear();
	//screen.move_cursor(8, 0);
	//screen.write(RIP);
	//let gold_string = format!("{} Au", purse);
	//screen.move_cursor(16, center_text_index(&gold_string));
	//screen.write(&gold_string);
	//let (article, killer_name) = killer_name(monster);
	//screen.move_cursor(18, center_text_index(&killer_name));
	//screen.write(killer_name.as_str());
	//let killed_string = format!("{} {}", KILLED_BY, article);
	//screen.move_cursor(17, center_text_index(&killed_string.trim_end()));
	//screen.write(&killed_string.trim_end());
	//screen.move_cursor(SCREEN_HEIGHT - 1, 0);
	//screen.write(PRESS_ANY_KEY_TO_CONTINUE);
	// screen.draw();
}

/*
// Code for a winner
void
total_winner()
{
    int worth = 0;
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
pub fn total_winner() {
	//screen.clear();
	//screen.move_cursor(0, 0);
	//screen.write(WINNER);
	//screen.move_cursor(SCREEN_HEIGHT - 1, 0);
	//screen.write(PRESS_ANY_KEY_TO_CONTINUE);
	// wait_for_character(' ');
	//screen.clear();
}

// Convert a code to a monster name
// killname
fn killer_name(killer_char: char) -> (String, String) {
	let death_causes = [
		('a', "arrow", true),
		('b', "bolt", true),
		('d', "dart", true),
		('h', "hypothermia", false),
		('s', "starvation", false),
	];

	let (name, article) = if killer_char.is_ascii_uppercase() {
		(String::from(MONSTERS[killer_char as usize - 'A' as usize].name), true)
	} else {
		let mut pair: Option<(String, bool)> = None;
		for hp in death_causes.iter() {
			if hp.0 == killer_char {
				pair = Some((String::from(hp.1), hp.2));
				break;
			}
		}
		pair.expect(format!("Unknown killer char: {}", killer_char).as_str())
	};

	(if article { format!("a{}", vowelstr(&name)) } else { "".to_string() }, name)
}

#[cfg(test)]
mod tests {
	use serial_test::serial;

    use super::{draw_death_screen};

	#[test]
	fn killer_name_arrow() {
		let (article, name) = super::killer_name('a');
		assert_eq!(article, "an");
		assert_eq!(name, "arrow");
	}

	#[test]
	fn killer_name_snake() {
		let (article, name) = super::killer_name('S');
		assert_eq!(article, "a");
		assert_eq!(name, "snake");
	}

	#[test]
	fn killer_name_hypothermia() {
		let (article, name) = super::killer_name('h');
		assert_eq!(article, "");
		assert_eq!(name, "hypothermia");
	}

    #[test]
	#[serial]
    fn death_by_arrow() {
		// draw_death_screen(Box::new(MockScreen::new()), 100, 'a');
    }
    
	
	#[test]
	#[serial]
    fn death_by_snake() {
		// draw_death_screen(Box::new(MockScreen::new()), 100, 'S');
    }
	
	#[test]
	#[serial]
    fn death_by_starvation() {
		// draw_death_screen(Box::new(MockScreen::new()), 0, 's');
    }
}
