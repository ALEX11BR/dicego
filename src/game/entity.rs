use super::point::Point;

#[derive(Debug)]
pub struct Entity {
    pub position: Point,
}
impl Entity {
    pub fn new_player(position: Point) -> Self {
        Entity { position }
    }
}
