use rand::{thread_rng, Rng};

use super::point::Point;

#[derive(Debug, Clone, Copy)]
pub struct Room {
    pub top_left: Point,
    pub width: i32,
    pub height: i32,
}
impl Room {
    pub fn new(width: i32, height: i32, top_left: Point) -> Self {
        Room {
            top_left,
            width,
            height,
        }
    }
    pub fn generate(level_width: i32, level_height: i32) -> Self {
        let width = thread_rng().gen_range(4..=10);
        let height = thread_rng().gen_range(4..=10);
        let top_left_x = thread_rng().gen_range(1..(level_width - width - 1));
        let top_left_y = thread_rng().gen_range(1..(level_height - height - 1));

        Room::new(width, height, Point::new(top_left_x, top_left_y))
    }
    pub fn generate_point(&self) -> Point {
        Point::new(
            thread_rng().gen_range(self.top_left.x..(self.top_left.x + self.width)),
            thread_rng().gen_range(self.top_left.y..(self.top_left.y + self.height)),
        )
    }
    pub fn all_points(&self) -> impl Iterator<Item = Point> + '_ {
        (0..(self.width * self.height))
            .map(|i| self.top_left + Point::new(i % self.width, i / self.width))
    }
    pub fn has_point(&self, point: Point) -> bool {
        self.top_left.x <= point.x
            && self.top_left.y <= point.y
            && (self.top_left.x + self.width) > point.x
            && (self.top_left.y + self.height) > point.y
    }
}
