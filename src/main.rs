use raylib::prelude::*;
use rot_faster_rust::char_set;
use rot_faster_rust::glyph_buffer::GlyphBuffer;
use rot_faster_rust::ui::{self, UIElement};

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1280, 960)
        .resizable()
        .title("Rot Faster - UI Test")
        .build();

    rl.enable_cursor();

    let char_set = char_set::CharSet::new(
        &mut rl,
        &thread,
        "cp437_16x16.png",
        16, 16, 16, 16,
    );

    let mut gb = GlyphBuffer::new(char_set, 16, 16, 80, 60);

    // ---- Main menu ----
    let main_menu = UIElement::vstack_bordered(
        vec![
            UIElement::text_bordered("   ===  ROT  FASTER  ===   "),
            UIElement::text(""),
            UIElement::text("An old-school roguelike"),
            UIElement::text("built from scratch in Rust."),
            UIElement::text(""),
            UIElement::button("new",  "  New Game  "),
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

    // ---- Test sub-menu ----
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

    // ---- Cell viewer ----
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

    let mut current_ui = main_menu.clone();
    let mut last_click = String::new();

    while !rl.window_should_close() {
        let mouse_x = rl.get_mouse_x();
        let mouse_y = rl.get_mouse_y();
        let mouse_down = rl.is_mouse_button_down(
            raylib::consts::MouseButton::MOUSE_BUTTON_LEFT,
        );

        let (mcol, mrow) = gb.pixel_to_grid(mouse_x, mouse_y);

        if rl.is_window_resized() {
            gb.resize(rl.get_screen_width(), rl.get_screen_height());
        }

        if let Some(id) = ui::update(&mut current_ui, mcol, mrow, mouse_down) {
            last_click = id.clone();
            println!("Clicked: {}", id);

            match id.as_str() {
                "quit" => break,
                "test" => current_ui = test_menu.clone(),
                "cell" => current_ui = cell_menu.clone(),
                "back" => current_ui = main_menu.clone(),
                _ => {}
            }
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        gb.clear(Color::BLACK);

        // Column header
        gb.put_text("0         10        20        30        40        50        60        70", 0, 0, Color::new(40, 40, 40, 255), Color::BLACK);

        ui::draw(&mut gb, &mut current_ui, 24, 6, Color::WHITE, Color::BLACK);

        // Status bar
        let status = format!(
            "Last: {:<16} | Mouse: ({:>2},{:>2}) | Cell: {}x{} {}",
            last_click, mcol, mrow,
            gb.glyph_w, gb.glyph_h,
            if gb.glyph_w == gb.glyph_h { "[sq]" } else { "" },
        );
        gb.put_text(&status, 1, 58, Color::GRAY, Color::BLACK);

        gb.render(&mut d);
    }
}
