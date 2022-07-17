#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Wall,
    Floor,
    EntryPoint,
    ExitPoint,
}
impl Tile {
    pub fn is_walkable(&self) -> bool {
        match self {
            Tile::Wall => false,
            _ => true,
        }
    }
}
