use raylib::prelude::{RaylibDrawHandle, RaylibHandle};

pub mod game;
pub mod main_menu;

use crate::glyph_buffer::GlyphBuffer;

pub struct SceneContext {
    pub glyph_buffer: GlyphBuffer,
    pub mouse_col: i32,
    pub mouse_row: i32,
    pub mouse_down: bool,
}

impl SceneContext {
    pub fn new(char_set: crate::char_set::CharSet) -> Self {
        Self {
            glyph_buffer: GlyphBuffer::new(char_set, 16, 16, 80, 60),
            mouse_col: 0,
            mouse_row: 0,
            mouse_down: false,
        }
    }

    pub fn update_input(&mut self, ctx: &mut RaylibHandle) {
        let mouse_x = ctx.get_mouse_x();
        let mouse_y = ctx.get_mouse_y();
        self.mouse_down = ctx.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_BUTTON_LEFT);
        let (mcol, mrow) = self.glyph_buffer.pixel_to_grid(mouse_x, mouse_y);
        self.mouse_col = mcol;
        self.mouse_row = mrow;
    }
}

pub trait Scene {
    fn update(&mut self, ctx: &mut RaylibHandle, scene_ctx: &mut SceneContext) -> SceneResult;
    fn draw(&mut self, d: &mut RaylibDrawHandle, scene_ctx: &mut SceneContext);
}

pub enum SceneResult {
    None,
    Push(Box<dyn Scene>),
    Pop,
    Exit,
}
