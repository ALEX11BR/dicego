use std::ops::{Index, IndexMut};

use rand::{thread_rng, Rng};

use super::{point::Point, tile::Tile};

#[derive(Debug)]
pub struct Array2D<T> {
    array: Vec<T>,
    width: i32,
    height: i32,
    default: T,
}
impl<T: Copy> Array2D<T> {
    pub fn new(width: i32, height: i32, default: T) -> Self {
        Array2D {
            array: vec![default; (width * height) as usize],
            width,
            height,
            default,
        }
    }
}
impl Array2D<Tile> {
    pub fn carve_h_corridor(&mut self, x1: i32, x2: i32, y: i32) {
        for x in x1.min(x2)..=x1.max(x2) {
            self[Point::new(x, y)] = Tile::Floor;
        }
    }
    pub fn carve_v_corridor(&mut self, x: i32, y1: i32, y2: i32) {
        for y in y1.min(y2)..=y1.max(y2) {
            self[Point::new(x, y)] = Tile::Floor;
        }
    }
    pub fn generate_floor_point(&self) -> Point {
        loop {
            let proposed_point = Point::new(
                thread_rng().gen_range(0..self.width),
                thread_rng().gen_range(0..self.height),
            );
            if let Tile::Floor = self[proposed_point] {
                break proposed_point;
            }
        }
    }
}
impl<T> Index<Point> for Array2D<T> {
    type Output = T;
    fn index(&self, index: Point) -> &Self::Output {
        if index.out_of_bounds(self.width, self.height) {
            &self.default
        } else {
            &self
                .array
                .get((index.y * self.height + index.x) as usize)
                .unwrap_or(&self.default)
        }
    }
}
impl<T> IndexMut<Point> for Array2D<T> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.array[(index.y * self.height + index.x) as usize]
    }
}
