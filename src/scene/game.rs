use crate::input::Direction;
use crate::level_generation::{Level, LevelGenAlgorithm};
use crate::player::Player;
use crate::scene::{Scene, SceneContext, SceneResult};
use raylib::prelude::{KeyboardKey, RaylibDrawHandle, RaylibHandle};

pub struct GameScene {
    pub level: Level,
    pub player: Player,
}

impl GameScene {
    pub fn new(gbuf_cols: usize, gbuf_rows: usize) -> Self {
        let mut level = Level::new(gbuf_cols, gbuf_rows, (40, 30));
        level.generate(
            LevelGenAlgorithm::CellularAutomataCave,
            gbuf_cols,
            gbuf_rows,
        );
        Self {
            level,
            player: Player::new("Test Player", 10, 10),
        }
    }
}

impl Scene for GameScene {
    fn update(&mut self, ctx: &mut RaylibHandle, _scene_ctx: &mut SceneContext) -> SceneResult {
        self.player.update();

        // Check for input from raylib
        if ctx.is_key_pressed(KeyboardKey::KEY_W) {
            self.try_move_player(Direction::Up);
        } else if ctx.is_key_pressed(KeyboardKey::KEY_S) {
            self.try_move_player(Direction::Down);
        } else if ctx.is_key_pressed(KeyboardKey::KEY_A) {
            self.try_move_player(Direction::Left);
        } else if ctx.is_key_pressed(KeyboardKey::KEY_D) {
            self.try_move_player(Direction::Right);
        }

        return SceneResult::None;
    }

    fn draw(&mut self, _d: &mut RaylibDrawHandle, scene_ctx: &mut SceneContext) {
        scene_ctx.glyph_buffer.clear(raylib::prelude::Color::BLACK);

        self.level.render_to_buffer(&mut scene_ctx.glyph_buffer);
        self.player.draw(&mut scene_ctx.glyph_buffer);
    }
}

impl GameScene {
    fn try_move_player(&mut self, dir: Direction) {
        let (dx, dy) = match dir {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        let next_x = self.player.tile_pos_x + dx;
        let next_y = self.player.tile_pos_y + dy;
        if self.level.is_walkable(next_x, next_y) {
            self.player.move_delta(dx, dy);
        }
    }
}
