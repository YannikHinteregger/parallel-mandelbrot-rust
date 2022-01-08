use rand::prelude::*;

use crate::{IMG_WIDTH, Pixel};

pub fn rgb_to_u32(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

pub fn rand_f64() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

pub fn pixel_color(r: f64, iter: usize) -> (u8, u8, u8) {
    if r > 4 as f64 {
        return hsl_to_rgb((iter as f64) / 100_f64 * r, 1.0, 0.5);
    }
    (0, 0, 0) // if in set return black
}

pub fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
    let (r, g, b): (u8, u8, u8);
    if s as usize == 0 {
        r = l as u8;
        b = l as u8;
        g = l as u8;
    } else {
        let (q, p): (f64, f64);
        if l < 0.5 {
            q = l * (1_f64 + s);
        } else {
            q = l + s - l * s;
        }
        p = 2_f64 * l - q;
        r = hue_to_color_chan(p, q, h + 1.0 / 3.0);
        g = hue_to_color_chan(p, q, h);
        b = hue_to_color_chan(p, q, h - 1.0 / 3.0);
    }
    (r, b, g)
}

pub fn hue_to_color_chan(p: f64, q: f64, mut t: f64) -> u8 {
    let ret_val: f64;
    if t < 0_f64 {
        t += 1_f64
    }
    if t > 1_f64 {
        t -= 1_f64
    }

    if t < 1.0 / 6.0 {
        ret_val = p + (q - p) * 6_f64 * t;
    } else if t < 1.0 / 2.0 {
        ret_val = q;
    } else if t < 2.0 / 3.0 {
        ret_val = p + (q - p) * (2.0 / 3.0 - t) * 6_f64;
    } else {
        ret_val = p
    }
    (ret_val * 255_f64) as u8
}

pub fn pixel_to_values(pixel: Pixel, width: usize) -> (usize, u32) {
    let idx = width * (pixel.y) + pixel.x;
    let color = rgb_to_u32(pixel.r, pixel.g, pixel.b);
    (idx, color)
}