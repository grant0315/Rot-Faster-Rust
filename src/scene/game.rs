use crate::glyph_buffer::GlyphBuffer;
use crate::input::Direction;
use crate::level_generation::{Level, LevelGenAlgorithm, TileType};
use crate::player::Player;
use crate::scene::{Scene, SceneContext, SceneResult};
use crate::ui::{self, UIElement};
use crate::xp_parse::{Sprite, XpFile};
use raylib::prelude::{KeyboardKey, RaylibDrawHandle, RaylibHandle};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RelativeWall {
    Front,
    LeftSide,
    RightSide,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WallVisibility {
    pub front: bool,
    pub left: bool,
    pub right: bool,
}

impl WallVisibility {
    pub fn new() -> Self {
        WallVisibility {
            front: false,
            left: false,
            right: false,
        }
    }

    pub fn any(&self) -> bool {
        self.front || self.left || self.right
    }

    pub fn as_vec(&self) -> Vec<RelativeWall> {
        let mut walls = Vec::new();
        if self.front {
            walls.push(RelativeWall::Front);
        }
        if self.left {
            walls.push(RelativeWall::LeftSide);
        }
        if self.right {
            walls.push(RelativeWall::RightSide);
        }
        walls
    }
}

pub struct GameScene {
    pub level: Level,
    pub player: Player,
    pub is_minimap_open: bool,
    pub background_sprite: Sprite,
    pub left_wall_sprite: Sprite,
    pub middle_wall_sprite: Sprite,
    pub right_wall_sprite: Sprite,
}

impl GameScene {
    pub fn new(gbuf_cols: usize, gbuf_rows: usize) -> Self {
        let mut level = Level::new(gbuf_cols, gbuf_rows, (40, 30));
        level.generate(
            LevelGenAlgorithm::CellularAutomataCave,
            gbuf_cols,
            gbuf_rows,
        );

        let mut xp_file: XpFile = XpFile::new("assets/background.xp".to_string());
        let background_sprite: Sprite = xp_file.parse();

        xp_file = XpFile::new("assets/wall-center.xp".to_string());
        let middle_wall_sprite: Sprite = xp_file.parse();

        xp_file = XpFile::new("assets/wall-left.xp".to_string());
        let left_wall_sprite: Sprite = xp_file.parse();

        xp_file = XpFile::new("assets/wall-right.xp".to_string());
        let right_wall_sprite: Sprite = xp_file.parse();

        Self {
            level,
            player: Player::new("Test Player", 10, 10),
            is_minimap_open: false,
            background_sprite,
            left_wall_sprite,
            middle_wall_sprite,
            right_wall_sprite,
        }
    }
}

impl Scene for GameScene {
    fn update(&mut self, ctx: &mut RaylibHandle, _scene_ctx: &mut SceneContext) -> SceneResult {
        self.player.update();

        if ctx.is_key_pressed(KeyboardKey::KEY_W) || ctx.is_key_pressed(KeyboardKey::KEY_UP) {
            self.try_move_player(self.player.facing);
        } else if ctx.is_key_pressed(KeyboardKey::KEY_S)
            || ctx.is_key_pressed(KeyboardKey::KEY_DOWN)
        {
            let back = self.player.facing.opposite();
            self.try_move_player(back);
        } else if ctx.is_key_pressed(KeyboardKey::KEY_A)
            || ctx.is_key_pressed(KeyboardKey::KEY_LEFT)
        {
            self.player.facing = self.player.facing.turn_left();
        } else if ctx.is_key_pressed(KeyboardKey::KEY_D)
            || ctx.is_key_pressed(KeyboardKey::KEY_RIGHT)
        {
            self.player.facing = self.player.facing.turn_right();
        } else if ctx.is_key_pressed(KeyboardKey::KEY_M) {
            self.is_minimap_open = !self.is_minimap_open;
        }

        return SceneResult::None;
    }

    fn draw(&mut self, _d: &mut RaylibDrawHandle, scene_ctx: &mut SceneContext) {
        scene_ctx.glyph_buffer.clear(raylib::prelude::Color::BLACK);

        if self.is_minimap_open {
            self.level.render_to_buffer(&mut scene_ctx.glyph_buffer);
            self.player.draw(&mut scene_ctx.glyph_buffer);
            return;
        }

        self.background_sprite
            .draw(&mut scene_ctx.glyph_buffer, 0, 0);

        let walls = determine_player_wall_vis(&self.player, &self.level);

        if walls.front {
            self.middle_wall_sprite
                .draw(&mut scene_ctx.glyph_buffer, 0, 0);
        }

        if walls.left {
            self.left_wall_sprite
                .draw(&mut scene_ctx.glyph_buffer, 0, 0);
        }

        if walls.right {
            self.right_wall_sprite
                .draw(&mut scene_ctx.glyph_buffer, 0, 0);
        }

        // Draw ui
        draw_ui(&mut scene_ctx.glyph_buffer, &mut self.player);
    }
}

impl GameScene {
    fn try_move_player(&mut self, dir: Direction) {
        let (dx, dy) = match dir {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        let next_x = self.player.tile_pos_x + dx;
        let next_y = self.player.tile_pos_y + dy;
        if self.level.is_walkable(next_x, next_y) {
            self.player.move_delta(dx, dy);
        }
    }
}

fn determine_player_wall_vis(player: &Player, level: &Level) -> WallVisibility {
    let px = player.tile_pos_x;
    let py = player.tile_pos_y;

    let height = level.level_grid.len() as i32;
    let width = level.level_grid[0].len() as i32;

    let (front_dx, front_dy, left_dx, left_dy, right_dx, right_dy) = match player.facing {
        Direction::Up => (0, -1, -1, 0, 1, 0),
        Direction::Down => (0, 1, 1, 0, -1, 0),
        Direction::Left => (-1, 0, 0, 1, 0, -1),
        Direction::Right => (1, 0, 0, -1, 0, 1),
    };

    let is_wall = |x: i32, y: i32| -> bool {
        if x < 0 || x >= width || y < 0 || y >= height {
            return true;
        }
        level.level_grid[y as usize][x as usize] == TileType::Wall
    };

    let fx = px + front_dx;
    let fy = py + front_dy;
    let lx = px + left_dx;
    let ly = py + left_dy;
    let rx = px + right_dx;
    let ry = py + right_dy;

    let front = is_wall(fx, fy);
    let left = is_wall(lx, ly);
    let right = is_wall(rx, ry);

    eprintln!(
        "Facing: {:?}, Front: {}, Left: {}, Right: {}",
        player.facing, front, left, right
    );

    WallVisibility { front, left, right }
}

fn draw_ui(gb: &mut GlyphBuffer, player: &Player) {
    let hp_text: String = format!("HEALTH: {}", player.health);
    let mana_text = format!("MANA: {}", player.mana);

    let mut stats = UIElement::vstack_bordered(
        vec![
            UIElement::text_bordered(hp_text),
            UIElement::text_bordered(mana_text),
        ],
        0,
    );

    ui::draw(
        gb,
        &mut stats,
        40,
        10,
        raylib::prelude::Color::WHITE,
        raylib::prelude::Color::BLACK,
    );
}
