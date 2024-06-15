use std::fmt;

use abstract_game_engine::core::simulator::Action;

use super::direction::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum RogueAction {
	DropObject,
	EatFood,
	Fight(Direction),
	Identify,
	IdentifyTrap(Direction),
	Move(Direction),
	MoveNoPickup(Direction),
	PickUp,
	QuaffPotion,
	ReadScroll,
	RemoveArmor,
	RemoveRing,
	Rest,
	Run(Direction),
	Search,
	StairsDown,
	StairsUp,
	ShowInventory,
	Throw(Direction),
	WearArmor,
	WearRing,
	WieldWeapon,
	ZapWand(Direction),
}

impl Action for RogueAction {}

impl fmt::Display for RogueAction {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		todo!()
	}
}