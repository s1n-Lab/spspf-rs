/// List of some preset colors, to be used when `Color` is requested, you must utilize it the following way:
/// `Colors::RED.as_color()`.
pub enum Colors {
    /// R: 0, G: 0, B: 0, A: 255
    BLACK,
    /// R: 255, G: 0, B: 0, A: 255
    RED,
    /// R: 0, G: 255, B: 0, A: 255
    GREEN,
    /// R: 0, G: 0, B: 255, A: 255
    BLUE,
    /// R: 255, G: 255, B: 0, A: 255
    YELLOW,
    /// R: 255, G: 0, B: 255, A: 255
    MAGENTA,
    /// R: 0, G: 255, B: 255, A: 255
    CYAN,
    /// R: 255, G: 255, B: 255, A: 255
    WHITE,
}

impl Colors {
    pub fn as_color(&self) -> Color {
        match self {
            Colors::BLACK => Color::new(0, 0, 0, 255),            
            Colors::RED => Color::new(255, 0, 0, 255),            
            Colors::GREEN => Color::new(0, 255, 0, 255),
            Colors::BLUE => Color::new(0, 0, 255, 255),
            Colors::YELLOW => Color::new(255, 255, 0, 255),
            Colors::MAGENTA => Color::new(255, 0, 255, 255),
            Colors::CYAN => Color::new(0, 255, 255, 255),
            Colors::WHITE => Color::new(255, 255, 255, 255),
        }
    }
}

/// Defines what a color is in RGBA8.
#[derive(Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    /// Returns a color based on provided RGBA.
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color {
            r: r,
            g: g,
            b: b,
            a: a,
        }
    }

    pub(crate) fn as_abgr(&self) -> u32 {
        (self.a as u32) << 24 | (self.b as u32) << 16 | (self.g as u32) << 8 | (self.r as u32)
    }
}
