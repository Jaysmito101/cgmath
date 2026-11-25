extern crate cgmath;

use cgmath::{Euler, Quaternion, Rad};
use std::hint::black_box;
use std::time::Instant;

const ORIENTATIONS: usize = 4096;
const PASSES: usize = 32;
const SAMPLES: usize = 15;

fn measure(name: &str, input: &[Quaternion<f32>], adjustment: impl Fn(usize) -> Quaternion<f32>) {
    let mut output = vec![input[0]; input.len()];
    let mut batch = || {
        for _ in 0..PASSES {
            for (i, (dst, &orientation)) in output.iter_mut().zip(black_box(input)).enumerate() {
                *dst = adjustment(i) * orientation;
            }
            black_box(&output);
        }
    };
    batch();
    let mut samples: Vec<_> = (0..SAMPLES)
        .map(|_| {
            let start = Instant::now();
            batch();
            start.elapsed().as_secs_f64() * 1e9 / (PASSES * input.len()) as f64
        })
        .collect();
    samples.sort_unstable_by(f64::total_cmp);
    println!("{} {:.4} ns/pair", name, samples[SAMPLES / 2]);
}

fn main() {
    let orientation = |i: usize| {
        let angle = |shift| ((i.wrapping_mul(0x9e37_79b9) >> shift) as u8 as f32 - 128.0) / 64.0;
        Quaternion::from(Euler {
            x: Rad(angle(0)),
            y: Rad(angle(8)),
            z: Rad(angle(16)),
        })
    };
    let input: Vec<_> = (0..ORIENTATIONS).map(orientation).collect();
    let adjustments: Vec<_> = (ORIENTATIONS..2 * ORIENTATIONS).map(orientation).collect();
    let fixed = black_box(adjustments[0]);
    let adjustments = black_box(&adjustments);
    measure("fixed_adjustment", &input, |_| fixed);
    measure("varying_adjustments", &input, |i| adjustments[i]);
}
