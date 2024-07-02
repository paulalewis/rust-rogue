use crate::constants::{MAXARMORS, MAXPOTIONS, MAXRINGS, MAXSCROLLS, MAXSTICKS, MAXWEAPONS, NUMTHINGS};

/*struct obj_info {
    char *oi_name;
    int oi_prob;
    int oi_worth;
    char *oi_guess;
    bool oi_know;
};*/
/// Stuff about objects
pub struct ObjInfo {
    pub name: &'static str,
    pub prob: usize,
    pub worth: usize,
    // pub guess: String,
    pub know: bool,
}

//struct obj_info things[NUMTHINGS]
pub static THINGS: [ObjInfo; NUMTHINGS] = [
    ObjInfo { name: "", prob: 26, worth: 0, know: false },/* potion */
    ObjInfo { name: "", prob: 62, worth: 0, know: false },/* scroll */
    ObjInfo { name: "", prob: 78, worth: 0, know: false },/* food */
    ObjInfo { name: "", prob: 85, worth: 0, know: false },/* weapon */
    ObjInfo { name: "", prob: 92, worth: 0, know: false },/* armor */
    ObjInfo { name: "", prob: 96, worth: 0, know: false },/* ring */
    ObjInfo { name: "", prob: 100, worth: 0, know: false },/* stick */
];

//struct obj_info arm_info[MAXARMORS]
pub static ARMOR_INFO: [ObjInfo; MAXARMORS] = [
    ObjInfo { name: "leather armor", prob: 20, worth: 20, know: false },
    ObjInfo { name: "ring mail", prob: 35, worth: 25, know: false },
    ObjInfo { name: "studded leather armor", prob: 50, worth: 20, know: false },
    ObjInfo { name: "scale mail", prob: 63, worth: 30, know: false },
    ObjInfo { name: "chain mail", prob: 75, worth: 75, know: false },
    ObjInfo { name: "splint mail", prob: 85, worth: 80, know: false },
    ObjInfo { name: "banded mail", prob: 95, worth: 90, know: false },
    ObjInfo { name: "plate mail", prob: 100, worth: 150, know: false },
];

//struct obj_info ring_info[MAXRINGS]
pub static RING_INFO: [ObjInfo; MAXRINGS] = [
    ObjInfo { name: "protection", prob: 9, worth: 400, know: false },
    ObjInfo { name: "add strength", prob: 18, worth: 400, know: false },
    ObjInfo { name: "sustain strength", prob: 23, worth: 280, know: false },
    ObjInfo { name: "searching", prob: 33, worth: 420, know: false },
    ObjInfo { name: "see invisible", prob: 43, worth: 310, know: false },
    ObjInfo { name: "adornment", prob: 44, worth: 10, know: false },
    ObjInfo { name: "aggravate monster", prob: 54, worth: 10, know: false },
    ObjInfo { name: "dexterity", prob: 62, worth: 440, know: false },
    ObjInfo { name: "increase damage", prob: 70, worth: 400, know: false },
    ObjInfo { name: "regeneration", prob: 74, worth: 460, know: false },
    ObjInfo { name: "slow digestion", prob: 83, worth: 240, know: false },
    ObjInfo { name: "teleportation", prob: 88, worth: 30, know: false },
    ObjInfo { name: "stealth", prob: 95, worth: 470, know: false },
    ObjInfo { name: "maintain armor", prob: 100, worth: 380, know: false },
];

//struct obj_info pot_info[MAXPOTIONS]
pub static POTION_INFO: [ObjInfo; MAXPOTIONS] = [
    ObjInfo { name: "confusion", prob: 7, worth: 5, know: false },
    ObjInfo { name: "hallucination", prob: 15, worth: 5, know: false },
    ObjInfo { name: "poison", prob: 23, worth: 5, know: false },
    ObjInfo { name: "gain strength", prob: 36, worth: 150, know: false },
    ObjInfo { name: "see invisible", prob: 39, worth: 100, know: false },
    ObjInfo { name: "healing", prob: 52, worth: 130, know: false },
    ObjInfo { name: "monster detection", prob: 58, worth: 130, know: false },
    ObjInfo { name: "magic detection", prob: 64, worth: 105, know: false },
    ObjInfo { name: "raise level", prob: 66, worth: 250, know: false },
    ObjInfo { name: "extra healing", prob: 71, worth: 200, know: false },
    ObjInfo { name: "haste self", prob: 76, worth: 190, know: false },
    ObjInfo { name: "restore strength", prob: 89, worth: 130, know: false },
    ObjInfo { name: "blindness", prob: 94, worth: 5, know: false },
    ObjInfo { name: "levitation", prob: 100, worth: 75, know: false },
];

//struct obj_info scr_info[MAXSCROLLS]
pub static SCROLL_INFO: [ObjInfo; MAXSCROLLS] = [
    ObjInfo { name: "monster confusion", prob: 7, worth: 140, know: false },
    ObjInfo { name: "magic mapping", prob: 11, worth: 150, know: false },
    ObjInfo { name: "hold monster", prob: 13, worth: 180, know: false },
    ObjInfo { name: "sleep", prob: 16, worth: 5, know: false },
    ObjInfo { name: "enchant armor", prob: 23, worth: 160, know: false },
    ObjInfo { name: "identify potion", prob: 33, worth: 80, know: false },
    ObjInfo { name: "identify scroll", prob: 43, worth: 80, know: false },
    ObjInfo { name: "identify weapon", prob: 49, worth: 80, know: false },
    ObjInfo { name: "identify armor", prob: 56, worth: 100, know: false },
    ObjInfo { name: "identify ring, wand or staff", prob: 66, worth: 115, know: false },
    ObjInfo { name: "scare monster", prob: 69, worth: 200, know: false },
    ObjInfo { name: "food detection", prob: 71, worth: 60, know: false },
    ObjInfo { name: "teleportation", prob: 76, worth: 165, know: false },
    ObjInfo { name: "enchant weapon", prob: 84, worth: 150, know: false },
    ObjInfo { name: "create monster", prob: 88, worth: 75, know: false },
    ObjInfo { name: "remove curse", prob: 95, worth: 105, know: false },
    ObjInfo { name: "aggravate monsters", prob: 98, worth: 20, know: false },
    ObjInfo { name: "protect armor", prob: 100, worth: 250, know: false },
];

//struct obj_info weap_info[MAXWEAPONS + 1]
pub static WEAPON_INFO: [ObjInfo; MAXWEAPONS] = [
    ObjInfo { name: "mace", prob: 11, worth: 8, know: false },
    ObjInfo { name: "long sword", prob: 22, worth: 15, know: false },
    ObjInfo { name: "short bow", prob: 34, worth: 15, know: false },
    ObjInfo { name: "arrow", prob: 46, worth: 1, know: false },
    ObjInfo { name: "dagger", prob: 54, worth: 3, know: false },
    ObjInfo { name: "two handed sword", prob: 64, worth: 75, know: false },
    ObjInfo { name: "dart", prob: 76, worth: 2, know: false },
    ObjInfo { name: "shuriken", prob: 88, worth: 5, know: false },
    ObjInfo { name: "spear", prob: 100, worth: 5, know: false },
    //{ NULL, 0 }, /* DO NOT REMOVE: fake entry for dragon's breath
    //ObjInfo { name: String::new(), prob: 0, worth: 0, guess: String::new(), know: false },
];


//struct obj_info ws_info[MAXSTICKS]
pub static STICK_INFO: [ObjInfo; MAXSTICKS] = [
    ObjInfo { name: "light", prob: 12, worth: 250, know: false },
    ObjInfo { name: "invisibility", prob: 18, worth: 5, know: false },
    ObjInfo { name: "lightning", prob: 21, worth: 330, know: false },
    ObjInfo { name: "fire", prob: 24, worth: 330, know: false },
    ObjInfo { name: "cold", prob: 27, worth: 330, know: false },
    ObjInfo { name: "polymorph", prob: 42, worth: 310, know: false },
    ObjInfo { name: "magic missile", prob: 52, worth: 170, know: false },
    ObjInfo { name: "haste monster", prob: 62, worth: 5, know: false },
    ObjInfo { name: "slow monster", prob: 73, worth: 350, know: false },
    ObjInfo { name: "drain life", prob: 82, worth: 300, know: false },
    ObjInfo { name: "nothing", prob: 83, worth: 5, know: false },
    ObjInfo { name: "teleport away", prob: 89, worth: 340, know: false },
    ObjInfo { name: "teleport to", prob: 95, worth: 50, know: false },
    ObjInfo { name: "cancellation", prob: 100, worth: 280, know: false },
];