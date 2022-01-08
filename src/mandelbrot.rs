use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use crate::{Config, pixel_color, pixel_to_values, rand_f64};

const SCALE_X: f64 = -2.0;
const SCALE_Y: f64 = -1.255;
const SCALE_HEIGHT: f64 = 2.5;

pub struct Pixel {
    pub x: usize,
    pub y: usize,
    pub r: u8,
    pub b: u8,
    pub g: u8,
}

impl PartialEq for Pixel {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x &&
            self.y == other.y &&
            self.r == other.r &&
            self.b == other.b &&
            self.g == other.g
    }
}

pub struct WorkItem {
    pub initial_x: usize,
    pub final_x: usize,
    pub initial_y: usize,
    pub final_y: usize,
}

impl PartialEq for WorkItem {
    fn eq(&self, other: &Self) -> bool {
        self.initial_x == other.initial_x &&
            self.final_x == other.final_x &&
            self.initial_y == other.initial_y &&
            self.final_y == other.final_y
    }
}

pub fn work_item_creator(work_chan: Sender<WorkItem>, config: Config) {
    let sqrt = (config.num_blocks as f64).sqrt() as usize;
    for i in 0..sqrt {
        for j in 0..sqrt {
            let _ = work_chan.send(WorkItem {
                initial_x: i * (config.side_lengths / sqrt),
                final_x: (i + 1) * (config.side_lengths / sqrt),
                initial_y: j * (config.side_lengths / sqrt),
                final_y: (j + 1) * (config.side_lengths / sqrt),
            });
        }
    }
}

pub fn worker_creator(work_rx: Receiver<WorkItem>, result_tx: Sender<Pixel>, config: Config) {
    let (status_tx, status_receive) = mpsc::channel();
    for _ in 0..config.num_threads {
        let _ = status_tx.send(true);
    }

    for _ in status_receive.iter() {
        let work_item = work_rx.recv();
        if work_item.is_err() { return; }
        let result_tx = result_tx.clone();
        let status_tx = status_tx.clone();
        let _ = thread::spawn(move || worker(work_item.unwrap(), result_tx, status_tx, config));
    }
}

pub fn worker(work_item: WorkItem, result_tx: Sender<Pixel>, status_tx: Sender<bool>, config: Config) {
    for x in work_item.initial_x..work_item.final_x {
        for y in work_item.initial_y..work_item.final_y {
            let (mut col_r, mut col_g, mut col_b): (u64, u64, u64) = (0, 0, 0);
            for _ in 0..config.samples {
                let a = SCALE_HEIGHT as f64 * config.img_ratio * (((x as f64) + rand_f64()) / (config.side_lengths as f64)) + SCALE_X;
                let b = SCALE_HEIGHT as f64 * (((y as f64) + rand_f64()) / (config.side_lengths as f64)) + SCALE_Y;
                let (r, iter) = mandelbrot_iteration(a, b, config.max_iter);
                let (r, g, b) = pixel_color(r, iter);
                col_r += r as u64;
                col_g += g as u64;
                col_b += b as u64;
            }
            let (cr, cg, cb): (u8, u8, u8);
            cr = ((col_r as f64) / (config.samples as f64)) as u8;
            cg = ((col_g as f64) / (config.samples as f64)) as u8;
            cb = ((col_b as f64) / (config.samples as f64)) as u8;

            let _ = result_tx.send(Pixel {
                x,
                y,
                r: cr,
                g: cg,
                b: cb,
            });
        }
    }
    let _ = status_tx.send(true);
}

pub fn buffer_updater(buffer: Arc<Mutex<Vec<u32>>>, pixel_receive: Receiver<Pixel>, config: Config) {
    for pixel in pixel_receive.iter() {
        let (idx, color) = pixel_to_values(pixel, config.side_lengths);
        let buffer = buffer.clone();
        {
            let mut guard = buffer.lock().unwrap();
            guard[idx] = color;
        }
    }
}

pub fn mandelbrot_iteration(a: f64, b: f64, max_iter: usize) -> (f64, usize) {
    let (mut x, mut y, mut xx, mut yy) : (f64, f64, f64, f64) = (0.0, 0.0, 0.0, 0.0);
    let mut xy: f64;
    for i in 0..max_iter {
        xx = x * x;
        yy = y * y;
        xy = x * y;
        if xx + yy > 4.0 {
            return (xx + yy, i);
        }
        x = xx - yy + a;
        y = 2_f64 * xy + b;
    }
    (xx + yy, max_iter)
}


#[cfg(test)]
mod test {
    use std::sync::{Arc, mpsc, Mutex};
    use std::thread;
    use crate::rgb_to_u32;
    use super::*;

    const TEST_ITEMS: [WorkItem; 10] = [
        WorkItem { initial_x: 0, final_x: 93, initial_y: 0, final_y: 93 },
        WorkItem { initial_x: 0, final_x: 93, initial_y: 93, final_y: 186 },
        WorkItem { initial_x: 0, final_x: 93, initial_y: 186, final_y: 279 },
        WorkItem { initial_x: 0, final_x: 93, initial_y: 279, final_y: 372 },
        WorkItem { initial_x: 0, final_x: 93, initial_y: 372, final_y: 465 },
        WorkItem { initial_x: 0, final_x: 93, initial_y: 465, final_y: 558 },
        WorkItem { initial_x: 0, final_x: 93, initial_y: 558, final_y: 651 },
        WorkItem { initial_x: 0, final_x: 93, initial_y: 651, final_y: 744 },
        WorkItem { initial_x: 0, final_x: 93, initial_y: 744, final_y: 837 },
        WorkItem { initial_x: 0, final_x: 93, initial_y: 837, final_y: 930 }
    ];

    const TEST_ITEM: WorkItem = WorkItem { initial_x: 0, final_x: 93, initial_y: 0, final_y: 93 };

    const TEST_PIXELS: [Pixel; 10] = [
        Pixel { x: 0, y: 0, r: 255, g: 0, b: 85 },
        Pixel { x: 0, y: 1, r: 255, g: 0, b: 84 },
        Pixel { x: 0, y: 2, r: 255, g: 0, b: 84 },
        Pixel { x: 0, y: 3, r: 255, g: 0, b: 84 },
        Pixel { x: 0, y: 4, r: 255, g: 0, b: 84 },
        Pixel { x: 0, y: 5, r: 255, g: 0, b: 84 },
        Pixel { x: 0, y: 6, r: 255, g: 0, b: 84 },
        Pixel { x: 0, y: 7, r: 255, g: 0, b: 84 },
        Pixel { x: 0, y: 8, r: 255, g: 0, b: 84 },
        Pixel { x: 0, y: 9, r: 255, g: 0, b: 84 },
    ];

    const CONFIG: Config = Config {
        side_lengths: 1023,
        img_ratio: 1023_f64 / 1023_f64,
        num_blocks: 128,
        num_threads: 1,
        samples: 200,
        max_iter: 500,
    };

    #[test]
    fn test_item_creator() {
        let (item_tx, item_rx) = mpsc::channel();
        thread::spawn(move || work_item_creator(item_tx, CONFIG));
        for i in 0..TEST_ITEMS.len() {
            assert!(TEST_ITEMS[i] == item_rx.recv().unwrap())
        }
    }

    #[test]
    fn test_worker() {

        let (result_tx, result_rx) = mpsc::channel();
        let (status_tx, _status_rx) = mpsc::channel();
        thread::spawn(move || worker(TEST_ITEM, result_tx, status_tx, CONFIG));
        for i in 0..TEST_PIXELS.len() {
            assert!(TEST_PIXELS[i] == result_rx.recv().unwrap())
        }
    }

    #[test]
    fn test_buffer_updater() {
        let config: Config = Config {
            side_lengths: 100,
            img_ratio: 100_f64 / 100_f64,
            num_blocks: 128,
            num_threads: 1,
            samples: 200,
            max_iter: 500,
        };

        let size = config.side_lengths;

        let buff = vec![rgb_to_u32(255, 255, 255); size * size];
        let buffer = Arc::new(Mutex::new(buff));

        let (pixel_tx, pixel_rx) = mpsc::channel();

        let result: Vec<u32> = vec![0; size * size];
        for x in 0..size {
            for y in 0..size {
                let _ = pixel_tx.send(Pixel { x, y, r: 0, g: 0, b: 0 });
            }
        }
        std::mem::drop(pixel_tx);
        buffer_updater(buffer.clone(), pixel_rx, config);
        let locked_buffer = buffer.lock().unwrap();
        for i in 0..result.len() {
            assert_eq!(locked_buffer[i], result[i]);
        }
    }

    #[test]
    fn test_mandelbrot_iteration() {
        let (r, iter) = mandelbrot_iteration(2_f64, 3_f64, 100);
        assert!(r == 13_f64 && iter == 1);

        let (r, iter) = mandelbrot_iteration(-0.872475504244032, -0.21588421079467435, 100);
        assert!(r == 0.8291297520206966 && iter == 100);
    }

    #[test]
    fn test_worker_creator() {
        let (work_tx, work_rx) = mpsc::channel();
        let (result_tx, result_rx) = mpsc::channel();
        let _ = work_tx.send(TEST_ITEM);
        std::mem::drop(work_tx);
        worker_creator(work_rx, result_tx, CONFIG);
        for pixel in TEST_PIXELS.iter() {
            assert!(*pixel == result_rx.recv().unwrap())
        }
    }
}