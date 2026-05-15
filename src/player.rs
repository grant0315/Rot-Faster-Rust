use raylib::prelude::*;
use crate::item::Item;
use crate::input::Direction;
use crate::level_generation;

pub struct Player {
    pub tile_pos_x: i32,
    pub tile_pos_y: i32,
    pub name: String,
    pub health: u32,
    pub mana: u32,
    pub inventory: Inventory,
}

pub struct Inventory {
    pub items: Vec<Item>,
}

impl Player {
    pub fn new(name: &str, tile_pos_x: i32, tile_pos_y: i32) -> Self {
        Player {
            name: name.to_string(),
            tile_pos_x: tile_pos_x,
            tile_pos_y: tile_pos_y,
            health: 100,
            mana: 50,
            inventory: Inventory { items: vec![] },
        }
    }

    pub fn update(&mut self) {
        // Movement and other logic would go here
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) { // Draw the player character at its current position For example, you could draw a simple rectangle or a sprite
        d.draw_rectangle(
            self.tile_pos_x * 16,
            self.tile_pos_y * 16,
            16,
            16,
            Color::WHITE,
        );
    }

    pub fn move_delta(&mut self, dx: i32, dy: i32) {
        self.tile_pos_x += dx;
        self.tile_pos_y += dy;
    }

    pub fn move_direction(&mut self, direction: crate::input::Direction) {
        match direction {
            Direction::Up => self.move_delta(0, -1),
            Direction::Down => self.move_delta(0, 1),
            Direction::Left => self.move_delta(-1, 0),
            Direction::Right => self.move_delta(1, 0),
        }
    }

    pub fn check_collision(&self, tile: &level_generation::Tile, max_x: i32, max_y: i32) -> bool {
        // Check if the desired tile position is out of bounds
        if self.check_out_of_bounds(max_x, max_y) {
            return false;
        }

        // Check if the tile is a wall
        if self.check_wall_collision(tile) {
            return false;
        }

        // If we passed both checks, the player can move there
        true
    }
        

    fn check_out_of_bounds(&self, max_x: i32, max_y: i32) -> bool {
        self.tile_pos_x < 0
            || self.tile_pos_y < 0
            || self.tile_pos_x >= max_x as i32
            || self.tile_pos_y >= max_y as i32
    }

    fn check_wall_collision(&self, tile: &level_generation::Tile) -> bool {
        // Check if the tile is a wall
        matches!(tile.tile_type, level_generation::TileType::Wall)
    }

    pub fn take_damage(&mut self, amount: u32) {
        if amount >= self.health {
            self.health = 0;
        } else {
            self.health -= amount;
        }
    }

    pub fn heal(&mut self, amount: u32) {
        self.health += amount;
        if self.health > 100 {
            self.health = 100;
        }
    }

    pub fn use_mana(&mut self, amount: u32) -> bool {
        if amount > self.mana {
            false
        } else {
            self.mana -= amount;
            true
        }
    }

    pub fn add_item(&mut self, item: Item) {
        self.inventory.items.push(item);
    }
}
