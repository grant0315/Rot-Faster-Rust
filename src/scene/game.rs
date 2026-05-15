use raylib::prelude::{RaylibDrawHandle, RaylibHandle};

use crate::scene::{Scene, SceneContext, SceneResult};

pub struct GameScene;

impl GameScene {
    pub fn new() -> Self {
        Self
    }
}

impl Scene for GameScene {
    fn update(&mut self, _ctx: &mut RaylibHandle, _scene_ctx: &mut SceneContext) -> SceneResult {
        SceneResult::None
    }

    fn draw(&mut self, d: &mut RaylibDrawHandle, scene_ctx: &mut SceneContext) {
        scene_ctx.glyph_buffer.clear(raylib::prelude::Color::BLACK);
        scene_ctx.glyph_buffer.put_text(
            "Game Scene - Press ESC to return",
            20,
            30,
            raylib::prelude::Color::WHITE,
            raylib::prelude::Color::BLACK,
        );
    }
}
