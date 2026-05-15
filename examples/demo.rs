use raylib::prelude::*;
use rot_faster_rust::char_set::Glyph;
use rot_faster_rust::glyph_buffer::{Container, BorderStyle};
use rot_faster_rust::glyph_buffer::Button;
use rot_faster_rust::{char_set, glyph_buffer};

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Rot Faster - Demo")
        .build();

    rl.enable_cursor();

    let char_set = char_set::CharSet::new(
        &mut rl,
        &thread,
        "cp437_16x16.png",
        16, 16, 16, 16,
    );

    let mut strict_container = Container::new(28, 50, 18, 6, false, BorderStyle::Single);
    let mut dynamic_container = Container::new(28, 200, 18, 6, true, BorderStyle::Single);
    let mut button = Button::new(
        Container::new(28, 350, 14, 3, true, BorderStyle::Single),
        " Click Me ".to_string(),
    );

    let mut click_count = 0;

    while !rl.window_should_close() {
        let mouse_x = rl.get_mouse_x();
        let mouse_y = rl.get_mouse_y();
        let mouse_down = rl.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_BUTTON_LEFT);

        if button.update(mouse_x, mouse_y, mouse_down, &char_set) {
            click_count += 1;
            println!("Button clicked! Total clicks: {}", click_count);
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::new(20, 20, 30, 255));

        glyph_buffer::draw_glyph_string(&mut d, &char_set, "=== ROT FASTER DEMO ===", 28, 12);

        glyph_buffer::draw_glyph_string(&mut d, &char_set, "Strict Container:", 28, 36);
        strict_container.draw_text(
            &mut d,
            &char_set,
            "This text wraps inside a fixed-size container. Extra lines are clipped.",
        );
        strict_container.draw_border(&mut d, &char_set);

        glyph_buffer::draw_glyph_string(&mut d, &char_set, "Dynamic Container:", 28, 186);
        dynamic_container.draw_text(
            &mut d,
            &char_set,
            "This container resizes\nto fit its content.",
        );
        dynamic_container.draw_border(&mut d, &char_set);

        glyph_buffer::draw_glyph_string(&mut d, &char_set, "Button:", 28, 336);
        button.draw(&mut d, &char_set);

        let count_text = format!("Clicks: {}", click_count);
        glyph_buffer::draw_glyph_string(&mut d, &char_set, &count_text, 28, 400);

        glyph_buffer::draw_glyph_string(&mut d, &char_set, "Glyphs:", 340, 36);
        let glyphs = [
            (Glyph::Player, 340),
            (Glyph::Wall, 356),
            (Glyph::Floor, 372),
            (Glyph::Door, 388),
            (Glyph::StairsDown, 404),
            (Glyph::StairsUp, 420),
            (Glyph::UpArrow, 340),
            (Glyph::DownArrow, 356),
            (Glyph::RightArrow, 372),
            (Glyph::LeftArrow, 388),
            (Glyph::FullBlock, 340),
            (Glyph::UpperHalfBlock, 356),
            (Glyph::LowerHalfBlock, 372),
            (Glyph::LeftHalfBlock, 388),
            (Glyph::RightHalfBlock, 404),
        ];
        let mut gy = 52;
        for (i, (glyph, gx)) in glyphs.iter().enumerate() {
            if i == 7 {
                gy += 16;
            }
            if i == 11 {
                gy += 16;
            }
            glyph_buffer::draw_glyph(&mut d, &char_set, *glyph, *gx, gy);
        }
    }
}
