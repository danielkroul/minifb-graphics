use minifb::{Window, WindowOptions};
use minifb_graphics::{Buffer, Color};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let mut buffer = Buffer::new(WIDTH, HEIGHT);
    let mut sprite = Buffer::new(100, 100);

    sprite.clear(Color::BLUE);
    sprite.draw_line(0, 0, 100, 100, Color::RED);
    sprite.draw_buffer(50, 50, &buffer);

    let mut window = Window::new(
        "minifb lines",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).expect("Unable to create window.");



    while window.is_open() {
        draw(&mut buffer, &sprite);
        window.update_with_buffer(&buffer.pixels, WIDTH, HEIGHT).unwrap();
    }
}

fn draw(buffer: &mut Buffer, sprite: &Buffer) {
    buffer.clear(Color::rgb(40, 40, 30));
    buffer.draw_buffer(200, 200, sprite);
}