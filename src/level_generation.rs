use crate::char_set::Glyph;

pub struct Level {
    width: usize,
    height: usize,
    pub tiles: Vec<Tile>,
}

#[derive(Clone)]
pub struct Tile {
    pub tile_type: TileType,
    pub walkable: bool,
}

#[derive(Clone, Copy)]
pub enum TileType {
    Floor,
    Wall,
    Door,
    Chest,
}

pub enum Generator {
    Random,
    CellularAutomata,
    BSP,
}

impl Clone for Level {
    fn clone(&self) -> Self {
        Level {
            width: self.width,
            height: self.height,
            tiles: self.tiles.clone(),
        }
    }
}

impl Level {
    pub fn new(width: usize, height: usize) -> Self {
        let tiles: Vec<Tile> = vec![
            Tile {
                tile_type: TileType::Floor,
                walkable: true,
            };
            width * height
        ];
        Level {
            width,
            height,
            tiles,
        }
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile_type: TileType) {
        if x < self.width && y < self.height {
            let walkable = match tile_type {
                TileType::Floor => true,
                TileType::Wall => false,
                TileType::Door => true,
                TileType::Chest => false,
            };
            self.tiles[y * self.width + x] = Tile {
                tile_type,
                walkable,
            };
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<&Tile> {
        if x < self.width && y < self.height {
            Some(&self.tiles[y * self.width + x])
        } else {
            None
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

fn count_neighboring_walls(level: &Level, x: usize, y: usize) -> usize {
    let mut count = 0;
    for dy in -1i32..=1 {
        for dx in -1i32..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx < 0 || ny < 0 {
                count += 1; // Out of bounds counts as wall
            } else {
                let nx = nx as usize;
                let ny = ny as usize;
                if nx >= level.width || ny >= level.height {
                    count += 1; // Out of bounds counts as wall
                } else if let Some(tile) = level.get_tile(nx, ny) {
                    if matches!(tile.tile_type, TileType::Wall) {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

pub fn generate_level(level: &mut Level, generator: Generator, iterations: usize, wall_chance: f64) -> Result<(), String> {
    match generator {
        Generator::Random => Ok(()),
        Generator::CellularAutomata => {
            cellular_automata_generation(level, iterations, wall_chance)?;
            Ok(())
        }
        Generator::BSP => Ok(()),
    }
}

pub fn cellular_automata_generation(level: &mut Level, iterations: usize, wall_chance: f64) -> Result<(), String> {
    use rand::Rng;

    let mut rng = rand::rng();

    // Step 1: Initialize the level with random walls and floors.
    for y in 0..level.height {
        for x in 0..level.width {
            let tile_type = if rng.random::<f64>() < wall_chance {
                TileType::Wall
            } else {
                TileType::Floor
            };
            level.set_tile(x, y, tile_type);
        }
    }

    // Step 2: Apply cellular automata rules for a number of iterations.
    for _ in 0..iterations {
        let mut new_tiles: Vec<Tile> = level.tiles.clone();
        for y in 0..level.height {
            for x in 0..level.width {
                let wall_count = count_neighboring_walls(level, x, y);
                let new_tile_type = match level.get_tile(x, y).unwrap().tile_type {
                    TileType::Wall => {
                        if wall_count >= 4 { TileType::Wall } else { TileType::Floor }
                    }
                    TileType::Floor => {
                        if wall_count >= 5 { TileType::Wall } else { TileType::Floor }
                    }
                    other => other,
                };
                new_tiles[y * level.width + x] = Tile {
                    tile_type: new_tile_type,
                    walkable: matches!(new_tile_type, TileType::Floor | TileType::Door),
                };
            }
        }
        level.tiles = new_tiles;
    }

    Ok(())
}
