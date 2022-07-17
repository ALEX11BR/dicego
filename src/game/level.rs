use super::{array2d::Array2D, dice::Dice, point::Point, room::Room, tile::Tile};
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Level {
    pub width: i32,
    pub height: i32,
    pub tiles: Array2D<Tile>,
    pub last_seen: Array2D<i32>,
    pub starting_point: Point,
    pub top_target: i8,
    pub dice: Vec<Dice>,
}
impl Level {
    pub fn generate(current_level: i32) -> Self {
        let level_width = 35 + current_level;
        let level_height = level_width;

        let mut tiles = Array2D::new(level_width, level_height, Tile::Wall);
        let last_seen = Array2D::new(level_width, level_height, 0);

        let mut prev_room = Room::generate(level_width, level_height);
        for point in prev_room.all_points() {
            tiles[point] = Tile::Floor;
        }
        for _ in 0..(10 + current_level / 10) {
            let new_room = Room::generate(level_width, level_height);

            for point in new_room.all_points() {
                tiles[point] = Tile::Floor;
            }

            let new_room_point = new_room.generate_point();
            let prev_room_point = prev_room.generate_point();
            if thread_rng().gen_bool(0.5) {
                tiles.carve_h_corridor(prev_room_point.x, new_room_point.x, prev_room_point.y);
                tiles.carve_v_corridor(new_room_point.x, prev_room_point.y, new_room_point.y);
            } else {
                tiles.carve_h_corridor(prev_room_point.x, new_room_point.x, new_room_point.y);
                tiles.carve_v_corridor(prev_room_point.x, prev_room_point.y, new_room_point.y);
            }

            prev_room = new_room;
        }
        let starting_point = prev_room.generate_point();
        tiles[starting_point] = Tile::EntryPoint;

        let exit_point = tiles.generate_floor_point();
        tiles[exit_point] = Tile::ExitPoint;

        let top_target = thread_rng().gen_range(1..=6);

        let mut dice = Vec::with_capacity((6 + current_level / 2) as usize);
        for _ in 0..(6 + current_level / 2) {
            let die_point = loop {
                let proposed_point = tiles.generate_floor_point();
                if proposed_point
                    .diagonal_neighbors()
                    .all(|p| tiles[p].is_walkable())
                    && dice.iter().all(|d: &Dice| d.position != proposed_point)
                {
                    break proposed_point;
                }
            };
            let mut die_proposal = Dice::generate(die_point);
            if die_proposal.top == top_target {
                die_proposal.top = 7 - die_proposal.top;
                die_proposal.bottom = 7 - die_proposal.bottom;
            }
            dice.push(die_proposal);
        }

        Level {
            width: level_width,
            height: level_height,
            tiles,
            last_seen,
            starting_point,
            top_target,
            dice,
        }
    }
}
