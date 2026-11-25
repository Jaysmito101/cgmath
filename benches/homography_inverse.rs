extern crate cgmath;

use cgmath::{BaseFloat, Matrix3, SquareMatrix};
use std::hint::black_box;
use std::time::Instant;

const TILE_COUNT: usize = 4096;
const PASSES: usize = 128;
const SAMPLES: usize = 15;

fn measure<S: BaseFloat>(name: &str) {
    let input: Vec<_> = (0..TILE_COUNT)
        .map(|i| {
            let t = i as f64 / TILE_COUNT as f64;
            Matrix3::new(
                0.82 + t,
                0.31,
                -0.0003,
                -0.44,
                1.13,
                0.0007,
                127.52,
                48.23 + t,
                1.0,
            )
            .cast::<S>()
            .unwrap()
        })
        .collect();
    let mut output = vec![None; TILE_COUNT];
    let mut samples = [0.0; SAMPLES];

    for sample in &mut samples {
        let start = Instant::now();
        for _ in 0..PASSES {
            for (dst, src) in output.iter_mut().zip(black_box(input.as_slice())) {
                *dst = src.invert();
            }
            black_box(output.as_slice());
        }
        *sample = start.elapsed().as_secs_f64() * 1e9 / (PASSES * input.len()) as f64;
    }

    samples.sort_by(f64::total_cmp);
    println!("{} {:.6} ns/matrix", name, samples[SAMPLES / 2]);
}

fn main() {
    measure::<f32>("f32");
    measure::<f64>("f64");
}
