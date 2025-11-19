
/// Stores a u32 color value in format 0xAARRGGBB
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color(pub u32);

impl Color {
    /// Returns a Color(u32) value in format 0xAARRGGBB
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        let val = (a as u32) << 24 | (r as u32) << 16 | (g as u32) << 8 | (b as u32);
        Color(val)
    }

    pub fn alpha(&self) -> u8 { (self.0 >> 24) as u8 }

    pub fn red(&self) -> u8 { ((self.0 >> 16) & 0xFF) as u8 }

    pub fn green(&self) -> u8 { ((self.0 >> 8) & 0xFF) as u8 }

    pub fn blue(&self) -> u8 { (self.0 & 0xFF) as u8 }

    pub fn mix(bottom: Color, top: Color) -> Color {
        let ta = top.alpha() as u32;

        if ta == 0 { return bottom }
        if ta == 255 { return top }

        let tr = top.red() as u32;
        let tg = top.green() as u32;
        let tb = top.blue() as u32;

        let ba = bottom.alpha() as u32;
        let br = bottom.red() as u32;
        let bg = bottom.green() as u32;
        let bb = bottom.blue() as u32;

        let inv_ta = 255 - ta;

        let r = (tr * ta + br * inv_ta) >> 8;
        let g = (tg * ta + bg * inv_ta) >> 8;
        let b = (tb * ta + bb * inv_ta) >> 8;

        let a = ta + ((ba * inv_ta) >> 8);

        Self::rgba(r as u8, g as u8, b as u8, a as u8)
    }

    pub const BLACK: Color = Color(0xFF000000);
    pub const WHITE: Color = Color(0xFFFFFFFF);
    pub const RED: Color = Color(0xFFFF0000);
    pub const GREEN: Color = Color(0xFF00FF00);
    pub const BLUE: Color = Color(0xFF0000FF);
    pub const CYAN: Color = Color(0xFF00FFFF);
    pub const MAGENTA: Color = Color(0xFFFF00FF);
    pub const YELLOW: Color = Color(0xFFFFFF00);
}

impl From<Color> for u32 {
    fn from(color: Color) -> Self {
        color.0
    }
}