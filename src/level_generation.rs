use crate::glyph_buffer::GlyphBuffer;

// A level is a 2D grid of glyphs, along with the player's spawn point.
pub struct Level {
    pub glyph_buffer: GlyphBuffer,
    pub gbuf_width: usize,
    pub gbuf_height: usize,
    pub player_spawn: (usize, usize),
    pub level_grid: Vec<Vec<TileType>>,
}

// Enum placeholder for tile types to transfer from level generation to glyph buffer population.
#[derive(Clone, Copy)]
enum TileType {
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
    pub fn new(glyph_buffer: GlyphBuffer, gbuf_width: usize, gbuf_height: usize, player_spawn: (usize, usize)) -> Self {
        Self {
            glyph_buffer,
            gbuf_width,
            gbuf_height,
            player_spawn,
            level_grid: vec![vec![TileType::Wall; gbuf_width]; gbuf_height], // PLaceholder grid initialization; User will replace with actual generated grid via the generate() method. 
        }
    }

    pub fn generate(self: &mut Self, level_gen_algorithm: LevelGenAlgorithm, gbuf_width: usize, gbuf_height: usize) {
        // Generate a tile grid using the specified algorithm.
        self.level_grid = match level_gen_algorithm {
            LevelGenAlgorithm::CellularAutomataCave => cave_level_generation(gbuf_width, gbuf_height),
            // Other algorithms can be added here.
        };
    }
}

// Placeholder for a cellular automata for cave level generation algorithm.
fn cave_level_generation(width: usize, height: usize) -> Vec<Vec<TileType>> {
    let mut grid = vec![vec![TileType::Wall; width]; height];

    return grid;
}
