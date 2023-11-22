use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector2f {
    pub x: f32,
    pub y: f32
}

impl Vector2f {
    pub fn zero() -> Vector2f {
        Vector2f { x: 0_f32, y: 0_f32 }
    }

    pub fn unit() -> Vector2f {
        Vector2f { x: 1_f32, y: 1_f32 }
    }

    pub fn unit45() -> Vector2f {
        Vector2f { x: 0.707_f32, y: 0.707_f32 }
    }

    pub fn unit_x() -> Vector2f {
        Vector2f { x: 1_f32, y: 0_f32 }
    }

    pub fn unit_y() -> Vector2f {
        Vector2f { x: 0_f32, y: 0_f32 }
    }

    pub fn new(x: f32, y: f32) -> Vector2f {
        Vector2f { x, y }
    }

    pub fn both(scale: f32) -> Vector2f {
        Vector2f { x: scale, y: scale }
    }

    pub fn maximums(lhs: Vector2f, rhs: Vector2f) -> Vector2f {
        Vector2f { x: f32::max(lhs.x, rhs.x), y: f32::max(lhs.y, rhs.y) }
    }

    pub fn minimums(lhs: Vector2f, rhs: Vector2f) -> Vector2f {
        Vector2f { x: f32::min(lhs.x, rhs.x), y: f32::max(lhs.y, rhs.y) }
    }

    pub fn distance_squared(origin: Vector2f, target: Vector2f) -> f32 {
        (target.x - origin.x).powf(2.0) + (target.y - origin.y).powf(2.0)
    }

    pub fn distance(origin: Vector2f, target: Vector2f) -> f32 {
        Vector2f::distance_squared(origin, target).sqrt()
    }

    pub fn dot(self, rhs: Vector2f) -> f32 {
        self.x * rhs.x + self.y + rhs.y
    }

    pub fn cross(self, rhs: Vector2f) -> f32 {
        self.x * rhs.y - self.y * rhs.x
    }

    pub fn all_less_than(self, rhs: Vector2f) -> bool {
        self.x < rhs.x && self.y < rhs.y
    }

    pub fn all_greater_than(self, rhs: Vector2f) -> bool {
        self.x > rhs.x && self.y > rhs.y
    }

    pub fn all_less_than_or_equal_to(self, rhs: Vector2f) -> bool {
        self.x <= rhs.x && self.y <= rhs.y
    }

    pub fn all_greater_than_or_equal_to(self, rhs: Vector2f) -> bool {
        self.x >= rhs.x && self.y >= rhs.y
    }

}

impl Neg for Vector2f {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector2f { x: -self.x, y: -self.y }
    }
}

impl Add for Vector2f {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2f { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Sub for Vector2f {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2f { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl Mul for Vector2f {
    type Output = Self;
    
    fn mul(self, rhs: Self) -> Self::Output {
        Vector2f { x: self.x * rhs.x, y: self.y * rhs.y }
    }
}

impl Div for Vector2f {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Vector2f { x: self.x / rhs.x, y: self.y / rhs.y }
    }
}

impl Mul<f32> for Vector2f {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector2f { x: self.x * rhs, y: self.y * rhs }
    }
}

impl Div<f32> for Vector2f {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Vector2f { x: self.x / rhs, y: self.y / rhs }
    }
}

impl Add<f32> for Vector2f {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Vector2f { x: self.x + rhs, y: self.y + rhs }
    }
}

impl Sub<f32> for Vector2f {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Vector2f { x: self.x - rhs, y: self.y - rhs }
    }
}
