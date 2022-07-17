use super::{entity::Entity, level::Level, playeraction::PlayerAction, point::Point, tile::Tile};
use symmetric_shadowcasting::compute_fov;

#[derive(Debug)]
pub struct GameState {
    pub level: Level,
    pub current_level: i32,
    pub current_turn: i32,
    pub game_start: f64,
    pub player: Entity,
}
impl GameState {
    pub fn new(game_start: f64) -> Self {
        let mut game = GameState {
            level: Level::generate(1),
            current_level: 1,
            current_turn: 1,
            game_start,
            player: Entity::new_player(Point::new(2, 2)),
        };
        game.player.position = game.level.starting_point;
        game.update_fov();

        game
    }
    fn update_fov(&mut self) {
        let Point { x, y } = self.player.position;
        compute_fov(
            (y as isize, x as isize),
            &mut |(y, x)| self.level.tiles[Point::new(x as i32, y as i32)] == Tile::Wall,
            &mut |(y, x)| {
                self.level.last_seen[Point::new(x as i32, y as i32)] = self.current_turn;
            },
        )
    }
    pub fn next_turn(&mut self, action: PlayerAction) {
        self.current_turn += 1;
        match action {
            PlayerAction::MoveBy(move_by) => {
                let proposed_position = self.player.position + move_by;
                if self.level.tiles[proposed_position].is_walkable() {
                    if self
                        .level
                        .dice
                        .iter()
                        .any(|d| d.position == proposed_position)
                    {
                        let new_die_position = proposed_position + move_by;
                        if self.level.tiles[new_die_position].is_walkable()
                            && self
                                .level
                                .dice
                                .iter()
                                .all(|d| d.position != new_die_position)
                        {
                            if let Some(mut die) = self
                                .level
                                .dice
                                .iter_mut()
                                .find(|d| d.position == proposed_position)
                            {
                                die.position = new_die_position;
                                die.move_by(move_by);
                                self.player.position = proposed_position;
                            }
                        }
                    } else {
                        self.player.position = proposed_position;
                    }
                }
            }
            PlayerAction::Select(time) => match self.level.tiles[self.player.position] {
                Tile::ExitPoint => {
                    if self
                        .level
                        .dice
                        .iter()
                        .all(|d| d.top == self.level.top_target)
                    {
                        self.current_level += 1;
                        self.level = Level::generate(self.current_level);
                        self.player.position = self.level.starting_point;
                        self.game_start = time;
                    }
                }
                _ => {}
            },
        }
        self.update_fov();
    }
}
