use crate::{constants::{MAXSCROLLS, MAXRINGS, MAXPOTIONS, MAXSTICKS}, utils::rnd, rogue::{NSTONES, stones, NCOLORS, rainbow, NMETAL, NWOOD, metal, wood}, core::{object::Object, dungeon::Dungeon, player::Player}};

// This represents the state of the game.
// It can be used to save and restore a game.
#[derive(Debug, Clone, PartialEq)]
pub struct RogueState<'a> {
    // amulet player found the amulet
    pub amulet: bool,
    //no_food number of levels without food 
    pub no_food: usize,
    //ntraps number of traps on this level
    pub n_traps: usize,
    // max_level deepest player has gone 
    pub max_level: usize,
    pub player: Player,
    pub dungeon: Dungeon,
    // p_colors colors of the potions
    pub potion_colors: [&'a str; MAXPOTIONS],
    // quiet number of quiet turns 
    pub quiet: usize,
    // r_stones stone settings of the rings 
    pub ring_stones: [&'a str; MAXRINGS],
    // s_names names of the scrolls
    pub scroll_names: Vec<String>,
    //seed random number seed
    pub seed: u64,
    // ws_made what sticks are made of
    pub stick_material: [&'a str; MAXSTICKS],
    // ws_type is it a wand or a staff
    pub is_wand: [bool; MAXSTICKS],
}

impl <'a> RogueState<'a> {
    pub fn new(seed: u64) -> RogueState<'a> {
        let (stick_material, is_wand) = init_wand_and_staff_materials();
        RogueState {
            amulet: false,
            no_food: 0,
            n_traps: 0,
            max_level: 1,
            player: Player::new(),
            dungeon: Dungeon::new(),
            potion_colors: init_potion_colors(),
            quiet: 0,
            ring_stones: init_ring_stones(),
            scroll_names: init_scroll_names(),
            seed,
            stick_material,
            is_wand,
        }
    }

    pub fn calculate_score(&self) -> usize {
	    let score = self.player.purse - self.player.purse / 10;
        score
    }
}

fn init_scroll_names() -> Vec<String> {
    (0..MAXSCROLLS).map(|_| generate_scroll_name()).collect()
}

const MAX_NAME_LENGTH: usize = 40;
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

fn init_ring_stones<'a>() -> [&'a str; MAXRINGS] {
    let mut used = [false; NSTONES];
    let mut ring_stones = [""; MAXRINGS];
    for i in 0..MAXRINGS {
        let mut j;
        loop {
            j = rnd(NSTONES);
            if !used[j] { break; }
        }
        used[j] = true;
        unsafe {
            // todo - ring_info[i].worth += stones[j].value;
        }
        ring_stones[i] = stones[j].name;
    }
    return ring_stones;
}

fn init_potion_colors<'a>() -> [&'a str; MAXPOTIONS] {
    let mut potion_colors = [""; MAXPOTIONS];
    let mut used = [false; NCOLORS];
    for i in 0..MAXPOTIONS {
        let mut j;
        loop {
            j = rnd(NCOLORS);
            if !used[j] {
                break;
            }
        }
        used[j] = true;
        potion_colors[i] = rainbow[j];
    }
    potion_colors
}

fn init_wand_and_staff_materials<'a>() -> ([&'a str; MAXSTICKS], [bool; MAXSTICKS]) {
    let mut stick_material = [""; MAXSTICKS];
    let mut is_wand = [false; MAXSTICKS];
    let mut wood_used = [false; NWOOD];
    let mut metal_used = [false; NMETAL];
    for i in 0..MAXSTICKS {
        let mut j;
        let material: &'a str;
        loop {
            if rnd(2) == 0 {
                j = rnd(NMETAL);
                if !metal_used[j] {
                    is_wand[i] = true;
                    material = metal[j];
                    metal_used[j] = true;
                    break;
                }
            } else {
                j = rnd(NWOOD);
                if !wood_used[j] {
                    is_wand[i] = false;
                    material = wood[j];
                    wood_used[j] = true;
                    break;
                }
            }
        }
        stick_material[i] = material;
    }
    (stick_material, is_wand)
}

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