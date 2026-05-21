use crate::char_set::Glyph;
use crate::glyph_buffer::GlyphBuffer;
use crate::input::Direction;
use crate::item::Item;
use raylib::prelude::*;

pub struct Player {
    pub tile_pos_x: i32,
    pub tile_pos_y: i32,
    pub facing: Direction,
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
            facing: Direction::Down,
            health: 100,
            mana: 50,
            inventory: Inventory { items: vec![] },
        }
    }

    pub fn update(&mut self) {
        // Movement and other logic would go here
    }

    pub fn draw(&self, gb: &mut GlyphBuffer) {
        // Draw the player character at its current position For example, you could draw a simple rectangle or a sprite
        gb.put_glyph(
            self.tile_pos_x,
            self.tile_pos_y,
            Glyph::Player,
            Color::WHITE,
            Color::BLACK,
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

    fn check_out_of_bounds(&self, max_x: i32, max_y: i32) -> bool {
        self.tile_pos_x < 0
            || self.tile_pos_y < 0
            || self.tile_pos_x >= max_x as i32
            || self.tile_pos_y >= max_y as i32
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
