use raylib::prelude::{RaylibDrawHandle, RaylibHandle};

use crate::scene::{Scene, SceneContext, SceneResult};
use crate::level_generation::{Level, LevelGenAlgorithm};
use crate::player::Player;

pub struct GameScene {
    pub level: Level,
    pub player: Player,
}

impl GameScene {
    pub fn new(gbuf_cols: usize, gbuf_rows: usize) -> Self {
        let mut level = Level::new(gbuf_cols, gbuf_rows, (40, 30));
        level.generate(LevelGenAlgorithm::CellularAutomataCave, gbuf_cols, gbuf_rows);
        Self {
            level,
            player: Player::new("Test Player", 10, 10),
        }
    }
}

impl Scene for GameScene {
    fn update(&mut self, _ctx: &mut RaylibHandle, _scene_ctx: &mut SceneContext) -> SceneResult {
        SceneResult::None
    }

    fn draw(&mut self, d: &mut RaylibDrawHandle, scene_ctx: &mut SceneContext) {
        scene_ctx.glyph_buffer.clear(raylib::prelude::Color::BLACK);
        self.level.render_to_buffer(&mut scene_ctx.glyph_buffer);
    }
}
