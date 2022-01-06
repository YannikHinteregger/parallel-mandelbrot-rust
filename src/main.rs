mod utils;

use std::sync::{Arc, Mutex};
use std::{sync, thread};
use std::thread::spawn;
use std::time::Duration;
use minifb::{Key, Window, WindowOptions};
use rand::Rng;

const IMG_WIDTH: usize = 500;
const IMG_HEIGHT: usize = 500;

struct Pixel {
    idx: usize,
    color: u32,
}

fn white_generator(buffer: Arc<Mutex<Vec<u32>>>) {
    let (tx, rx) = sync::mpsc::channel();
    for i in 0..IMG_HEIGHT * IMG_WIDTH {
        let tx = tx.clone();
        let i = i.clone();
        thread::spawn(move || {
            let duration = rand::thread_rng().gen_range(0..4000);
            thread::sleep(Duration::from_millis(duration));
            tx.send(Pixel {
                idx: i,
                color: utils::rgb_to_u32(255, 255, 255),
            });
        });
    }
    println!("done creating");
    std::mem::drop(tx);
    for x in rx.iter() {
        let buffer = buffer.clone();
        {
            let mut guard = buffer.lock().unwrap();
            guard[x.idx] = x.color;
        }
    }
}

fn main() {
    let mut buff: Vec<u32> = vec![0; IMG_WIDTH * IMG_HEIGHT];
    let buffer = Arc::new(Mutex::new(buff));

    let mut window = Window::new(
        "Mandelbrot - ESC to exit",
        IMG_WIDTH as usize,
        IMG_HEIGHT as usize,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let test = buffer.clone();
    let _ = thread::spawn(|| white_generator(test));

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let buffer = buffer.clone();
        let buffer_copy;
        {
            buffer_copy = buffer.lock().unwrap().to_vec();
        }
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer_copy, IMG_WIDTH, IMG_HEIGHT)
            .unwrap();
    }
}

struct WorkItem {
    initial_x: i32,
    final_x: i32,
    initial_y: i32,
    final_y: i32,
}

