
pub const ADVENTURER_NAME: &str = "Rustacean"; //whoami
pub const FRUIT: &str = "slime-mold";
pub const PRESS_SPACE_TO_CONTINUE: &str = "--Press space to continue--";
pub const RELEASE: &str = "5.4.4";

// maximum length of strings
pub const MAXSTR: usize = 1024;
// maximum number of screen lines used 
pub const MAXLINES: usize = 32;
// maximum number of screen columns used
pub const MAXCOLS: usize = 80;
pub const MAXDAEMONS: usize = 20;
pub const MAXROOMS: usize = 9;
pub const MAXTHINGS: usize = 9;
pub const MAXOBJ: usize = 9;
pub const MAXPACK: usize = 23;
pub const MAXTRAPS: usize = 10;
pub const AMULETLEVEL: usize = 26;
// number of types of things
pub const NUMTHINGS: usize = 7;
// upper limit on number of passages
pub const MAXPASS: usize = 13;
pub const NUMLINES: usize = 24;
pub const NUMCOLS: usize = 80;
pub const STATLINE: usize = NUMLINES - 1;
pub const BORE_LEVEL: usize = 50;

// *** game object characters ***

pub const PASSAGE: char = '#';
pub const DOOR: char = '+';
pub const FLOOR: char = '.';
pub const PLAYER: char = '@';
pub const TRAP: char = '^';
pub const STAIRS: char = '%';
pub const GOLD: char = '*';
pub const POTION: char = '!';
pub const SCROLL: char = '?';
pub const MAGIC: char = '$';
pub const FOOD: char = ':';
pub const WEAPON: char = ')';
pub const ARMOR: char = ']';
pub const AMULET: char = ',';
pub const RING: char = '=';
pub const STICK: char = '/';
pub const CALLABLE: isize = -1;
pub const R_OR_S: isize = -2;

