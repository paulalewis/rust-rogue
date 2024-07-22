use crate::core::object_type::{ArmorType, PotionType, RingType};

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
