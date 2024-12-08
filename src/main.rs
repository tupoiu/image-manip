use rayon::prelude::*;
use image::{ImageBuffer, Rgb};

fn compute_pixel(x: u32, y: u32, width: u32, height: u32) -> (u8, u8, u8) {
    // let r = (x as f32 / width as f32 * 255.0) as u8;
    // let g = (y as f32 / height as f32 * 255.0) as u8;
    // let b = 128;

    let (mut r, mut g, mut b) = (0.,0.,0.);
    let mut weight = 0.;
    let n_octaves = 5;
    for i in 1..=n_octaves {

        // Each layer is 1/3 of the weight of the previous
        let octave_weight = 3_f64.powf(-(i as f64));
        weight += octave_weight;
        let freq = 2_i32.pow(7 - i);

        let (oct_r,oct_g,oct_b) = noise(x as i32, y as i32, freq);
        r += octave_weight * oct_r;
        g += octave_weight * oct_g;
        b += octave_weight * oct_b;
    }

    r /= weight;
    g /= weight;
    b /= weight;

    let to_u8 = |u| (u * 255.0) as u8;

    (to_u8(r), to_u8(g), to_u8(b))
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    return a + (b-a) * t
}

fn noise(x: i32, y: i32, freq: i32) -> (f64, f64, f64) {
    fn value_at(x: i32, y: i32, freq: i32, seed: i32) -> f64 {
        let tl = rand(x/freq    , y/freq    , seed);
        let tr = rand(x/freq + 1, y/freq    , seed);
        let bl = rand(x/freq    , y/freq + 1, seed);
        let br = rand(x/freq + 1, y/freq + 1, seed);

        let t_x = (x % freq) as f64 / freq as f64;
        let t_y = (y % freq) as f64 / freq as f64;
        let top = lerp(tl, tr, t_x); 
        let bot = lerp(bl, br, t_x);

        return lerp(top, bot, t_y);
    }

    let out_x = value_at(x, y, freq, 1);
    let out_y = value_at(x, y, freq, 2);
    let out_z = value_at(x, y, freq, 3);
    return (out_x, out_y, out_z)
}

fn irand(x: i32) -> i32 {
    return x * 7654321 % 64811
}

// Returns number between 0 and 1
fn rand(x: i32, y: i32, seed: i32) -> f64 {
    let seed = irand(seed);
    let z = (y * 1234567 - seed) % 30983;
    let a = z * x + 1;
    let b = (z * x % (y.abs() + 10) / (a.abs() + 1)) as f64; 
    let c = 4.5 + (a / (100000 * seed)) as f64 + z as f64;
    let d = b.abs() + c + (0.2 * seed as f64);
    let e = (d + ((5 - 7*seed)*x + (3 + seed)*y) as f64 * (z as f64/293.0)) / (c.abs());
    return e.abs() - e.abs().floor();
}

fn main() {
    let width = 1920;
    let height = 540;

    // Create a buffer to store image data
    let mut img_buffer = ImageBuffer::new(width, height);

    // Parallelize row processing
    img_buffer.enumerate_rows_mut().par_bridge().for_each(|(y, row)| {
        for (x, _, pixel) in row {
            let (r, g, b) = compute_pixel(x, y, width, height);
            *pixel = Rgb([r, g, b]);
        }
    });

    // Save the image
    img_buffer.save("output.png").unwrap();
}
