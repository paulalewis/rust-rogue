use crate::core::{constants::MAXARMORS, object_type::ArmorType};

pub const FRUIT: &str = "slime-mold";
pub const PRESS_ANY_KEY_TO_CONTINUE: &str = "--Press any key to continue--";

pub const RIP: &str = "
                       __________
                      /          \\
                     /            \\
                    /  here lies   \\
                   / one whose name \\
                  / was writ in water\\
                  |                  |
                  |                  |
                  |                  |
                  |                  |
                  |                  |
                 *|     *  *  *      | *
         ________)/\\\\_//(\\/(/\\)/\\//\\/|_)_______
";

pub const WINNER: &str = "
    Congratulations, you have made it to the light of day!
    You have joined the elite ranks of those who have escaped the
    Dungeons of Doom alive. You journey home and sell all your loot at
    a great profit and are admitted to the Fighters' Guild.
";

pub const KILLED_BY: &str = "killed by";

//tr_name
pub const TRAP_NAMES: [&str; 8] = [
    "a trapdoor",
    "an arrow trap",
    "a sleeping gas trap",
    "a beartrap",
    "a teleport trap",
    "a poison dart trap",
    "a rust trap",
    "a mysterious trap"
];

pub const ARMOR_DISPLAY_NAME_BANDED: &str = "banded mail";
pub const ARMOR_DISPLAY_NAME_CHAIN: &str = "chain mail";
pub const ARMOR_DISPLAY_NAME_LEATHER: &str = "leather armor";
pub const ARMOR_DISPLAY_NAME_PLATE: &str = "plate mail";
pub const ARMOR_DISPLAY_NAME_RING: &str = "ring mail";
pub const ARMOR_DISPLAY_NAME_SCALE: &str = "scale mail";
pub const ARMOR_DISPLAY_NAME_SPLINT: &str = "splint mail";
pub const ARMOR_DISPLAY_NAME_STUDDED: &str = "studded leather armor";

pub const RING_DISPLAY_NAME_PROTECTION: &str = "protection";
pub const RING_DISPLAY_NAME_ADD_STRENGTH: &str = "add strength";
pub const RING_DISPLAY_NAME_SUSTAIN_STRENGTH: &str = "sustain strength";
pub const RING_DISPLAY_NAME_SEARCHING: &str = "searching";
pub const RING_DISPLAY_NAME_SEE_INVISIBLE: &str = "see invisible";
pub const RING_DISPLAY_NAME_ADORNMENT: &str = "adornment";
pub const RING_DISPLAY_NAME_AGGRAVATE_MONSTER: &str = "aggravate monster";
pub const RING_DISPLAY_NAME_DEXTERITY: &str = "dexterity";
pub const RING_DISPLAY_NAME_INCREASE_DAMAGE: &str = "increase damage";
pub const RING_DISPLAY_NAME_REGENERATION: &str = "regeneration";
pub const RING_DISPLAY_NAME_SLOW_DIGESTION: &str = "slow digestion";
pub const RING_DISPLAY_NAME_TELEPORTATION: &str = "teleportation";
pub const RING_DISPLAY_NAME_STEALTH: &str = "stealth";
pub const RING_DISPLAY_NAME_MAINTAIN_ARMOR: &str = "maintain armor";

pub const POTION_DISPLAY_NAME_CONFUSION: &str = "confusion";
pub const POTION_DISPLAY_NAME_HALLUCINATION: &str = "hallucination";
pub const POTION_DISPLAY_NAME_POISON: &str = "poison";
pub const POTION_DISPLAY_NAME_GAIN_STRENGTH: &str = "gain strength";
pub const POTION_DISPLAY_NAME_SEE_INVISIBLE: &str = "see invisible";
pub const POTION_DISPLAY_NAME_HEALING: &str = "healing";
pub const POTION_DISPLAY_NAME_MONSTER_DETECTION: &str = "monster detection";
pub const POTION_DISPLAY_NAME_MAGIC_DETECTION: &str = "magic detection";
pub const POTION_DISPLAY_NAME_RAISE_LEVEL: &str = "raise level";
pub const POTION_DISPLAY_NAME_EXTRA_HEALING: &str = "extra healing";
pub const POTION_DISPLAY_NAME_HASTE: &str = "haste self";
pub const POTION_DISPLAY_NAME_RESTORE_STREGTH: &str = "restore strength";
pub const POTION_DISPLAY_NAME_BLINDNESS: &str = "blindness";
pub const POTION_DISPLAY_NAME_LEVITATION: &str = "levitation";