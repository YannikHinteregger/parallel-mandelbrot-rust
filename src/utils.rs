use rand::prelude::*;
use crate::mandelbrot::Pixel;

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
    (0, 0, 0) // if in set, return black
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rgb_to_u32() {
        assert_eq!(0, rgb_to_u32(0, 0, 0));
        assert_eq!(16777215, rgb_to_u32(255, 255, 255));
        assert_eq!(16711680, rgb_to_u32(255, 0, 0));
        assert_eq!(65280, rgb_to_u32(0, 255, 0));
        assert_eq!(255, rgb_to_u32(0, 0, 255));
    }

    #[test]
    fn test_pixel_color() {
        assert_eq!((255, 0, 250), pixel_color(4.1, 4));
        assert_eq!((203, 255, 0), pixel_color(20.0, 4));
        assert_eq!((0, 0, 0), pixel_color(3.9, 10));
        assert_eq!((0, 0, 0), pixel_color(3.0, 0));
    }

    #[test]
    fn test_hsl_to_rgb() {
        assert_eq!((195, 0, 255), hsl_to_rgb(0.20566146450275685, 1.0, 0.5));
        assert_eq!((0, 0, 0), hsl_to_rgb(13.6414189022027, 1.0, 0.5));
        assert_eq!((0, 255, 80), hsl_to_rgb(0.6143762624896743, 1.0, 0.5));
        assert_eq!((66, 255, 0), hsl_to_rgb(0.7101028637264182, 1.0, 0.5));
        assert_eq!((0, 0, 0), hsl_to_rgb(8.01207663335424, 1.0, 0.5));
        assert_eq!((93, 0, 255), hsl_to_rgb(1.2725352898356532, 1.0, 0.5));
        assert_eq!((0, 0, 0), hsl_to_rgb(5.112130021844454, 1.0, 0.5));
        assert_eq!((0, 0, 0), hsl_to_rgb(4.385813740592475, 1.0, 0.5));
    }

    #[test]
    fn test_hue_to_color_chan() {
        assert_eq!(0, hue_to_color_chan(0.0, 1.0, 0.9818469993127288));
        assert_eq!(0, hue_to_color_chan(0.0, 1.0, 0.9063340634233428));
        assert_eq!(255, hue_to_color_chan(0.0, 1.0, 0.4787203590719167));
        assert_eq!(0, hue_to_color_chan(0.0, 1.0, 0.8153499114293143));
        assert_eq!(255, hue_to_color_chan(0.0, 1.0, 0.2288429838508271));
        assert_eq!(209, hue_to_color_chan(0.0, 1.0, 0.136648500409548));
        assert_eq!(255, hue_to_color_chan(0.0, 1.0, 0.3827122883555413));
        assert_eq!(255, hue_to_color_chan(0.0, 1.0, 0.31463978895080813));
        assert_eq!(250, hue_to_color_chan(0.0, 1.0, 0.1634889227893747));
        assert_eq!(0, hue_to_color_chan(0.0, 1.0, -0.06651326739036106));
    }

    #[test]
    fn test_pixel_to_values() {
        assert_eq!((793441, 590079), pixel_to_values(Pixel { x: 441, y: 793, r: 9, g: 0, b: 255 }, 1000));
        assert_eq!((853494, 65361), pixel_to_values(Pixel { x: 494, y: 853, r: 0, g: 255, b: 81 }, 1000));
        assert_eq!((566341, 0), pixel_to_values(Pixel { x: 341, y: 566, r: 0, g: 0, b: 0 }, 1000));
        assert_eq!((885684, 10944767), pixel_to_values(Pixel { x: 684, y: 885, r: 167, g: 0, b: 255 }, 1000));
        assert_eq!((675756, 0), pixel_to_values(Pixel { x: 756, y: 675, r: 0, g: 0, b: 0 }, 1000));
        assert_eq!((923890, 16711875), pixel_to_values(Pixel { x: 890, y: 923, r: 255, g: 0, b: 195 }, 1000));
        assert_eq!((404979, 5869454), pixel_to_values(Pixel { x: 979, y: 404, r: 89, g: 143, b: 142 }, 1000));
    }
}