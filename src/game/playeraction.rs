use super::point::Point;

#[derive(Debug, Clone, Copy)]
pub enum PlayerAction {
    MoveBy(Point),
    Select(f64),
}
