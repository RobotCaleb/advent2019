use std::fmt;

#[derive(PartialEq, Copy, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2 { x: x, y: y }
    }

    pub fn zero() -> Vector2 {
        Vector2::new(0.0, 0.0)
    }

    /// returns manhattan distance
    pub fn dist_man(&self, other: &Vector2) -> f32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn dist_sqr(&self, other: &Vector2) -> f32 {
        (self.x - other.x) * (self.x - other.x) + (self.y - other.y) * (self.y - other.y)
    }

    pub fn dist(&self, other: &Vector2) -> f32 {
        self.dist_sqr(other).sqrt()
    }

    pub fn mag_sqr(&self) -> f32 {
        self.dist_sqr(&Vector2::zero())
    }

    pub fn mag(&self) -> f32 {
        self.mag_sqr().sqrt()
    }
}

impl fmt::Display for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vector2 {{ x: {}, y: {} }}", self.x, self.y)
    }
}

impl fmt::Debug for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vector2 {{ x: {}, y: {} }}", self.x, self.y)
    }
}

impl std::ops::Add for Vector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Vector2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new() {
        assert_eq!(Vector2 { x: 1.0, y: 2.0 }, Vector2::new(1.0, 2.0));
    }

    #[test]
    fn add() {
        let a = Vector2::new(1.0, 2.0);
        let b = Vector2::new(1.0, 2.0);
        assert_eq!(a + b, Vector2::new(2.0, 4.0));
    }

    #[test]
    fn sub() {
        let a = Vector2::new(1.0, 2.0);
        let b = Vector2::new(1.0, 2.0);
        assert_eq!(a - b, Vector2::zero());
    }

    #[test]
    fn dist_man() {
        let a = Vector2::new(1.0, 2.0);
        let b = Vector2::new(1.0, 2.0);
        let c = Vector2::zero();
        let d = Vector2::new(1.0, 2.0);
        assert_eq!(a.dist_man(&b), 0.0);
        assert_eq!(c.dist_man(&d), 3.0);
        assert_eq!(d.dist_man(&c), 3.0);
    }

    #[test]
    fn dist() {
        assert_eq!(Vector2::zero().dist(&Vector2::new(0.0, 1.0)), 1.0);
    }

    #[test]
    fn mag() {
        assert_eq!(Vector2::new(0.0, 5.0).mag(), 5.0);
    }
}
