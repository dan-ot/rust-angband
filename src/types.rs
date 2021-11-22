use crate::random::Random;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, PartialEq)]
pub struct Loc {
	pub x: i32,
	pub y: i32
}

impl Loc {
	pub fn zero() -> Loc {
		Loc {x: 0, y: 0}
	}

	pub fn rand(rng: &mut Random, center: Loc, x_spread: i32, y_spread: i32) -> Loc {
		Loc {
			x: rng.rand_spread(center.x, x_spread),
			y: rng.rand_spread(center.y, y_spread)
		}
	}

	pub fn is_origin(self) -> bool {
		self.x == 0 && self.y == 0
	}

	pub fn offset(self, dx: i32, dy: i32) -> Loc {
		Loc {
			x: self.x + dx,
			y: self.y + dy
		}
	}
}

impl Add for Loc {
	type Output = Loc;

	fn add(self, rhs: Loc) -> Loc { 
		Loc {
			x: self.x + rhs.x,
			y: self.y + rhs.y
		}
	}
}

impl Sub for Loc {
	type Output = Loc;

	fn sub(self, rhs: Loc) -> Loc {
		Loc {
			x: self.x - rhs.x,
			y: self.y - rhs.y
		}
	}
}

