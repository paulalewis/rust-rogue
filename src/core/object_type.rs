use super::{object_info::{ARMOR_INFO, POTION_INFO, RING_INFO, SCROLL_INFO, STICK_INFO, WEAPON_INFO}, utils::rnd};


/// struct obj_info things[NUMTHINGS]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ObjectType {
    Amulet,
    Food(FoodType),
    Potion(PotionType),
    Scroll(ScrollType),
    Weapon(WeaponType),
    Armor(ArmorType),
    Ring(RingType),
    Stick(StickType),
}

impl ObjectType {
    /// pub const OBJECT_PROBABILITIES: [usize; NUMTHINGS] = [26, 62, 78, 85, 92, 96, 100];
    pub fn generate_new() -> Self {
        match rnd(100) {
            0..=25 => ObjectType::Potion(PotionType::new()),
            26..=61 => ObjectType::Scroll(ScrollType::new()),
            62..=77 => ObjectType::Food(FoodType::new()),
            78..=84 => ObjectType::Weapon(WeaponType::new()),
            85..=91 => ObjectType::Armor(ArmorType::new()),
            92..=95 => ObjectType::Ring(RingType::new()),
            96..=99 => ObjectType::Stick(StickType::new()),
            _ => panic!("should not happen"),
        }
    }

    pub fn generate_new_food() -> Self {
        ObjectType::Food(FoodType::new())
    }

    pub fn base_value(self) -> usize {
        match self {
            ObjectType::Amulet => 1000,
            ObjectType::Food(_) => 2,
            ObjectType::Potion(potion_type) => POTION_INFO[potion_type as usize].worth,
            ObjectType::Scroll(scroll_type) => SCROLL_INFO[scroll_type as usize].worth,
            ObjectType::Weapon(weapon_type) => WEAPON_INFO[weapon_type as usize].worth,
            ObjectType::Armor(armor_type) => ARMOR_INFO[armor_type as usize].worth,
            ObjectType::Ring(ring_type) => RING_INFO[ring_type as usize].worth,
            ObjectType::Stick(stick_type) => STICK_INFO[stick_type as usize].worth,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum FoodType {
    Ration,
    SlimeMold,
}

impl FoodType {
    pub fn new() -> Self {
        if rnd(10) != 0 { FoodType::Ration } else { FoodType::SlimeMold }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum PotionType {
    Confusion,
    Hallucination,
    Poison,
    GainStrength,
    SeeInvisible,
    Healing,
    MonsterDetection,
    MagicDetection,
    RaiseLevel,
    ExtraHealing,
    Haste,
    RestoreStrength,
    Blindness,
    Levitation,
}

impl PotionType {
    fn new() -> Self {
        match rnd(100) {
            0..=6 => Self::Confusion,
            7..=14 => Self::Hallucination,
            15..=22 => Self::Poison,
            23..=35 => Self::GainStrength,
            36..=38 => Self::SeeInvisible,
            39..=51 => Self::Healing,
            52..=57 => Self::MonsterDetection,
            58..=63 => Self::MagicDetection,
            64..=65 => Self::RaiseLevel,
            66..=70 => Self::ExtraHealing,
            71..=75 => Self::Haste,
            76..=88 => Self::RestoreStrength,
            89..=93 => Self::Blindness,
            94..=99 => Self::Levitation,
            _ => panic!("should not happen"),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ScrollType {
    MonsterConfusion,
    MagicMapping,
    HoldMonster,
    Sleep,
    EnchantArmor,
    IdentifyPotion,
    IdentifyScroll,
    IdentifyWeapon,
    IdentifyArmor,
    IdentifyRingWandOrStaff,
    ScareMonster,
    FoodDetection,
    Teleportation,
    EnchantWeapon,
    CreateMonster,
    RemoveCurse,
    AggravateMonsters,
    ProtectArmor,
}

impl ScrollType {
    pub fn new() -> Self {
        match rnd(100) {
            0..=6 => Self::MonsterConfusion,
            7..=10 => Self::MagicMapping,
            11..=12 => Self::HoldMonster,
            13..=15 => Self::Sleep,
            16..=22 => Self::EnchantArmor,
            23..=32 => Self::IdentifyPotion,
            33..=42 => Self::IdentifyScroll,
            43..=48 => Self::IdentifyWeapon,
            49..=55 => Self::IdentifyArmor,
            56..=65 => Self::IdentifyRingWandOrStaff,
            66..=68 => Self::ScareMonster,
            69..=70 => Self::FoodDetection,
            71..=75 => Self::Teleportation,
            76..=83 => Self::EnchantWeapon,
            84..=87 => Self::CreateMonster,
            88..=94 => Self::RemoveCurse,
            95..=97 => Self::AggravateMonsters,
            98..=99 => Self::ProtectArmor,
            _ => panic!("should not happen"),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum WeaponType {
    Mace,
    LongSword,
    ShortBow,
    Arrow,
    Dagger,
    TwoHandedSword,
    Dart,
    Shuriken,
    Spear,
}

impl WeaponType {
    pub fn new() -> Self {
        match rnd(100) {
            0..=10 => Self::Mace,
            11..=21 => Self::LongSword,
            22..=33 => Self::ShortBow,
            34..=45 => Self::Arrow,
            46..=53 => Self::Dagger,
            54..=63 => Self::TwoHandedSword,
            64..=75 => Self::Dart,
            76..=87 => Self::Shuriken,
            88..=99 => Self::Spear,
            _ => panic!("should not happen"),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ArmorType {
    BandedMail,
    ChainMail,
    LeatherArmor,
    PlateMail,
    RingMail,
    ScaleMail,
    SplintMail,
    StuddedLeatherArmor,
}

impl ArmorType {
    pub fn new() -> Self {
        match rnd(100) {
            0..=19 => Self::LeatherArmor,
            20..=34 => Self::RingMail,
            35..=49 => Self::StuddedLeatherArmor,
            50..=62 => Self::ScaleMail,
            63..=74 => Self::ChainMail,
            75..=84 => Self::SplintMail,
            85..=94 => Self::BandedMail,
            95..=99 => Self::PlateMail,
            _ => panic!("should not happen"),
        }
    }

    //int a_class[MAXARMORS] = {
    //	8,	/* LEATHER */
    //	7,	/* RING_MAIL */
    //	7,	/* STUDDED_LEATHER */
    //	6,	/* SCALE_MAIL */
    //	5,	/* CHAIN_MAIL */
    //	4,	/* SPLINT_MAIL */
    //	4,	/* BANDED_MAIL */
    //	3,	/* PLATE_MAIL */
    //};
    pub fn armor_class(&self) -> usize {
        match self {
            ArmorType::BandedMail => 4,
            ArmorType::ChainMail => 5,
            ArmorType::LeatherArmor => 8,
            ArmorType::PlateMail => 3,
            ArmorType::RingMail => 7,
            ArmorType::ScaleMail => 6,
            ArmorType::SplintMail => 4,
            ArmorType::StuddedLeatherArmor => 7,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum RingType {
    Protection,
    AddStrength,
    SustainStrength,
    Searching,
    SeeInvisible,
    Adornment,
    AggravateMonster,
    Dexterity,
    IncreaseDamage,
    Regeneration,
    SlowDigestion,
    Teleportation,
    Stealth,
    MaintainArmor,
}

impl RingType {
    pub fn new() -> Self {
        match rnd(100) {
            0..=8 => Self::Protection,
            9..=17 => Self::AddStrength,
            18..=22 => Self::SustainStrength,
            23..=32 => Self::Searching,
            33..=42 => Self::SeeInvisible,
            43..=43 => Self::Adornment,
            44..=53 => Self::AggravateMonster,
            54..=61 => Self::Dexterity,
            62..=69 => Self::IncreaseDamage,
            70..=73 => Self::Regeneration,
            74..=82 => Self::SlowDigestion,
            83..=87 => Self::Teleportation,
            88..=94 => Self::Stealth,
            95..=99 => Self::MaintainArmor,           
            _ => panic!("should not happen"),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum StickType {
    Light,
    Invisibility,
    Lightning,
    Fire,
    Cold,
    Polymorph,
    MagicMissile,
    HasteMonster,
    SlowMonster,
    DrainLife,
    Nothing,
    TeleportAway,
    TeleportTo,
    Cancellation,
}

impl StickType {
    pub fn new() -> Self {
        match rnd(100) {
            0..=11 => Self::Light,
            12..=17 => Self::Invisibility,
            18..=20 => Self::Lightning,
            21..=23 => Self::Fire,
            24..=26 => Self::Cold,
            27..=41 => Self::Polymorph,
            42..=51 => Self::MagicMissile,
            52..=61 => Self::HasteMonster,
            62..=72 => Self::SlowMonster,
            73..=81 => Self::DrainLife,
            82..=82 => Self::Nothing,
            83..=88 => Self::TeleportAway,
            89..=94 => Self::TeleportTo,
            95..=99 => Self::Cancellation,
            _ => panic!("should not happen"),
        }
    }
}