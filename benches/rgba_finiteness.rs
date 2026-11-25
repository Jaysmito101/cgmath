extern crate cgmath;

use cgmath::{Array, BaseFloat, Vector4};
use std::hint::black_box;
use std::time::Instant;

const PIXELS: usize = 4096;
const PASSES: usize = 128;
const SAMPLES: usize = 12;

fn measure<S: BaseFloat>(name: &str, mixed: bool) {
    let pixels: Vec<_> = (0..PIXELS)
        .map(|i| {
            let t = S::from(i).unwrap() / S::from(PIXELS).unwrap();
            let mut pixel = Vector4::new(t, S::one() - t, t * t, S::one());
            if mixed && i % 11 == 0 {
                pixel[(i / 11) % 4] = if i % 3 == 0 { S::nan() } else { S::infinity() };
            }
            pixel
        })
        .collect();
    let mut mask = vec![0u8; PIXELS];
    let mut pass = || {
        for (flag, pixel) in mask.iter_mut().zip(black_box(pixels.as_slice())) {
            *flag = u8::from(pixel.is_finite());
        }
        black_box(mask.as_slice());
    };
    for _ in 0..8 {
        pass();
    }
    print!("{}", name);
    for _ in 0..SAMPLES {
        let start = Instant::now();
        for _ in 0..PASSES {
            pass();
        }
        let ns_per_pixel = start.elapsed().as_secs_f64() * 1e9 / (PASSES * PIXELS) as f64;
        print!(" {:.6}", ns_per_pixel);
    }
    println!();
}

fn main() {
    measure::<f32>("finite-f32", false);
    measure::<f32>("mixed-f32", true);
    measure::<f64>("finite-f64", false);
    measure::<f64>("mixed-f64", true);
}
