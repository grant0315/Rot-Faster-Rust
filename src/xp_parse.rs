use crate::glyph_buffer::{Cell, GlyphBuffer};
use flate2::read::GzDecoder;
use raylib::prelude::Color;
use std::fs::File;
use std::io::Read;

pub struct Sprite {
    pub cols: i32,
    pub rows: i32,
    pub grid: Vec<Cell>,
}

impl Sprite {
    pub fn draw(&self, buffer: &mut GlyphBuffer, col: i32, row: i32) {
        for sy in 0..self.rows {
            for sx in 0..self.cols {
                let dst_col = col + sx;
                let dst_row = row + sy;
                if dst_col >= 0 && dst_col < buffer.cols && dst_row >= 0 && dst_row < buffer.rows {
                    let dst_idx = (dst_row * buffer.cols + dst_col) as usize;
                    let src_idx = (sy * self.cols + sx) as usize;
                    let src_cell = &self.grid[src_idx];

                    let is_hotpink = src_cell.bg.r == 255
                        && src_cell.bg.g == 0
                        && src_cell.bg.b == 255;
                    let is_all_black = src_cell.glyph == Some(0)
                        && src_cell.fg.r == 0
                        && src_cell.fg.g == 0
                        && src_cell.fg.b == 0
                        && src_cell.bg.r == 0
                        && src_cell.bg.g == 0
                        && src_cell.bg.b == 0;

                    if !is_hotpink && !is_all_black {
                        buffer.grid[dst_idx] = src_cell.clone();
                    }
                }
            }
        }
    }

    pub fn select(&self, x: i32, y: i32, w: i32, h: i32) -> Sprite {
        let mut grid = Vec::with_capacity((w * h) as usize);

        for row in y..y + h {
            for col in x..x + w {
                let idx = (row * self.cols + col) as usize;
                grid.push(self.grid[idx].clone());
            }
        }

        Sprite {
            cols: w,
            rows: h,
            grid,
        }
    }

    pub fn split(&self, tile_w: i32, tile_h: i32) -> Vec<Sprite> {
        if self.cols % tile_w != 0 || self.rows % tile_h != 0 {
            panic!(
                "Sprite dimensions ({}x{}) must be divisible by tile size ({}x{})",
                self.cols, self.rows, tile_w, tile_h
            );
        }

        let cols = self.cols / tile_w;
        let rows = self.rows / tile_h;
        let mut tiles = Vec::with_capacity((cols * rows) as usize);

        for ty in 0..rows {
            for tx in 0..cols {
                let mut grid = Vec::with_capacity((tile_w * tile_h) as usize);

                for row in 0..tile_h {
                    for col in 0..tile_w {
                        let src_col = tx * tile_w + col;
                        let src_row = ty * tile_h + row;
                        let src_idx = (src_row * self.cols + src_col) as usize;
                        grid.push(self.grid[src_idx].clone());
                    }
                }

                tiles.push(Sprite {
                    cols: tile_w,
                    rows: tile_h,
                    grid,
                });
            }
        }

        tiles
    }
}

pub struct XpFile {
    pub filepath: String,
}

impl XpFile {
    pub fn new(filepath: String) -> XpFile {
        XpFile { filepath }
    }

    pub fn parse(&self) -> Sprite {
        let file = File::open(&self.filepath).expect("Failed to open .xp file");
        let mut decoder = GzDecoder::new(file);
        let mut data = Vec::new();
        decoder
            .read_to_end(&mut data)
            .expect("Failed to decompress .xp file");

        let mut offset = 0;

        let version = i32::from_le_bytes(data[offset..offset + 4].try_into().unwrap());
        offset += 4;

        let num_layers = i32::from_le_bytes(data[offset..offset + 4].try_into().unwrap());
        offset += 4;

        if version != -1 {
            eprintln!(
                "Warning: REXPaint version {} (expected -1), parsing may be inaccurate",
                version
            );
        }

        let mut width = 0i32;
        let mut height = 0i32;
        let mut merged_grid: Vec<Cell> = Vec::new();

        for layer_idx in 0..num_layers {
            let lw = i32::from_le_bytes(data[offset..offset + 4].try_into().unwrap());
            offset += 4;

            let lh = i32::from_le_bytes(data[offset..offset + 4].try_into().unwrap());
            offset += 4;

            if layer_idx == 0 {
                width = lw;
                height = lh;
                let transparent_cell = Cell {
                    glyph: Some(0),
                    fg: Color::BLACK,
                    bg: Color {
                        r: 255,
                        g: 0,
                        b: 255,
                        a: 255,
                    },
                };
                merged_grid.resize((lw * lh) as usize, transparent_cell);
            }

            for _col in 0..lw {
                for _row in 0..lh {
                    let ch = u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap()) as u8;
                    offset += 4;

                    let fg_r = data[offset];
                    offset += 1;
                    let fg_g = data[offset];
                    offset += 1;
                    let fg_b = data[offset];
                    offset += 1;

                    let bg_r = data[offset];
                    offset += 1;
                    let bg_g = data[offset];
                    offset += 1;
                    let bg_b = data[offset];
                    offset += 1;

                    let is_hotpink = bg_r == 255 && bg_g == 0 && bg_b == 255;
                    let is_allblack = ch == 0
                        && fg_r == 0
                        && fg_g == 0
                        && fg_b == 0
                        && bg_r == 0
                        && bg_g == 0
                        && bg_b == 0;
                    let is_transparent = is_hotpink || is_allblack;
                    if !is_transparent {
                        // Column-major (file) → row-major (grid):
                        //   file index = col * height + row
                        //   grid index = row * width + col
                        let grid_idx = (_row * lw + _col) as usize;
                        merged_grid[grid_idx] = Cell {
                            glyph: Some(ch),
                            fg: Color {
                                r: fg_r,
                                g: fg_g,
                                b: fg_b,
                                a: 255,
                            },
                            bg: Color {
                                r: bg_r,
                                g: bg_g,
                                b: bg_b,
                                a: 255,
                            },
                        };
                    }
                }
            }
        }

        Sprite {
            cols: width,
            rows: height,
            grid: merged_grid,
        }
    }
}
