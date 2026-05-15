use raylib::prelude::*;
use rot_faster_rust::char_set;
use rot_faster_rust::scene::main_menu::MainMenu;
use rot_faster_rust::scene_manager::SceneManager;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1280, 960)
        .resizable()
        .title("Rot Faster")
        .build();

    rl.enable_cursor();

    let char_set = char_set::CharSet::new(&mut rl, &thread, "cp437_16x16.png", 16, 16, 16, 16);

    let mut scene_manager = SceneManager::new(Box::new(MainMenu::new()), char_set);

    while !rl.window_should_close() {
        if rl.is_window_resized() {
            scene_manager.resize(rl.get_screen_width(), rl.get_screen_height());
        }

        if !scene_manager.update(&mut rl) {
            break;
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        scene_manager.draw(&mut d);
    }
}
