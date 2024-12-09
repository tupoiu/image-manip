use rayon::prelude::*;
use image::{open, ImageBuffer, Pixel, Rgb};
use chrono::Local;
use rand::Rng;


fn to_u8 (u : f64) -> u8 {
    (u * 255.0) as u8
}

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
        let freq = 2_i32.pow(6 - i);

        let (oct_r,oct_g,oct_b) = noise(x as i32, y as i32, freq);
        r += octave_weight * oct_r;
        g += octave_weight * oct_g;
        b += octave_weight * oct_b;
    }

    r /= weight;
    g /= weight;
    b /= weight;

    let noise_1 = value_at(x as i32, y as i32, 100, 7) as f64;
    let noise_2 = value_at(x as i32, y as i32, 100, 8) as f64;

    let theta = 2. * std::f64::consts::PI * noise_1;

    let cx = (theta.cos() + 1.) / 2.;
    let cy = (theta.sin() + 1.) / 2.;

    let squish = |x: f64| (x * 8.).round() / 8.;

    r = (cx + 0.2*cy) / 1.1;
    g = 0.6 * (0.2*cx + cx*cy);
    b = 0.5*cy.sqrt();

    if noise_2 > 0.7 {
        r = g;
        b = g;
    } else if noise_2 < 0.3 {
        g = r;
        b = r;
    }

    (to_u8(r), to_u8(g), to_u8(b))
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    return a + (b-a) * t
}

fn sstep(a: f64, b: f64, t: f64) -> f64 {
    let theta = 2. * std::f64::consts::PI * t;
    let t = (theta.cos() + 1.) / 2.;
    return lerp(a, b, t)
}

fn coslerp(a: f64, b: f64, t: f64) -> f64 {
    let theta = std::f64::consts::PI * t;
    let t = (1. - theta.cos()) / 2.;
    return lerp(a, b, t)
}

fn value_at(x: i32, y: i32, freq: i32, seed: i32) -> f64 {
    // let lerp = lerp;
    let lerp = coslerp;

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

fn noise(x: i32, y: i32, freq: i32) -> (f64, f64, f64) {

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
    let z = (y * 234567 - seed) % 30983;
    let a = z * x + 1;
    let b = (z * x % (y.abs() + 10) / (a.abs() + 1)) as f64; 
    let c = 4.5 + (a / (10000 * seed)) as f64 + z as f64;
    let d = b.abs() + c + (0.2 * seed as f64);
    let e = (d + ((5 - 7*seed)*x + (3 + seed)*y) as f64 * (z as f64/293.0)) / (c.abs());
    return e.abs() - e.abs().floor();
}

fn main() {
    // create_new_img();
    meld_imgs();
}

fn temp_path() -> String {
    let now = Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    print!("Now: {}", now);
    let path = format!("archive/output_{}.png", now);
    return path;
}

fn meld_imgs(){

    let image1_path = "archive/output_2024-12-08_17-48-20.png";
    let image2_path = "archive/output_2024-12-08_17-51-21.png";
    let output_path = "output.png";

    // Open the images
    let img1 = open(image1_path).expect("Failed to open image 1").to_rgb8();
    let img2 = open(image2_path).expect("Failed to open image 2").to_rgb8();

    // Ensure both images have the same dimensions
    if img1.dimensions() != img2.dimensions() {
        panic!("Images must have the same dimensions");
    }

    let (width, height) = img1.dimensions();

    // Create an output image buffer
    let mut output_image = ImageBuffer::new(width, height);

    // Randomly sample pixels from both images
    let mut rng = rand::thread_rng();

    for (x, y, pixel) in output_image.enumerate_pixels_mut() {
        // Get the corresponding pixels from both images
        let pixel1 = img1.get_pixel(x, y);
        let pixel2 = img2.get_pixel(x, y);

        // Randomly select one of the pixels
        let p = x as f64 / width as f64;
        let p = coslerp(-0.4, 1.4, p).clamp(0., 1.);

        let r = lerp(pixel1[0] as f64, pixel2[0] as f64, 1. - p) / 255.; 
        let g = lerp(pixel1[1] as f64, pixel2[1] as f64, 1. - p) / 255.; 
        let b = lerp(pixel1[2] as f64, pixel2[2] as f64, 1. - p) / 255.; 
        
        // let chosen_pixel = if rng.gen_bool(p) { pixel1 } else { pixel2 };

        // Set the chosen pixel in the output image
        *pixel = Rgb([to_u8(r),to_u8(g),to_u8(b)]);
    }

    // Save the output image
    output_image
        .save(output_path)
        .expect("Failed to save the output image");
    output_image
        .save(temp_path())
        .expect("Failed to save the output image");

    println!("Output image saved to {}", output_path);
}

fn create_new_img() {
    let width = 1920;
    let height = 540;

    // Create a buffer to store image data
    let mut img_buffer = ImageBuffer::new(width, height);

    // Parallelize row processing
    img_buffer.enumerate_rows_mut().par_bridge().for_each(|(y, row)| {
        for (x, _, pixel) in row {
            let (r, g, b) = compute_pixel(2*x, 2*y, width, height);
            *pixel = Rgb([r, g, b]);
        }
    });

    // Save the image
    img_buffer.save("output.png").unwrap();
    
    img_buffer.save(temp_path()).unwrap();
}
