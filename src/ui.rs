use crate::glyph_buffer::GlyphBuffer;
use raylib::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum ButtonState {
    Normal,
    Hovered,
    Pressed,
}

#[derive(Clone, Copy, Default)]
pub(crate) struct Rect {
    col: i32,
    row: i32,
    width: i32,
    height: i32,
}

impl Rect {
    fn contains(&self, col: i32, row: i32) -> bool {
        col >= self.col
            && col < self.col + self.width
            && row >= self.row
            && row < self.row + self.height
    }
}

#[derive(Clone)]
pub enum UIElement {
    Text {
        text: String,
        border: bool,
        rect: Rect,
    },
    Button {
        id: String,
        label: String,
        state: ButtonState,
        rect: Rect,
    },
    VStack {
        children: Vec<UIElement>,
        gap: i32,
        border: bool,
        rect: Rect,
    },
}

// ---------------------------------------------------------------------------
// Construction helpers
// ---------------------------------------------------------------------------

impl UIElement {
    pub fn text(text: impl Into<String>) -> Self {
        UIElement::Text { text: text.into(), border: false, rect: Rect::default() }
    }

    pub fn text_bordered(text: impl Into<String>) -> Self {
        UIElement::Text { text: text.into(), border: true, rect: Rect::default() }
    }

    pub fn button(id: impl Into<String>, label: impl Into<String>) -> Self {
        UIElement::Button {
            id: id.into(),
            label: label.into(),
            state: ButtonState::Normal,
            rect: Rect::default(),
        }
    }

    pub fn vstack(children: Vec<UIElement>, gap: i32) -> Self {
        UIElement::VStack { children, gap, border: false, rect: Rect::default() }
    }

    pub fn vstack_bordered(children: Vec<UIElement>, gap: i32) -> Self {
        UIElement::VStack { children, gap, border: true, rect: Rect::default() }
    }
}

// ---------------------------------------------------------------------------
// Measurement — bottom-up, returns (width, height) in glyphs
// ---------------------------------------------------------------------------

fn measure_text(text: &str) -> (i32, i32) {
    let mut max_chars = 0i32;
    let mut line_count = 0i32;
    for line in text.lines() {
        let chars = line.chars().count() as i32;
        if chars > max_chars {
            max_chars = chars;
        }
        line_count += 1;
    }
    (max_chars, line_count)
}

fn measure_element(element: &UIElement) -> (i32, i32) {
    match element {
        UIElement::Text { text, border, .. } => {
            let (tw, th) = measure_text(text);
            if *border { (tw + 2, th + 2) } else { (tw, th) }
        }
        UIElement::Button { label, .. } => {
            let (tw, th) = measure_text(label);
            (tw + 2, th + 2)
        }
        UIElement::VStack { children, gap, border, .. } => {
            let mut max_w = 0i32;
            let mut total_h = 0i32;
            for child in children {
                let (cw, ch) = measure_element(child);
                if cw > max_w { max_w = cw; }
                total_h += ch;
            }
            let n = children.len() as i32;
            if n > 1 {
                total_h += gap * (n - 1);
            }
            if *border { (max_w + 2, total_h + 2) } else { (max_w, total_h) }
        }
    }
}

// ---------------------------------------------------------------------------
// Draw — populates the GlyphBuffer grid, returns size used
// ---------------------------------------------------------------------------

pub fn draw(
    gb: &mut GlyphBuffer,
    element: &mut UIElement,
    col: i32,
    row: i32,
    fg: Color,
    bg: Color,
) -> (i32, i32) {
    match element {
        UIElement::Text { text, border, rect } => {
            let (tw, th) = measure_text(text);
            if *border {
                let w = tw + 2;
                let h = th + 2;
                *rect = Rect { col, row, width: w, height: h };
                gb.put_box(col, row, w, h, fg, bg);
                gb.put_text_bounded(text, col + 1, row + 1, col + w - 1, fg, bg);
                (w, h)
            } else {
                *rect = Rect { col, row, width: tw, height: th };
                gb.put_text(text, col, row, fg, bg);
                (tw, th)
            }
        }

        UIElement::Button { id: _, label, state, rect } => {
            let (tw, th) = measure_text(label);
            let w = tw + 2;
            let h = th + 2;
            *rect = Rect { col, row, width: w, height: h };

            let (box_fg, box_bg) = match *state {
                ButtonState::Normal => (fg, bg),
                ButtonState::Hovered => (fg, Color::new(60, 60, 60, 255)),
                ButtonState::Pressed => (bg, fg),
            };

            gb.put_box(col, row, w, h, box_fg, box_bg);
            gb.put_text_bounded(label, col + 1, row + 1, col + w - 1, box_fg, box_bg);
            (w, h)
        }

        UIElement::VStack { children, gap, border, rect } => {
            let mut max_w = 0i32;
            let mut total_h = 0i32;
            for child in children.iter() {
                let (cw, ch) = measure_element(child);
                if cw > max_w { max_w = cw; }
                total_h += ch;
            }
            let n = children.len() as i32;
            if n > 1 { total_h += *gap * (n - 1); }
            let mw = if *border { max_w + 2 } else { max_w };
            let mh = if *border { total_h + 2 } else { total_h };
            *rect = Rect { col, row, width: mw, height: mh };

            let inner_col = col + if *border { 1 } else { 0 };
            let mut cy = row + if *border { 1 } else { 0 };

            if *border {
                gb.put_box(col, row, mw, mh, fg, bg);
            }

            for child in children.iter_mut() {
                let (_, child_h) = draw(gb, child, inner_col, cy, fg, bg);
                cy += child_h + *gap;
            }
            (mw, mh)
        }
    }
}

// ---------------------------------------------------------------------------
// Update — walks the tree updating button states, returns clicked button id
// ---------------------------------------------------------------------------

pub fn update(
    element: &mut UIElement,
    mouse_col: i32,
    mouse_row: i32,
    mouse_down: bool,
) -> Option<String> {
    match element {
        UIElement::Text { .. } => None,

        UIElement::Button { id, state, rect, .. } => {
            let inside = rect.contains(mouse_col, mouse_row);
            match *state {
                ButtonState::Pressed => {
                    if inside && !mouse_down {
                        *state = ButtonState::Normal;
                        return Some(id.clone());
                    }
                    if !inside {
                        *state = ButtonState::Normal;
                    }
                }
                _ => {
                    if inside {
                        *state = if mouse_down {
                            ButtonState::Pressed
                        } else {
                            ButtonState::Hovered
                        };
                    } else {
                        *state = ButtonState::Normal;
                    }
                }
            }
            None
        }

        UIElement::VStack { children, .. } => {
            for child in children.iter_mut() {
                if let Some(id) = update(child, mouse_col, mouse_row, mouse_down) {
                    return Some(id);
                }
            }
            None
        }
    }
}
