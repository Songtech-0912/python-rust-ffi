use std::fmt;
use std::fmt::Display;

// A basic 2D vector type we want to expose to Python

#[derive(Clone, PartialEq, Eq, PartialOrd)]
pub struct Vec2<T: Clone + Display> {
	pub x: T,
	pub y: T
}

impl<T: Clone + Display> Vec2<T> {
	pub fn new(x: T, y: T) -> Vec2<T> {
		Vec2 { x, y }
	}
}

impl<T: Clone + Display> fmt::Debug for Vec2<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result 
	{
		write!(f, "Vec2({}, {})", self.x, self.y)
	}
}
