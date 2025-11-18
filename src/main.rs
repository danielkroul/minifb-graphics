use minifb::{Window, WindowOptions};
use minifb_graphics::{Buffer, Color};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let mut buffer = Buffer::new(WIDTH, HEIGHT);
    let mut sprite = Buffer::new(100, 100);
    let mut gradient = Buffer::new(250, 250);

    sprite.clear(Color::BLUE);
    sprite.draw_line(0, 0, 100, 100, Color::RED);
    sprite.draw_buffer(50, 50, &buffer);

    gradient.clear(Color::rgba(255, 255, 255, 0.0));
    for x in 0..gradient.width {
        for y in 0..gradient.height {
            gradient.set_pixel(
                x as i32,
                y as i32,
                Color::rgba(x as u8, y as u8, 255, 0.50)
            );
        }
    }

    let mut window = Window::new(
        "minifb lines",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).expect("Unable to create window.");



    while window.is_open() {
        draw(&mut buffer, &sprite, &gradient);
        window.update_with_buffer(&buffer.pixels, WIDTH, HEIGHT).unwrap();
    }
}

fn draw(buffer: &mut Buffer, sprite: &Buffer, gradient: &Buffer) {
    buffer.clear(Color::rgba(40, 40, 30, 1.0));
    buffer.draw_buffer(200, 200, sprite);
    buffer.draw_buffer(230, 30, gradient);
    //buffer.fill_rect(230, 50, 400, 230, Color::rgba(249, 38, 38, 0.28));
}