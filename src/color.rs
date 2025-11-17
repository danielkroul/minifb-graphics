#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color(pub u32);

impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        let val = (r as u32) << 16 | (g as u32) << 8 | (b as u32);
        Color(val)
    }

    pub const BLACK: Color = Color(0x000000);
    pub const WHITE: Color = Color(0xFFFFFF);
    pub const RED: Color = Color(0xFF0000);
    pub const GREEN: Color = Color(0x00FF00);
    pub const BLUE: Color = Color(0x0000FF);
    pub const CYAN: Color = Color(0x00FFFF);
    pub const MAGENTA: Color = Color(0xFF00FF);
    pub const YELLOW: Color = Color(0xFFFF00);
}

impl From<Color> for u32 {
    fn from(color: Color) -> Self {
        color.0
    }
}