use super::coord::Coord;

/*
typedef struct spot {
	int	nexits;
	coord	exits[4];
	int	used;
} SPOT;

#define GOLDGRP 1
*/
/// position matrix for maze positions
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Spot {
	pub nexits: isize,
	pub exits: [Coord; 4],
	pub used: bool,
}