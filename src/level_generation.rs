use crate::glyph_buffer::GlyphBuffer;
use crate::char_set::Glyph;
use raylib::prelude::Color;

// A level is a 2D grid of tiles, along with the player's spawn point.
pub struct Level {
    pub gbuf_width: usize,
    pub gbuf_height: usize,
    pub player_spawn: (usize, usize),
    pub level_grid: Vec<Vec<TileType>>,
}

// Enum placeholder for tile types to transfer from level generation to glyph buffer population.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Floor,
    Wall,
    Door,
    PlayerSpawn,
    StairsDown,
    StairsUp,
}

pub enum LevelGenAlgorithm {
    CellularAutomataCave,
    // Other algorithms can be added here.
}

impl Level {
    pub fn new(gbuf_width: usize, gbuf_height: usize, player_spawn: (usize, usize)) -> Self {
        Self {
            gbuf_width,
            gbuf_height,
            player_spawn,
            level_grid: vec![vec![TileType::Wall; gbuf_width]; gbuf_height],
        }
    }

    pub fn generate(&mut self, level_gen_algorithm: LevelGenAlgorithm, gbuf_width: usize, gbuf_height: usize) {
        self.level_grid = match level_gen_algorithm {
            LevelGenAlgorithm::CellularAutomataCave => cave_level_generation(gbuf_width, gbuf_height),
        };
    }

    pub fn render_to_buffer(&self, buffer: &mut GlyphBuffer) {
        for row in 0..self.gbuf_height {
            for col in 0..self.gbuf_width {
                let (glyph, fg, bg) = match self.level_grid[row][col] {
                    TileType::Wall => (Glyph::DarkShade, Color::LIGHTGRAY, Color::BLACK),
                    TileType::Floor => (Glyph::Space, Color::WHITE, Color::BLACK),
                    TileType::Door => (Glyph::Door, Color::BROWN, Color::BLACK),
                    TileType::PlayerSpawn => (Glyph::Floor, Color::WHITE, Color::BLACK),
                    TileType::StairsDown => (Glyph::StairsDown, Color::YELLOW, Color::BLACK),
                    TileType::StairsUp => (Glyph::StairsUp, Color::YELLOW, Color::BLACK),
                };
                buffer.put_glyph(col as i32, row as i32, glyph, fg, bg);
            }
        }
    }
}

// Stadnard 4-5 step cellular automata cave generation algorithm.
// 1. Start with a grid of random walls and floors. (about 45% walls is a common starting point)
// 2. For a set number of iterations (commonly 4-5), apply the following rules to each cell:
//    - If a wall cell has 4 or more neighboring wall, it remains a wall; otherwise, it becomes a
//    floor
//    - If a floor cell has 5 or more neighboring wall, it becomes a wall; otherwise, it remains a floor
// 3. Optionally, add features like doors, stairs, or player spawn points after the main cave structure is generated.
fn cave_level_generation(width: usize, height: usize) -> Vec<Vec<TileType>> {
    let mut grid = vec![vec![TileType::Wall; width]; height];
    
    for row in 0..height {
        for col in 0..width {
            // Randomly assign walls and floors with about 45% walls.
            grid[row][col] = if rand::random::<f32>() < 0.45 {
                TileType::Wall
            } else {
                TileType::Floor
            };
        }
    }

    // Apply cellular automata rules for a set number of iterations (e.g., 4-5).
    for _ in 0..5 {
        let mut new_grid = grid.clone();
        
        for row in 0..height {
            for col in 0..width {
                let wall_neighbors = count_wall_neighbors(&grid, row, col);
                
                new_grid[row][col] = match grid[row][col] {
                    TileType::Wall => if wall_neighbors >= 4 { TileType::Wall } else { TileType::Floor },
                    TileType::Floor => if wall_neighbors >= 5 { TileType::Wall } else { TileType::Floor },
                    other => other, // Keep other tile types unchanged.
                };
            }
        }
        
        grid = new_grid;
    }

    return grid;
}

fn count_wall_neighbors(grid: &Vec<Vec<TileType>>, row: usize, col: usize) -> usize {
    let mut count = 0;
    let directions = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
    
    for (dr, dc) in directions.iter() {
        let new_row = row as isize + dr;
        let new_col = col as isize + dc;
        
        if new_row >= 0 && new_row < grid.len() as isize && new_col >= 0 && new_col < grid[0].len() as isize {
            if grid[new_row as usize][new_col as usize] == TileType::Wall {
                count += 1;
            }
        } else {
            // Treat out-of-bounds as walls to create solid borders.
            count += 1;
        }
    }
    
    count
}
