//#include <stdlib.h>
//#include <curses.h>
//#include <ctype.h>
//#include <string.h>
use crate::extern_c::*;
use crate::rogue::*;
use crate::utils::*;
use std::cmp;

/*void
init_player()
{
    register THING *obj;

    pstats = max_stats;
    food_left = HUNGERTIME;
    /*
     * Give him some food
     */
    obj = new_item();
    obj->o_type = FOOD;
    obj->o_count = 1;
    add_pack(obj, TRUE);
    /*
     * And his suit of armor
     */
    obj = new_item();
    obj->o_type = ARMOR;
    obj->o_which = RING_MAIL;
    obj->o_arm = a_class[RING_MAIL] - 1;
    obj->o_flags |= ISKNOW;
    obj->o_count = 1;
    cur_armor = obj;
    add_pack(obj, TRUE);
    /*
     * Give him his weaponry.  First a mace.
     */
    obj = new_item();
    init_weapon(obj, MACE);
    obj->o_hplus = 1;
    obj->o_dplus = 1;
    obj->o_flags |= ISKNOW;
    add_pack(obj, TRUE);
    cur_weapon = obj;
    /*
     * Now a +1 bow
     */
    obj = new_item();
    init_weapon(obj, BOW);
    obj->o_hplus = 1;
    obj->o_flags |= ISKNOW;
    add_pack(obj, TRUE);
    /*
     * Now some arrows
     */
    obj = new_item();
    init_weapon(obj, ARROW);
    obj->o_count = rnd(15) + 25;
    obj->o_flags |= ISKNOW;
    add_pack(obj, TRUE);
}*/

/*
 * Contains defintions and functions for dealing with things like
 * potions and scrolls
 */
const NCOLORS: usize = 27;
/*char *rainbow[] = {};*/
pub static rainbow: [&str; NCOLORS] = [
    "amber",
    "aquamarine",
    "black",
    "blue",
    "brown",
    "clear",
    "crimson",
    "cyan",
    "ecru",
    "gold",
    "green",
    "grey",
    "magenta",
    "orange",
    "pink",
    "plaid",
    "purple",
    "red",
    "silver",
    "tan",
    "tangerine",
    "topaz",
    "turquoise",
    "vermilion",
    "violet",
    "white",
    "yellow",
];

const NUMBER_OF_SYLLABLES: usize = 147;
//static char *sylls[] = {
static sylls: [&str; NUMBER_OF_SYLLABLES] = [
    "a", "ab", "ag", "aks", "ala", "an", "app", "arg", "arze", "ash",
    "bek", "bie", "bit", "bjor", "blu", "bot", "bu", "byt", "comp",
    "con", "cos", "cre", "dalf", "dan", "den", "do", "e", "eep", "el",
    "eng", "er", "ere", "erk", "esh", "evs", "fa", "fid", "fri", "fu",
    "gan", "gar", "glen", "gop", "gre", "ha", "hyd", "i", "ing", "ip",
    "ish", "it", "ite", "iv", "jo", "kho", "kli", "klis", "la", "lech",
    "mar", "me", "mi", "mic", "mik", "mon", "mung", "mur", "nej",
    "nelg", "nep", "ner", "nes", "nes", "nih", "nin", "o", "od", "ood",
    "org", "orn", "ox", "oxy", "pay", "ple", "plu", "po", "pot",
    "prok", "re", "rea", "rhov", "ri", "ro", "rog", "rok", "rol", "sa",
    "san", "sat", "sef", "seh", "shu", "ski", "sna", "sne", "snik",
    "sno", "so", "sol", "sri", "sta", "sun", "ta", "tab", "tem",
    "ther", "ti", "tox", "trol", "tue", "turs", "u", "ulk", "um", "un",
    "uni", "ur", "val", "viv", "vly", "vom", "wah", "wed", "werg",
    "wex", "whon", "wun", "xo", "y", "yot", "yu", "zant", "zeb", "zim",
    "zok", "zon", "zum",
];

const NSTONES: usize = 26;
//STONE stones[] = {
pub static stones: [Stone; NSTONES] = [
    Stone { name: "agate", value: 25 },
    Stone { name: "alexandrite", value: 40 },
    Stone { name: "amethyst", value: 50 },
    Stone { name: "carnelian", value: 40 },
    Stone { name: "diamond", value: 300 },
    Stone { name: "emerald", value: 300 },
    Stone { name: "germanium", value: 225 },
    Stone { name: "granite", value: 5 },
    Stone { name: "garnet", value: 50 },
    Stone { name: "jade", value: 150 },
    Stone { name: "kryptonite", value: 300 },
    Stone { name: "lapis lazuli", value: 50 },
    Stone { name: "moonstone", value: 50 },
    Stone { name: "obsidian", value: 15 },
    Stone { name: "onyx", value: 60 },
    Stone { name: "opal", value: 200 },
    Stone { name: "pearl", value: 220 },
    Stone { name: "peridot", value: 63 },
    Stone { name: "ruby", value: 350 },
    Stone { name: "sapphire", value: 285 },
    Stone { name: "stibotantalite", value: 200 },
    Stone { name: "tiger eye", value: 50 },
    Stone { name: "topaz", value: 60 },
    Stone { name: "turquoise", value: 70 },
    Stone { name: "taaffeite", value: 300 },
    Stone { name: "zircon", value: 80 },
];

const NWOOD: usize = 33;
//char *wood[] = {
pub static wood: [&str; NWOOD] = [
    "avocado wood",
    "balsa",
    "bamboo",
    "banyan",
    "birch",
    "cedar",
    "cherry",
    "cinnibar",
    "cypress",
    "dogwood",
    "driftwood",
    "ebony",
    "elm",
    "eucalyptus",
    "fall",
    "hemlock",
    "holly",
    "ironwood",
    "kukui wood",
    "mahogany",
    "manzanita",
    "maple",
    "oaken",
    "persimmon wood",
    "pecan",
    "pine",
    "poplar",
    "redwood",
    "rosewood",
    "spruce",
    "teak",
    "walnut",
    "zebrawood",
];

const NMETAL: usize = 22;
//char *metal[] = {
pub static metal: [&str; NMETAL] = [
    "aluminum",
    "beryllium",
    "bone",
    "brass",
    "bronze",
    "copper",
    "electrum",
    "gold",
    "iron",
    "lead",
    "magnesium",
    "mercury",
    "nickel",
    "pewter",
    "platinum",
    "steel",
    "silver",
    "silicon",
    "tin",
    "titanium",
    "tungsten",
    "zinc",
];

// Initialize the potion color scheme
// void init_colors()
pub fn init_colors() {
    let mut used = [false; NCOLORS];
    for i in 0..NCOLORS {
        let mut j;
        loop {
            j = rnd(NCOLORS);
            if !used[j] {
                break;
            }
        }
        used[j] = true;
        unsafe {
            p_colors[i] = rainbow[i];
        }
    }
}

const MAX_NAME_LENGTH: usize = 40;
// Generate the names of the various scrolls
// void init_names()
pub fn init_names() -> Vec<String> {
    let mut scroll_names = Vec::<String>::with_capacity(MAXSCROLLS);
    for _ in 0..MAXSCROLLS {
        scroll_names.push(generate_scroll_name());
    }
    scroll_names
}

fn generate_scroll_name() -> String {
    let mut scroll_name = String::new();
    let nwords = rnd(3) + 2;
    'words: for _ in 0..nwords {
        let nsyl = rnd(3) + 1;
        for _ in 0..nsyl {
            let sp = sylls[rnd(NUMBER_OF_SYLLABLES)];
            if scroll_name.len() + sp.len() > MAX_NAME_LENGTH {
                break 'words;
            }
            scroll_name.push_str(sp);
            scroll_name.push_str(" ");
        }
    }
    scroll_name
}

// Initialize the ring stone setting scheme for this time
/*void init_stones()*/
pub fn init_stones() -> Vec<String> {
    let mut used = [false; NSTONES];
    let mut ring_stones: Vec<String> = Vec::new();
    for i in 0..MAXRINGS {
        let mut j;
        loop {
            j = rnd(NSTONES);
            if !used[j] { break; }
        }
        used[j] = true;
        unsafe {
            // ring_info[i].worth += stones[j].value;
        }
        ring_stones.push(String::from(stones[j].name));
    }
    return ring_stones;
}

/*/
// Initialize the construction materials for wands and staffs
void
init_materials()
{
    register int i, j;
    register char *str;
    static bool metused[NMETAL];

    for (i = 0; i < NWOOD; i++)
	used[i] = FALSE;
    for (i = 0; i < NMETAL; i++)
	metused[i] = FALSE;
    for (i = 0; i < MAXSTICKS; i++)
    {
	for (;;)
	    if (rnd(2) == 0)
	    {
		j = rnd(NMETAL);
		if (!metused[j])
		{
		    ws_type[i] = "wand";
		    str = metal[j];
		    metused[j] = TRUE;
		    break;
		}
	    }
	    else
	    {
		j = rnd(NWOOD);
		if (!used[j])
		{
		    ws_type[i] = "staff";
		    str = wood[j];
		    used[j] = TRUE;
		    break;
		}
	    }
	ws_made[i] = str;
    }
}*/

/*
/*
 *	If he is halucinating, pick a random color name and return it,
 *	otherwise return the given color.
 */
char *
pick_color(char *col)
{
    return (on(player, ISHALU) ? rainbow[rnd(NCOLORS)] : col);
}
*/