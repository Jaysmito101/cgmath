extern crate cgmath;

use cgmath::{BaseFloat, Matrix4, SquareMatrix};
use std::hint::black_box;
use std::time::Instant;

const PASSES: usize = 128;
const SAMPLES: usize = 15;

#[inline(never)]
fn measure<S: BaseFloat>(label: &str, blocks: &mut [Matrix4<S>]) {
    let mut samples = [0.0; SAMPLES];
    for sample in &mut samples {
        let start = Instant::now();
        for _ in 0..PASSES {
            for block in black_box(&mut *blocks) {
                block.transpose_self();
            }
        }
        *sample = start.elapsed().as_secs_f64() * 1e9 / (PASSES * blocks.len()) as f64;
    }
    samples.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("{}, {:.6}", label, samples[SAMPLES / 2]);
}

fn main() {
    let mut blocks: Vec<_> = (0..4096)
        .map(|block| {
            let pixels = std::array::from_fn(|col| {
                std::array::from_fn(|row| {
                    ((block * 17 + (col * 4 + row) * 53) % 256) as f32 / 255.0
                })
            });
            Matrix4::from(pixels)
        })
        .collect();
    let mut blocks_f64: Vec<_> = blocks.iter().map(|block| block.cast().unwrap()).collect();
    measure("transpose_f32_256", &mut blocks[..256]);
    measure("transpose_f32_4096", &mut blocks);
    measure::<f64>("transpose_f64_4096", &mut blocks_f64);
}
