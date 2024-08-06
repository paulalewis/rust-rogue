use std::fmt;

use abstract_game_engine::core::simulator::Action;

use super::{direction::Direction, object::Object};

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum RogueAction {
	Drop(Object),
	Eat(Object),
	Identify(Object),
	IdentifyTrap(Direction),
	Move(Direction),
	PickUp,
	QuaffPotion(Object),
	ReadScroll(Object),
	RemoveArmor(Object),
	RemoveRing(Object),
	Rest,
	Search,
	StairsDown,
	StairsUp,
	Throw(Object, Direction),
	WearArmor(Object),
	WearRing(Object),
	WieldWeapon(Object),
	ZapWand(Object, Direction),
}

impl Action for RogueAction {}

impl fmt::Display for RogueAction {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.to_string().fmt(f)
	}
}