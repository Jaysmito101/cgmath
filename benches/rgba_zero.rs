extern crate cgmath;

use cgmath::{num_traits::Zero, vec4, Vector4};
use std::hint::black_box;
use std::time::Instant;

const PIXELS: usize = 4096;
const PASSES: usize = 128;

fn measure(name: &str, pixels: &[Vector4<f32>]) {
    let mut mask = vec![0u8; pixels.len()];
    let mut samples = [0.0; 15];
    for sample in &mut samples {
        let start = Instant::now();
        for _ in 0..PASSES {
            for (dst, pixel) in mask.iter_mut().zip(black_box(pixels)) {
                *dst = pixel.is_zero() as u8;
            }
            black_box(&mask);
        }
        *sample = start.elapsed().as_secs_f64() * 1e9 / (pixels.len() * PASSES) as f64;
    }
    samples.sort_by(f64::total_cmp);
    println!("{name} {:.6}", samples[samples.len() / 2]);
}

fn main() {
    let clear = vec![vec4(0.0, -0.0, 0.0, -0.0); PIXELS];
    let mut rng = 91u32;
    let mut sparse = clear.clone();
    let mut dense = clear.clone();
    for (sparse, dense) in sparse.iter_mut().zip(&mut dense) {
        rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
        let value = (1 + (rng >> 16)) as f32 / 65536.0;
        if rng & 0x8000 != 0 {
            sparse[((rng >> 12) & 3) as usize] = value;
        }
        *dense = vec4(value, value * 0.75, value * 0.5, 1.0);
    }
    measure("clear", &clear);
    measure("sparse", &sparse);
    measure("dense", &dense);
}
