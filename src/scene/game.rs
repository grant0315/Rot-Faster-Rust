use crate::input::Direction;
use crate::level_generation::{Level, LevelGenAlgorithm, TileType};
use crate::player::Player;
use crate::scene::{Scene, SceneContext, SceneResult};
use crate::xp_parse::{Sprite, XpFile};
use raylib::prelude::{KeyboardKey, RaylibDrawHandle, RaylibHandle};

pub enum RelativeWall {
    Front,
    LeftSide,
    RightSide,
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

        xp_file = XpFile::new("assets/stright-wall.xp".to_string());
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

        if ctx.is_key_pressed(KeyboardKey::KEY_W)
            || ctx.is_key_pressed(KeyboardKey::KEY_UP)
        {
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

        for wall in determine_player_wall_vis(&self.player, &self.level) {
            match wall {
                RelativeWall::Front => {
                    self.middle_wall_sprite
                        .draw(&mut scene_ctx.glyph_buffer, 0, 0);
                }
                RelativeWall::LeftSide => {
                    self.left_wall_sprite
                        .draw(&mut scene_ctx.glyph_buffer, 0, 0);
                }
                RelativeWall::RightSide => {
                    self.right_wall_sprite
                        .draw(&mut scene_ctx.glyph_buffer, 0, 0);
                }
            }
        }
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

fn determine_player_wall_vis(player: &Player, level: &Level) -> Vec<RelativeWall> {
    let px = player.tile_pos_x;
    let py = player.tile_pos_y;
    let mut visible = Vec::new();

    let height = level.level_grid.len() as i32;
    let width = level.level_grid[0].len() as i32;

    let (front_dx, front_dy, left_dx, left_dy, right_dx, right_dy) = match player.facing {
        Direction::Up => (0, -1, -1, 0, 1, 0),
        Direction::Down => (0, 1, 1, 0, -1, 0),
        Direction::Left => (-1, 0, 0, 1, 0, -1),
        Direction::Right => (1, 0, 0, -1, 0, 1),
    };

    let fx = px + front_dx;
    let fy = py + front_dy;
    let lx = px + left_dx;
    let ly = py + left_dy;
    let rx = px + right_dx;
    let ry = py + right_dy;

    let is_wall = |x: i32, y: i32| -> bool {
        x >= 0 && x < width && y >= 0 && y < height
            && level.level_grid[y as usize][x as usize] == TileType::Wall
    };

    let has_front = is_wall(fx, fy);
    let has_left = is_wall(lx, ly);
    let has_right = is_wall(rx, ry);

    eprintln!(
        "Facing: {:?}, Front: {}, Left: {}, Right: {}",
        player.facing, has_front, has_left, has_right
    );

    if has_front {
        visible.push(RelativeWall::Front);
    }
    if has_left {
        visible.push(RelativeWall::LeftSide);
    }
    if has_right {
        visible.push(RelativeWall::RightSide);
    }

    visible
}
