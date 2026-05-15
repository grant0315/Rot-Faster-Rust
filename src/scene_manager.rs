use raylib::prelude::{RaylibDrawHandle, RaylibHandle};

use crate::scene::{Scene, SceneContext, SceneResult};

pub struct SceneManager {
    scenes: Vec<Box<dyn Scene>>,
    scene_ctx: SceneContext,
}

impl SceneManager {
    pub fn new(initial_scene: Box<dyn Scene>, char_set: crate::char_set::CharSet) -> Self {
        Self {
            scenes: vec![initial_scene],
            scene_ctx: SceneContext::new(char_set),
        }
    }

    pub fn update(&mut self, ctx: &mut RaylibHandle) -> bool {
        self.scene_ctx.update_input(ctx);

        if let Some(active_scene) = self.scenes.last_mut() {
            match active_scene.update(ctx, &mut self.scene_ctx) {
                SceneResult::Push(new_scene) => self.scenes.push(new_scene),
                SceneResult::Pop => {
                    self.scenes.pop();
                }
                SceneResult::Exit => return false,
                SceneResult::None => {}
            }
        }
        true
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        if let Some(active_scene) = self.scenes.last_mut() {
            active_scene.draw(d, &mut self.scene_ctx);
        }
        self.scene_ctx.glyph_buffer.render(d);
    }

    pub fn resize(&mut self, screen_w: i32, screen_h: i32) {
        self.scene_ctx.glyph_buffer.resize(screen_w, screen_h);
    }
}
