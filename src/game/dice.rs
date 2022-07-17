use rand::{prelude::SliceRandom, thread_rng, Rng};

use super::point::Point;

#[derive(Debug, Clone, Copy)]
pub struct Dice {
    pub position: Point,
    pub top: i8,
    pub left: i8,
    pub bottom: i8,
    pub right: i8,
    pub front: i8,
    pub back: i8,
}
impl Dice {
    pub fn new(position: Point, top: i8, left: i8, front: i8) -> Self {
        Dice {
            position,
            top,
            left,
            bottom: 7 - top,
            right: 7 - left,
            front,
            back: 7 - front,
        }
    }
    pub fn generate(position: Point) -> Self {
        let mut values = [1, 2, 3];
        values.shuffle(&mut thread_rng());
        for value in values.iter_mut() {
            if thread_rng().gen_bool(0.5) {
                let new = 7 - *value;
                *value = new;
            }
        }

        Dice::new(position, values[0], values[1], values[2])
    }
    pub fn move_by(&mut self, move_by: Point) {
        if move_by.x == 1 {
            let aux = self.top;
            self.top = self.left;
            self.left = self.bottom;
            self.bottom = self.right;
            self.right = aux;
        }
        if move_by.x == -1 {
            let aux = self.top;
            self.top = self.right;
            self.right = self.bottom;
            self.bottom = self.left;
            self.left = aux;
        }
        if move_by.y == 1 {
            let aux = self.top;
            self.top = self.back;
            self.back = self.bottom;
            self.bottom = self.front;
            self.front = aux;
        }
        if move_by.y == -1 {
            let aux = self.top;
            self.top = self.front;
            self.front = self.bottom;
            self.bottom = self.back;
            self.back = aux;
        }
    }
}
