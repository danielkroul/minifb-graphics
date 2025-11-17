use crate::{Color, Sprite};

pub struct Buffer {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u32>,
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Buffer {
        Buffer {
            width: width,
            height: height,
            pixels: vec![0; width * height],
        }
    }

    fn coords_to_index(&self, x: i32, y: i32) -> i32 {
        y * self.width as i32 + x
    }

    /// Sets a single pixel
    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        if x < self.width as i32 && y < self.height as i32 {
            let index = self.coords_to_index(x, y);
            self.pixels[index as usize] = color.into();
        }
    }

    /// Fills the buffer with a single color
    pub fn clear(&mut self, color: Color) {
        let color = color.into();
        self.pixels
            .iter_mut()
            .for_each(|i| *i = color);
    }

    // A private helper for a fast horizontal line.
    fn draw_hline(&mut self, x1: i32, y: i32, x2: i32, color: Color) {
        let (start_x, end_x) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
        for x in start_x..=end_x { // ..= is an inclusive range
            self.set_pixel(x, y, color);
        }
    }

    // A private helper for a fast vertical line.
    fn draw_vline(&mut self, x: i32, y1: i32, y2: i32, color: Color) {
        let (start_y, end_y) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
        for y in start_y..=end_y {
            self.set_pixel(x, y, color);
        }
    }

    /// Draws a line between two points
    pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: Color) {
        if y1 == y2 {
            self.draw_hline(x1, y1, x2, color);
            return;
        }
        if x1 == x2 {
            self.draw_vline(x1, y1, y2, color);
            return;
        }
        
        let dx = x2 - x1;
        let dy = y2 - y1;
        
        let steps = dx.abs().max(dy.abs());

        if steps == 0 {
            self.set_pixel(x1, y1, color);
            return;
        }

        let step_x = dx as f64 / steps as f64;
        let step_y = dy as f64 / steps as f64;

        let mut current_x = x1 as f64;
        let mut current_y = y1 as f64;

        for _ in 0..=steps {
            self.set_pixel(current_x.round() as i32, current_y.round() as i32, color);
            current_x += step_x;
            current_y += step_y;
        }
    }

    /// Draws the outline of a rectangle.
    pub fn draw_rect(&mut self, x: i32, y: i32, width: i32, height: i32, color: Color) {
        if width <= 0 || height <= 0 { return; }
        let x2 = x + width - 1;
        let y2 = y + height - 1;

        self.draw_hline(x, y, x2, color);   // Top
        self.draw_hline(x, y2, x2, color);  // Bottom
        self.draw_vline(x, y, y2, color);   // Left
        self.draw_vline(x2, y, y2, color);  // Right
    }

    /// Draws a solid, filled rectangle using a fast path helper.
    pub fn fill_rect(&mut self, x: i32, y: i32, width: i32, height: i32, color: Color) {
        if width <= 0 || height <= 0 { return; }
        let x2 = x + width - 1;
        let y2 = y + height - 1;

        for py in y..=y2 {
            self.draw_hline(x, py, x2, color);
        }

        (y..=y2).for_each(|py| self.draw_hline(x, py, x2, color));
    }

    pub fn _draw_circle(&mut self, _x: i32, _y: i32, _radius: i32, _color: Color) {
        
    }

    pub fn _fill_circle(&mut self, _x: i32, _y: i32, _radius: i32, _color: Color) {

    }

    pub fn draw_buffer(&mut self, x: i32, y: i32, buffer: &Buffer) {
        for i in 0..buffer.pixels.len() {
            let dx = x + i as i32 % buffer.width as i32;
            let dy = y + i as i32 / buffer.width as i32;
            self.set_pixel(dx, dy, Color(buffer.pixels[i]));
        }
    }
}