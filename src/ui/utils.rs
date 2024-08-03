use crate::core::object_type::{ArmorType, PotionType, RingType, ScrollType, StickType, WeaponType};

use super::constants::*;

/// For printfs: if string starts with a vowel, return "n" for an "an".
/// vowelstr
pub fn vowelstr(str: &str) -> &str {
	match str.chars().next() {
		Some('a') | Some('A') | Some('e') | Some('E') | Some('i') | Some('I') | Some('o') | Some('O') | Some('u') | Some('U') => "n",
		_ => "",
	}
}

pub fn center_text_index(str: &str) -> usize {
	28 - ((str.len() + 1) / 2)
}
    
pub fn break_string(string: &String, max_length: usize) -> (String, String) {
    let mut line1 = String::with_capacity(max_length);
    let mut line2 = String::with_capacity(max_length);
    let parts = string.split(char::is_whitespace);
    for word in parts {
        if line1.len() + word.len() < max_length {
            if !line1.is_empty() { line1.push(' '); }
            line1.push_str(word);
        } else {
            if !line2.is_empty() { line2.push(' '); }
            line2.push_str(word);
        }
    }
    (line1, line2)
}

pub fn get_armor_type_display_name(armor_type: ArmorType) -> &'static str {
    match armor_type {
        ArmorType::BandedMail => ARMOR_DISPLAY_NAME_BANDED,
        ArmorType::ChainMail => ARMOR_DISPLAY_NAME_CHAIN,
        ArmorType::LeatherArmor => ARMOR_DISPLAY_NAME_LEATHER,
        ArmorType::PlateMail => ARMOR_DISPLAY_NAME_PLATE,
        ArmorType::RingMail => ARMOR_DISPLAY_NAME_RING,
        ArmorType::ScaleMail => ARMOR_DISPLAY_NAME_SCALE,
        ArmorType::SplintMail => ARMOR_DISPLAY_NAME_SPLINT,
        ArmorType::StuddedLeatherArmor => ARMOR_DISPLAY_NAME_STUDDED,
    }
}

pub fn get_ring_type_display_name(ring_type: RingType) -> &'static str {
    match ring_type {
        RingType::Protection => RING_DISPLAY_NAME_PROTECTION,
        RingType::AddStrength => RING_DISPLAY_NAME_ADD_STRENGTH,
        RingType::SustainStrength => RING_DISPLAY_NAME_SUSTAIN_STRENGTH,
        RingType::Searching => RING_DISPLAY_NAME_SEARCHING,
        RingType::SeeInvisible => RING_DISPLAY_NAME_SEE_INVISIBLE,
        RingType::Adornment => RING_DISPLAY_NAME_ADORNMENT,
        RingType::AggravateMonster => RING_DISPLAY_NAME_AGGRAVATE_MONSTER,
        RingType::Dexterity => RING_DISPLAY_NAME_DEXTERITY,
        RingType::IncreaseDamage => RING_DISPLAY_NAME_INCREASE_DAMAGE,
        RingType::Regeneration => RING_DISPLAY_NAME_REGENERATION,
        RingType::SlowDigestion => RING_DISPLAY_NAME_SLOW_DIGESTION,
        RingType::Teleportation => RING_DISPLAY_NAME_TELEPORTATION,
        RingType::Stealth => RING_DISPLAY_NAME_STEALTH,
        RingType::MaintainArmor => RING_DISPLAY_NAME_MAINTAIN_ARMOR,
    }
}

pub fn get_potion_type_display_name(potion_type: PotionType) -> &'static str {
    match potion_type {
        PotionType::Confusion => POTION_DISPLAY_NAME_CONFUSION,
        PotionType::Hallucination => POTION_DISPLAY_NAME_HALLUCINATION,
        PotionType::Poison => POTION_DISPLAY_NAME_POISON,
        PotionType::GainStrength => POTION_DISPLAY_NAME_GAIN_STRENGTH,
        PotionType::SeeInvisible => POTION_DISPLAY_NAME_SEE_INVISIBLE,
        PotionType::Healing => POTION_DISPLAY_NAME_HEALING,
        PotionType::MonsterDetection => POTION_DISPLAY_NAME_MONSTER_DETECTION,
        PotionType::MagicDetection => POTION_DISPLAY_NAME_MAGIC_DETECTION,
        PotionType::RaiseLevel => POTION_DISPLAY_NAME_RAISE_LEVEL,
        PotionType::ExtraHealing => POTION_DISPLAY_NAME_EXTRA_HEALING,
        PotionType::Haste => POTION_DISPLAY_NAME_HASTE,
        PotionType::RestoreStrength => POTION_DISPLAY_NAME_RESTORE_STREGTH,
        PotionType::Blindness => POTION_DISPLAY_NAME_BLINDNESS,
        PotionType::Levitation => POTION_DISPLAY_NAME_LEVITATION,
    }
}

pub fn get_scroll_type_display_name(scroll_type: ScrollType) -> &'static str {
    match scroll_type {
        ScrollType::MagicMapping => SCROLL_DISPLAY_NAME_MAGIC_MAPPING,
        ScrollType::HoldMonster => SCROLL_DISPLAY_NAME_HOLD_MONSTER,
        ScrollType::Sleep => SCROLL_DISPLAY_NAME_SLEEP,
        ScrollType::EnchantArmor => SCROLL_DISPLAY_NAME_ENCHANT_ARMOR,
        ScrollType::Teleportation => SCROLL_DISPLAY_NAME_TELEPORTATION,
        ScrollType::EnchantWeapon => SCROLL_DISPLAY_NAME_ENCHANT_WEAPON,
        ScrollType::CreateMonster => SCROLL_DISPLAY_NAME_CREATE_MONSTER,
        ScrollType::RemoveCurse => SCROLL_DISPLAY_NAME_REMOVE_CURSE,
        ScrollType::AggravateMonsters => SCROLL_DISPLAY_NAME_AGGRAVATE_MONSTERS,
        ScrollType::ProtectArmor => SCROLL_DISPLAY_NAME_PROTECT_ARMOR,
        ScrollType::MonsterConfusion => SCROLL_DISPLAY_NAME_MONSTER_CONFUSION,
        ScrollType::IdentifyPotion => SCROLL_DISPLAY_NAME_IDENTIFY_POTION,
        ScrollType::IdentifyScroll => SCROLL_DISPLAY_NAME_IDENTIFY_SCROLL,
        ScrollType::IdentifyWeapon => SCROLL_DISPLAY_NAME_IDENTIFY_WEAPON,
        ScrollType::IdentifyArmor => SCROLL_DISPLAY_NAME_IDENTIFY_ARMOR,
        ScrollType::IdentifyRingWandOrStaff => SCROLL_DISPLAY_NAME_IDENTIFY_STICK,
        ScrollType::ScareMonster => SCROLL_DISPLAY_NAME_SCARE_MONSTER,
        ScrollType::FoodDetection => SCROLL_DISPLAY_NAME_FOOD_DETECTION,
    }
}

pub fn get_weapon_type_display_name(weapon_type: WeaponType) -> &'static str {
    match weapon_type {
        WeaponType::Mace => WEAPON_DISPLAY_NAME_MACE,
        WeaponType::LongSword => WEAPON_DISPLAY_NAME_LONG_SWORD,
        WeaponType::ShortBow => WEAPON_DISPLAY_NAME_SHORT_BOW,
        WeaponType::Arrow => WEAPON_DISPLAY_NAME_ARROW,
        WeaponType::Dagger => WEAPON_DISPLAY_NAME_DAGGER,
        WeaponType::TwoHandedSword => WEAPON_DISPLAY_NAME_TWO_HANDED_SWORD,
        WeaponType::Dart => WEAPON_DISPLAY_NAME_DART,
        WeaponType::Spear => WEAPON_DISPLAY_NAME_SPEAR,
        WeaponType::Shuriken => WEAPON_DISPLAY_NAME_SHURIKEN,
    }
}

pub fn get_stick_type_display_name(stick_type: StickType) -> &'static str {
    match stick_type {
        StickType::Light => STICK_DISPLAY_NAME_LIGHT,
        StickType::Fire => STICK_DISPLAY_NAME_FIRE,
        StickType::Cold => STICK_DISPLAY_NAME_COLD,
        StickType::MagicMissile => STICK_DISPLAY_NAME_MAGIC_MISSILE,
        StickType::HasteMonster => STICK_DISPLAY_NAME_HASTE_MONSTER,
        StickType::SlowMonster => STICK_DISPLAY_NAME_SLOW_MONSTER,
        StickType::DrainLife => STICK_DISPLAY_NAME_DRAIN_LIFE,
        StickType::Nothing => STICK_DISPLAY_NAME_NOTHING,
        StickType::Invisibility => STICK_DISPLAY_NAME_INVISIBLE,
        StickType::Lightning => STICK_DISPLAY_NAME_LIGHTNING,
        StickType::Polymorph => STICK_DISPLAY_NAME_POLYMORPH,
        StickType::TeleportAway => STICK_DISPLAY_NAME_TELEPORT_AWAY,
        StickType::TeleportTo => STICK_DISPLAY_NAME_TELEPORT_TO,
        StickType::Cancellation => STICK_DISPLAY_NAME_CANCELLATION,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn center_text_index_zero_len() {
        let len = center_text_index("");
		assert_eq!(len, 28);
    }
    
    #[test]
    fn center_text_index_one_len() {
        let len = center_text_index("1");
		assert_eq!(len, 27);
    }
    
    #[test]
    fn center_text_index_21_len() {
        let len = center_text_index("live long and prosper");
		assert_eq!(len, 17);
    }
    
    #[test]
    #[should_panic]
    fn center_text_index_60_len() {
        let len = center_text_index("||||||||| ||||||||| ||||||||| ||||||||| ||||||||| ||||||||| ");
		assert_eq!(len, 0);
    }
}
