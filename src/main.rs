mod utils;

use std::thread;
use std::time::Duration;
use minifb::{Key, Window, WindowOptions};

const IMG_WIDTH: usize = 1023;
const IMG_HEIGHT: usize = 1023;

fn main() {
    utils::print_hello();
    let mut buffer: Vec<u32> = vec![0; IMG_WIDTH * IMG_HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        IMG_WIDTH as usize,
        IMG_HEIGHT as usize,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            *i = utils::rgb_to_u32(255, 255, 255);
            // thread::sleep(Duration::from_millis(1));
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, IMG_WIDTH, IMG_HEIGHT)
            .unwrap();
    }
}

struct WorkItem {
    initial_x: i32,
    final_x: i32,
    initial_y: i32,
    final_y: i32,
}

