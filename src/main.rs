use game::dice::Dice;
use game::playeraction::PlayerAction;
use game::room::Room;
use game::{gamestate::GameState, point::Point, tile::Tile};
use raylib::consts::KeyboardKey;
use raylib::prelude::*;

mod game;

const TILE_SIZE: i32 = 64;
const SCREEN_WIDTH: i32 = 1280;
const SCREEN_HEIGHT: i32 = 720;
const TIME_LIMIT: f64 = 60.0;

fn render_tile(d: &mut RaylibDrawHandle, texture: &Texture2D, position: Point, tint: Color) {
    d.draw_texture(
        texture,
        TILE_SIZE * position.x,
        TILE_SIZE * position.y,
        tint,
    );
}

fn render_die(
    d: &mut RaylibDrawHandle,
    dice_texture: &Texture2D,
    position: Point,
    die: &Dice,
    tint: Color,
) {
    render_tile(d, dice_texture, position, tint);
    d.draw_text(
        &die.top.to_string(),
        TILE_SIZE * position.x + 30,
        TILE_SIZE * position.y + 10,
        30,
        Color::BLACK,
    );
    d.draw_text(
        &die.left.to_string(),
        TILE_SIZE * position.x + 10,
        TILE_SIZE * position.y + 30,
        15,
        Color::BLACK,
    );
    d.draw_text(
        &die.front.to_string(),
        TILE_SIZE * position.x + 30,
        TILE_SIZE * position.y + 45,
        15,
        Color::BLACK,
    );
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("dicego")
        .build();
    rl.set_target_fps(60);
    rl.set_exit_key(None);

    let exit_point_texture = rl.load_texture(&thread, "img/ExitPoint.png").unwrap();
    let floor_texture = rl.load_texture(&thread, "img/Floor.png").unwrap();
    let wall_texture = rl.load_texture(&thread, "img/Wall.png").unwrap();
    let player_texture = rl.load_texture(&thread, "img/Player.png").unwrap();
    let dice_texture = rl.load_texture(&thread, "img/Dice3D.png").unwrap();

    let mut game = GameState::new(rl.get_time());
    let mut game_over = false;

    while !rl.window_should_close() {
        let current_time = rl.get_time() - game.game_start;
        if current_time >= TIME_LIMIT {
            game_over = true;
        }

        if game_over {
            if current_time >= (TIME_LIMIT + 1.0) {
                if let Some(_) = rl.get_key_pressed() {
                    game_over = false;
                    game = GameState::new(rl.get_time());
                }
            }

            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::BLACK);

            d.draw_text("GAME OVER!", 5, 5, 50, Color::WHITE);
            d.draw_text(
                &format!("You ended the game at level {}.", game.current_level),
                5,
                63,
                30,
                Color::WHITE,
            );
            d.draw_text(
                if current_time >= (TIME_LIMIT + 1.0) {
                    "Press any key to try again"
                } else {
                    "Please wait..."
                },
                5,
                150,
                30,
                Color::WHITE,
            );
            continue;
        }

        match rl.get_key_pressed() {
            Some(KeyboardKey::KEY_LEFT) => {
                game.next_turn(PlayerAction::MoveBy(Point::new(-1, 0)));
            }
            Some(KeyboardKey::KEY_DOWN) => {
                game.next_turn(PlayerAction::MoveBy(Point::new(0, 1)));
            }
            Some(KeyboardKey::KEY_UP) => {
                game.next_turn(PlayerAction::MoveBy(Point::new(0, -1)));
            }
            Some(KeyboardKey::KEY_RIGHT) => {
                game.next_turn(PlayerAction::MoveBy(Point::new(1, 0)));
            }
            Some(KeyboardKey::KEY_SPACE) => {
                game.next_turn(PlayerAction::MoveBy(Point::new(0, 0)));
            }
            Some(KeyboardKey::KEY_ENTER | KeyboardKey::KEY_KP_ENTER) => {
                game.next_turn(PlayerAction::Select(rl.get_time()));
            }
            _ => {}
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        let top_left_display = game.player.position - Point::new(10, 5);
        let display_room = Room::new(20, 10, top_left_display);

        for point in display_room.all_points() {
            render_tile(
                &mut d,
                match game.level.tiles[point] {
                    Tile::ExitPoint => &exit_point_texture,
                    Tile::Floor | Tile::EntryPoint => &floor_texture,
                    Tile::Wall => &wall_texture,
                },
                point - top_left_display,
                if game.level.last_seen[point] == game.current_turn {
                    Color::WHITE
                } else if game.level.last_seen[point] > 0 {
                    Color::GRAY
                } else {
                    Color::BLACK
                },
            );
        }

        for die in game.level.dice.iter() {
            if display_room.has_point(die.position) {
                render_die(
                    &mut d,
                    &dice_texture,
                    die.position - top_left_display,
                    die,
                    if die.top == game.level.top_target {
                        Color::GREEN
                    } else if game.level.last_seen[die.position] == game.current_turn {
                        Color::WHITE
                    } else if game.level.last_seen[die.position] > 0 {
                        Color::GRAY
                    } else {
                        Color::BLACK
                    },
                );
            }
        }

        render_tile(
            &mut d,
            &player_texture,
            game.player.position - top_left_display,
            Color::WHITE,
        );

        let good_dice = game
            .level
            .dice
            .iter()
            .filter(|d| d.top == game.level.top_target)
            .count();
        let dice_number = game.level.dice.len();

        d.draw_text(
            &format!(
                "Level {}. Time: {:.0}/{TIME_LIMIT}s. Place all dice with '{}' on top. Target {}/{}.",
                game.current_level, current_time, game.level.top_target, good_dice, dice_number
            ),
            0,
            SCREEN_HEIGHT - TILE_SIZE,
            32,
            if good_dice == dice_number {
                Color::GREEN
            } else if current_time >= (game.game_start + TIME_LIMIT*0.9) {
                Color::ORANGE
            } else {
                Color::WHITE
            },
        );
    }
}
