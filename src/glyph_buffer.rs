use crate::char_set::{CharSet, Glyph};
use raylib::prelude::*;

#[derive(Clone)]
struct Cell {
    glyph: Option<u8>,
    fg: Color,
    bg: Color,
}

/// A 2D character grid.  Call `clear` at the top of the frame, then
/// `put_glyph` / `put_text` to populate, then `render` once to draw.
pub struct GlyphBuffer {
    char_set: CharSet,
    pub glyph_w: i32,
    pub glyph_h: i32,
    pub cols: i32,
    pub rows: i32,
    offset_x: i32,
    offset_y: i32,
    grid: Vec<Cell>,
}

impl GlyphBuffer {
    pub fn new(char_set: CharSet, glyph_w: i32, glyph_h: i32, cols: i32, rows: i32) -> Self {
        let cell = Cell {
            glyph: None,
            fg: Color::WHITE,
            bg: Color::BLACK,
        };
        GlyphBuffer {
            char_set,
            glyph_w,
            glyph_h,
            cols,
            rows,
            offset_x: 0,
            offset_y: 0,
            grid: vec![cell; (cols * rows) as usize],
        }
    }

    /// Reset every cell — fill backgrounds, clear glyphs.
    pub fn clear(&mut self, bg: Color) {
        for cell in &mut self.grid {
            cell.glyph = None;
            cell.fg = Color::WHITE;
            cell.bg = bg;
        }
    }

    /// Set a single cell's background color without touching the glyph.
    pub fn put_bg(&mut self, col: i32, row: i32, bg: Color) {
        if let Some(cell) = self.cell_mut(col, row) {
            cell.bg = bg;
        }
    }

    /// Recalculate glyph dimensions so the fixed grid fills the screen
    /// while keeping cells square.  Excess space is split into offsets
    /// that centre the grid.
    pub fn resize(&mut self, screen_w: i32, screen_h: i32) {
        let cell = (screen_w / self.cols).min(screen_h / self.rows);
        self.glyph_w = cell;
        self.glyph_h = cell;
        self.offset_x = (screen_w - cell * self.cols) / 2;
        self.offset_y = (screen_h - cell * self.rows) / 2;
    }

    /// Convert pixel coordinates to grid (col, row), accounting for
    /// the centring offset used when aspect-ratio clamping is active.
    pub fn pixel_to_grid(&self, px: i32, py: i32) -> (i32, i32) {
        (
            (px - self.offset_x) / self.glyph_w,
            (py - self.offset_y) / self.glyph_h,
        )
    }

    /// Draw a CP437 single-line box into the grid.
    /// `width` and `height` are in glyphs (min 2).
    pub fn put_box(&mut self, col: i32, row: i32, width: i32, height: i32, fg: Color, bg: Color) {
        if width < 2 || height < 2 {
            return;
        }
        // Corners
        self.put_glyph(col, row, Glyph::ULCorner, fg, bg);
        self.put_glyph(col + width - 1, row, Glyph::URCorner, fg, bg);
        self.put_glyph(col, row + height - 1, Glyph::LLCorner, fg, bg);
        self.put_glyph(col + width - 1, row + height - 1, Glyph::LRCorner, fg, bg);
        // Top / bottom edges
        for c in 1..width - 1 {
            self.put_glyph(col + c, row, Glyph::HLine, fg, bg);
            self.put_glyph(col + c, row + height - 1, Glyph::HLine, fg, bg);
        }
        // Left / right edges
        for r in 1..height - 1 {
            self.put_glyph(col, row + r, Glyph::VLine, fg, bg);
            self.put_glyph(col + width - 1, row + r, Glyph::VLine, fg, bg);
        }
        // Interior fill
        self.fill_bg(col + 1, row + 1, width - 2, height - 2, bg);
    }

    /// Like `put_text` but wraps at a custom `max_col` instead of the screen edge.
    pub fn put_text_bounded(
        &mut self,
        text: &str,
        col: i32,
        row: i32,
        max_col: i32,
        fg: Color,
        bg: Color,
    ) {
        let mut current_row = row;
        for line in text.lines() {
            if current_row >= self.rows {
                break;
            }
            self.put_text_line_bounded(line, col, current_row, max_col, fg, bg);
            current_row += 1;
        }
    }

    fn put_text_line_bounded(
        &mut self,
        text: &str,
        col: i32,
        row: i32,
        max_col: i32,
        fg: Color,
        bg: Color,
    ) {
        if text.is_empty() {
            return;
        }
        match first_break_bounded(text, col, max_col) {
            None => {
                for (i, c) in text.chars().enumerate() {
                    self.put_char(col + i as i32, row, c, fg, bg);
                }
            }
            Some(split_at) => {
                let current = &text[..split_at];
                if !current.is_empty() {
                    for (i, c) in current.chars().enumerate() {
                        self.put_char(col + i as i32, row, c, fg, bg);
                    }
                }
                self.put_text_line_bounded(&text[split_at..], col, row + 1, max_col, fg, bg);
            }
        }
    }

    /// Fill a rectangular region of cells with a background color.
    pub fn fill_bg(&mut self, col: i32, row: i32, width: i32, height: i32, bg: Color) {
        for r in 0..height {
            for c in 0..width {
                self.put_bg(col + c, row + r, bg);
            }
        }
    }

    /// Write a glyph at grid position (col, row).  Clamped to bounds.
    pub fn put_glyph(&mut self, col: i32, row: i32, glyph: Glyph, fg: Color, bg: Color) {
        if let Some(cell) = self.cell_mut(col, row) {
            cell.glyph = Some(glyph as u8);
            cell.fg = fg;
            cell.bg = bg;
        }
    }

    /// Write a printable-ASCII character at grid position.
    fn put_char(&mut self, col: i32, row: i32, c: char, fg: Color, bg: Color) {
        let code = c as u8;
        if (0x20..=0x7E).contains(&code) {
            if let Some(cell) = self.cell_mut(col, row) {
                cell.glyph = Some(code);
                cell.fg = fg;
                cell.bg = bg;
            }
        }
    }

    /// Write text starting at a grid position.  Wraps on word boundaries;
    /// clips at the screen bottom.
    pub fn put_text(
        &mut self,
        text: &str,
        starting_col: i32,
        starting_row: i32,
        fg: Color,
        bg: Color,
    ) {
        let mut row = starting_row;
        for line in text.lines() {
            if row >= self.rows {
                break;
            }
            self.put_text_line(line, starting_col, row, fg, bg);
            row += 1;
        }
    }

    fn put_text_line(&mut self, text: &str, col: i32, row: i32, fg: Color, bg: Color) {
        if text.is_empty() {
            return;
        }
        match self.first_break(text, col) {
            None => {
                for (i, c) in text.chars().enumerate() {
                    self.put_char(col + i as i32, row, c, fg, bg);
                }
            }
            Some(split_at) => {
                let current = &text[..split_at];
                if !current.is_empty() {
                    for (i, c) in current.chars().enumerate() {
                        self.put_char(col + i as i32, row, c, fg, bg);
                    }
                }
                self.put_text_line(&text[split_at..], col, row + 1, fg, bg);
            }
        }
    }

    fn first_break(&self, text: &str, starting_col: i32) -> Option<usize> {
        let mut width = 0;
        for (i, _c) in text.chars().enumerate() {
            width += 1;
            if starting_col + width > self.cols {
                if let Some(space) = text[..i].rfind(' ') {
                    return Some(space + 1);
                }
                return Some(if i > 0 { i } else { 1 });
            }
        }
        None
    }

    /// Draw the entire grid to the screen in a single pass.
    /// Glyph textures are scaled to fill the current cell dimensions.
    pub fn render(&self, d: &mut RaylibDrawHandle) {
        for (i, cell) in self.grid.iter().enumerate() {
            let col = (i as i32) % self.cols;
            let row = (i as i32) / self.cols;
            let x = (self.offset_x + col * self.glyph_w) as f32;
            let y = (self.offset_y + row * self.glyph_h) as f32;
            let w = self.glyph_w as f32;
            let h = self.glyph_h as f32;

            d.draw_rectangle_v(Vector2 { x, y }, Vector2 { x: w, y: h }, cell.bg);

            if let Some(code) = cell.glyph {
                d.draw_texture_pro(
                    self.char_set.get(code),
                    Rectangle {
                        x: 0.0,
                        y: 0.0,
                        width: 16.0,
                        height: 16.0,
                    },
                    Rectangle {
                        x,
                        y,
                        width: w,
                        height: h,
                    },
                    Vector2 { x: 0.0, y: 0.0 },
                    0.0,
                    cell.fg,
                );
            }
        }
    }

    fn cell_mut(&mut self, col: i32, row: i32) -> Option<&mut Cell> {
        if col >= 0 && col < self.cols && row >= 0 && row < self.rows {
            Some(&mut self.grid[(row * self.cols + col) as usize])
        } else {
            None
        }
    }
}

fn first_break_bounded(text: &str, starting_col: i32, max_col: i32) -> Option<usize> {
    let mut width = 0;
    for (i, _c) in text.chars().enumerate() {
        width += 1;
        if starting_col + width > max_col {
            if let Some(space) = text[..i].rfind(' ') {
                return Some(space + 1);
            }
            return Some(if i > 0 { i } else { 1 });
        }
    }
    None
}
