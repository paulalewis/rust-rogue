use super::direction::Direction;

pub enum RogueAction {
	DropObject,
	EatFood,
	FightToDeath(Direction),
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