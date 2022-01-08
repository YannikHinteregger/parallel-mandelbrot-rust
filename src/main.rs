use std::thread;
use std::sync::{Arc, mpsc, Mutex};

use clap::Parser;
use minifb::{Key, Window, WindowOptions};

use crate::mandelbrot::{buffer_updater, work_item_creator, worker_creator};
use crate::utils::{pixel_color, pixel_to_values, rand_f64, rgb_to_u32};

mod utils;
mod mandelbrot;


#[derive(Clone, Copy)]
pub struct Config {
    side_lengths: usize,
    img_ratio: f64,
    num_blocks: usize,
    num_threads: usize,
    samples: usize,
    max_iter: usize,
}

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(long, default_value_t = 1000)]
    side_lengths: usize,

    #[clap(long, default_value_t = 100)]
    num_blocks: usize,

    #[clap(long, default_value_t = 10)]
    num_threads: usize,

    #[clap(long, default_value_t = 200)]
    samples: usize,

    #[clap(long, default_value_t = 500)]
    max_iter: usize,
}

fn main() {
    let args = Args::parse();
    let config = Config {
        side_lengths: args.side_lengths,
        img_ratio: args.side_lengths as f64 / args.side_lengths as f64,
        num_blocks: args.num_blocks,
        num_threads: args.num_threads,
        samples: args.samples,
        max_iter: args.max_iter,
    };

    println!("Initialise processing...");
    let buff: Vec<u32> = vec![rgb_to_u32(255, 255, 255); config.side_lengths * config.side_lengths];
    let buffer = Arc::new(Mutex::new(buff));

    let mut window = Window::new(
        "Mandelbrot - ESC to exit",
        config.side_lengths,
        config.side_lengths,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let (item_send, item_receive) = mpsc::channel();
    let (result_send, result_receive) = mpsc::channel();

    let _ = thread::spawn(move || work_item_creator(item_send, config));
    let _ = thread::spawn(move || worker_creator(item_receive, result_send, config));

    let buff_update = buffer.clone();
    let _ = thread::spawn(move || buffer_updater(buff_update, result_receive, config));

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    println!("Rendering...");
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let buffer = buffer.clone();
        let buffer_copy;
        {
            buffer_copy = buffer.lock().unwrap().to_vec();
        }
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer_copy, config.side_lengths, config.side_lengths)
            .unwrap();
    }
}