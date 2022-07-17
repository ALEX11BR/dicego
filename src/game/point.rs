use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
    pub fn out_of_bounds(&self, width: i32, height: i32) -> bool {
        self.x < 0 || self.y < 0 || self.x >= width || self.y >= height
    }
    pub fn diagonal_neighbors(&self) -> impl Iterator<Item = Point> + '_ {
        (0..9).map(|i| *self + Point::new((i % 3) - 1, (i / 3) - 1))
    }
}
impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}
