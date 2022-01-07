use std::{sync, thread};
use std::char::MAX;
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::thread::spawn;
use std::time::Duration;

use minifb::{Key, Window, WindowOptions};
use rand::Rng;

use crate::utils::{pixel_color, pixel_to_values, rand_f64};

mod utils;

const POS_X: i32 = -2;
const POS_Y: i32 = -2;
const HEIGHT: f64 = 2.5;

const IMG_WIDTH: usize = 1023;
const IMG_HEIGHT: usize = 1023;
const IMG_RATIO: f64 = IMG_WIDTH as f64 / IMG_HEIGHT as f64;

const NUM_BLOCKS: usize = 128;
const NUM_THREADS: usize = 10;

const SAMPLES: usize = 100;
const MAX_ITER: usize = 500;

pub struct Pixel {
    x: usize,
    y: usize,
    r: u8,
    b: u8,
    g: u8,
}

struct WorkItem {
    initial_x: usize,
    final_x: usize,
    initial_y: usize,
    final_y: usize,
}

fn work_item_creator(work_chan: Sender<WorkItem>) {
    let sqrt = (NUM_BLOCKS as f64).sqrt() as usize;
    let mut counter = 0;
    for i in 0..sqrt {
        for j in 0..sqrt {
            work_chan.send(WorkItem {
                initial_x: i * (IMG_WIDTH / sqrt),
                final_x: (i + 1) * (IMG_WIDTH / sqrt),
                initial_y: j * (IMG_HEIGHT / sqrt),
                final_y: (j + 1) * (IMG_HEIGHT / sqrt),
            });
            counter += 1;
            println!("Created item #{}", counter)
        }
    }
}

fn worker_creator(work_rx: Receiver<WorkItem>, result_tx: Sender<Pixel>) {
    let (status_tx, status_receive) = mpsc::channel();
    for i in 0..NUM_THREADS {
        status_tx.send(true);
    }

    for (num, _) in status_receive.iter().enumerate() {
        let work_item = work_rx.recv().unwrap();
        let result_tx = result_tx.clone();
        let status_tx = status_tx.clone();
        // println!("Starting worker #{} with item {} {} {} {}", num, work_item.initial_x, work_item.final_x, work_item.initial_y, work_item.final_y);
        let _ = thread::spawn(move || worker(work_item, result_tx, status_tx));
    }
}

fn worker(work_item: WorkItem, result_tx: Sender<Pixel>, status_tx: Sender<bool>) {
    for x in work_item.initial_x..work_item.final_x {
        for y in work_item.initial_y..work_item.final_y {
            let (mut col_r, mut col_g, mut col_b): (u64, u64, u64) = (0, 0, 0);
            for _ in 0..SAMPLES {
                let a = HEIGHT as f64 * IMG_RATIO * (((x as f64) + rand_f64()) / (IMG_WIDTH as f64)) + POS_X as f64;
                let b = HEIGHT as f64 * (((y as f64) + rand_f64()) / (IMG_HEIGHT as f64)) + POS_Y as f64;
                let (r, iter) = mandelbrot_iteration(a, b);
                let (r, g, b) = pixel_color(r, iter);
                col_r += r as u64;
                col_g += g as u64;
                col_b += b as u64;
            }
            let (cr, cg, cb): (u8, u8, u8);
            cr = ((col_r as f64) / (SAMPLES as f64)) as u8;
            cg = ((col_g as f64) / (SAMPLES as f64)) as u8;
            cb = ((col_b as f64) / (SAMPLES as f64)) as u8;

            result_tx.send(Pixel {
                x,
                y,
                r: cr,
                b: cb,
                g: cg,
            });
        }
    }
    status_tx.send(true);
}

// fn white_generator(buffer: Arc<Mutex<Vec<u32>>>) {
//     let (tx, rx) = sync::mpsc::channel();
//     for i in 0..IMG_HEIGHT * IMG_WIDTH {
//         let tx = tx.clone();
//         let i = i.clone();
//         thread::spawn(move || {
//             let duration = rand::thread_rng().gen_range(0..4000);
//             thread::sleep(Duration::from_millis(duration));
//             tx.send(Pixel);
//         });
//     }
//     println!("done creating");
//     std::mem::drop(tx);
//     for x in rx.iter() {
//         let buffer = buffer.clone();
//         {
//             let mut guard = buffer.lock().unwrap();
//             guard[x.idx] = x.color;
//         }
//     }
// }

fn buffer_updater(buffer: Arc<Mutex<Vec<u32>>>, pixel_receive: Receiver<Pixel>) {
    for pixel in pixel_receive.iter() {
        let (idx, color) = pixel_to_values(pixel);
        let buffer = buffer.clone();
        {
            let mut guard = buffer.lock().unwrap();
            guard[idx] = color;
        }
    }
}

fn mandelbrot_iteration(a: f64, b: f64) -> (f64, usize) {
    let (mut x, mut y, mut xx, mut yy, mut xy): (f64, f64, f64, f64, f64) = (0.0, 0.0, 0.0, 0.0, 0.0);
    for i in 0..MAX_ITER {
        xx = x * x;
        yy = y * y;
        xy = x * y;
        if xx + yy > 4.0 {
            return (xx + yy, i);
        }
        x = xx - yy + a;
        y = 2_f64 * xy + b;
    }

    (xx + yy, MAX_ITER)
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

    let (item_send, item_receive) = mpsc::channel();
    let (result_send, result_receive) = mpsc::channel();

    let _ = thread::spawn(move || work_item_creator(item_send));
    let _ = thread::spawn(move || worker_creator(item_receive, result_send));

    let buff_update = buffer.clone();
    let _ = thread::spawn(move || buffer_updater(buff_update, result_receive));

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
