use raylib::prelude::*;

pub enum Direction {
    Up,
    Down,
    Left, 
    Right
}

pub fn get_movement_input(rl: &RaylibHandle, player_tile_x: i32, player_tile_y: i32) -> Option<Direction> {
    // Check for keyboard input first
    if let Some(direction) = check_grid_key_movement(rl) {
        return Some(direction);
    }

    // If no keyboard input, check for mouse input on click
    if rl.is_mouse_button_pressed(raylib::consts::MouseButton::MOUSE_BUTTON_LEFT) {
        return check_grid_mouse_movement(rl, player_tile_x, player_tile_y);
    } else {
        // No input detected
        return None;
    }
}

// Function to check for single-press grid movement
fn check_grid_key_movement(rl: &RaylibHandle) -> Option<Direction> {
    if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_W) {
        return Some(Direction::Up);
    }
    if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_S) {
        return Some(Direction::Down);
    }
    if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_A) {
        return Some(Direction::Left);
    }
    if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_D) {
        return Some(Direction::Right);
    }
    None
}

fn check_grid_mouse_movement(rl: &RaylibHandle, player_tile_x: i32, player_tile_y: i32) -> Option<Direction> {
    let mouse_x = rl.get_mouse_x();
    let mouse_y = rl.get_mouse_y();

    let tile_x = mouse_x / 16;
    let tile_y = mouse_y / 16;

    // Determine direction based on relative position of clicked tile to player tile
    if tile_x == player_tile_x && tile_y < player_tile_y {
        return Some(Direction::Up);
    } else if tile_x == player_tile_x && tile_y > player_tile_y {
        return Some(Direction::Down);
    } else if tile_y == player_tile_y && tile_x < player_tile_x {
        return Some(Direction::Left);
    } else if tile_y == player_tile_y && tile_x > player_tile_x {
        return Some(Direction::Right);
    } else if tile_x < player_tile_x && tile_y < player_tile_y {
        // Clicked tile is diagonally up-left, prioritize vertical movement
        return Some(Direction::Up);
    } else if tile_x > player_tile_x && tile_y < player_tile_y {
        // Clicked tile is diagonally up-right, prioritize vertical movement
        return Some(Direction::Up);
    } else if tile_x < player_tile_x && tile_y > player_tile_y {
        // Clicked tile is diagonally down-left, prioritize vertical movement
        return Some(Direction::Down);
    } else if tile_x > player_tile_x && tile_y > player_tile_y {
        // Clicked tile is diagonally down-right, prioritize vertical movement
        return Some(Direction::Down);
    }
    None
}
