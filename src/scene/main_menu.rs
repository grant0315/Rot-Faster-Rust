use raylib::prelude::{RaylibDrawHandle, RaylibHandle};

use crate::scene::{Scene, SceneContext, SceneResult};
use crate::ui::{self, UIElement};

pub struct MainMenu {
    ui: UIElement,
    test_menu: UIElement,
    cell_menu: UIElement,
    current_ui: UIElement,
    last_click: String,
}

impl MainMenu {
    pub fn new() -> Self {
        let main_menu = UIElement::vstack_bordered(
            vec![
                UIElement::text_bordered("   ===  ROT  FASTER  ===   "),
                UIElement::text(""),
                UIElement::text("An old-school roguelike"),
                UIElement::text("built from scratch in Rust."),
                UIElement::text(""),
                UIElement::button("new", "  New Game  "),
                UIElement::button("load", "  Load Game  "),
                UIElement::button("conf", "  Configure  "),
                UIElement::text(""),
                UIElement::text_bordered("Debug"),
                UIElement::button("test", "  Run UI Test  "),
                UIElement::button("cell", "  Cell Viewer  "),
                UIElement::text(""),
                UIElement::button("quit", "  Quit  "),
            ],
            0,
        );

        let test_menu = UIElement::vstack_bordered(
            vec![
                UIElement::text_bordered("Test Suite"),
                UIElement::text(""),
                UIElement::button("test_a", "  Test A — Layout   "),
                UIElement::button("test_b", "  Test B — Borders  "),
                UIElement::button("test_c", "  Test C — Wrapping "),
                UIElement::text(""),
                UIElement::button("back", "  Back  "),
            ],
            0,
        );

        let cell_menu = UIElement::vstack_bordered(
            vec![
                UIElement::text_bordered("Cell Viewer"),
                UIElement::text(""),
                UIElement::text("Grid: 80 x 60"),
                UIElement::text("Cell size adapts to window."),
                UIElement::text("Aspect ratio: square cells."),
                UIElement::text(""),
                UIElement::button("back", "  Back  "),
            ],
            0,
        );

        Self {
            ui: main_menu.clone(),
            test_menu,
            cell_menu,
            current_ui: main_menu,
            last_click: String::new(),
        }
    }
}

impl Scene for MainMenu {
    fn update(&mut self, _ctx: &mut RaylibHandle, scene_ctx: &mut SceneContext) -> SceneResult {
        if let Some(id) = ui::update(
            &mut self.current_ui,
            scene_ctx.mouse_col,
            scene_ctx.mouse_row,
            scene_ctx.mouse_down,
        ) {
            self.last_click = id.clone();
            println!("Clicked: {}", id);

            match id.as_str() {
                "quit" => return SceneResult::Exit,
                "test" => self.current_ui = self.test_menu.clone(),
                "cell" => self.current_ui = self.cell_menu.clone(),
                "back" => self.current_ui = self.ui.clone(),
                "new" => return SceneResult::Push(Box::new(super::game::GameScene::new())),
                _ => {}
            }
        }

        SceneResult::None
    }

    fn draw(&mut self, d: &mut RaylibDrawHandle, scene_ctx: &mut SceneContext) {
        ui::draw(
            &mut scene_ctx.glyph_buffer,
            &mut self.current_ui,
            24,
            6,
            raylib::prelude::Color::WHITE,
            raylib::prelude::Color::BLACK,
        );
    }
}
