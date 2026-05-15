use raylib::prelude::*;
use raylib::core::texture::Texture2D;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Glyph {
    // --- ASCII printable ---
    Space = b' ',
    Player = b'@',
    Wall = b'#',
    Floor = b'.',
    StairsDown = b'>',
    StairsUp = b'<',
    Door = b'+',

    // --- CP437 box drawing ---
    VLine = 0xB3,
    HLine = 0xC4,
    Cross = 0xC5,
    TDown = 0xC2,
    TUp = 0xC1,
    TRight = 0xC3,
    TLeft = 0xB4,
    ULCorner = 0xDA,
    URCorner = 0xBF,
    LLCorner = 0xC0,
    LRCorner = 0xD9,

    // --- CP437 block elements ---
    FullBlock = 0xDB,
    LowerHalfBlock = 0xDC,
    LeftHalfBlock = 0xDD,
    RightHalfBlock = 0xDE,
    UpperHalfBlock = 0xDF,

    // --- CP437 Shades ---
    LightShade = 0xB0,
    MediumShade = 0xB1,
    DarkShade = 0xB2,

    // --- Arrows ---
    UpArrow = 0x18,
    DownArrow = 0x19,
    RightArrow = 0x1A,
    LeftArrow = 0x1B,

    // --- BLANK ---
    Blank = 0xFF,
}

pub struct CharSet {
    textures: Vec<Texture2D>,
    pub glyph_width: i32,
    pub glyph_height: i32,
}

impl CharSet {
    pub fn new(
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        path: &str,
        glyph_width: i32,
        glyph_height: i32,
        cols: i32,
        rows: i32,
    ) -> Self {
        let mut full_image = Image::load_image(path).unwrap();

        // Ensure RGBA so alpha_clear has an alpha channel to work with
        full_image.set_format(raylib::consts::PixelFormat::PIXELFORMAT_UNCOMPRESSED_R8G8B8A8);
        // Make the black background transparent so draw_texture tint works
        full_image.alpha_clear(Color::BLACK, 60.0);

        let mut textures = Vec::with_capacity((cols * rows) as usize);

        for row in 0..rows {
            for col in 0..cols {
                let x = col * glyph_width;
                let y = row * glyph_height;
                let glyph_image = full_image.from_image(Rectangle {
                    x: x as f32,
                    y: y as f32,
                    width: glyph_width as f32,
                    height: glyph_height as f32,
                });
                let texture = rl.load_texture_from_image(thread, &glyph_image).unwrap();
                textures.push(texture);
            }
        }

        CharSet {
            textures,
            glyph_width,
            glyph_height,
        }
    }

    pub fn get(&self, code: u8) -> &Texture2D {
        &self.textures[code as usize]
    }

    pub fn glyph(&self, glyph: Glyph) -> &Texture2D {
        self.get(glyph as u8)
    }

    // For string rendering: best-effort ASCII subset.
    // Only handles printable ASCII
    pub fn get_char(&self, c: char) -> Option<&Texture2D> {
        let code = c as u8;
        if (0x20..=0x7E).contains(&code) {
            Some(&self.textures[code as usize])
        } else {
            None
        }
    }
}
