extern crate cgmath;

use cgmath::{vec4, Matrix4, Vector4};
use std::hint::black_box;
use std::time::Instant;

const PIXELS: usize = 128 * 128;
const PASSES: usize = 16;
const SAMPLES: usize = 400;

fn measure(name: &str, input: &[Vector4<f32>], kernel: impl Fn(Vector4<f32>) -> Vector4<f32>) {
    let mut output = vec![vec4(0.0, 0.0, 0.0, 0.0); input.len()];
    let mut batch = || {
        for _ in 0..PASSES {
            for (dst, &src) in output.iter_mut().zip(black_box(input)) {
                *dst = kernel(src);
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
    println!(
        "{name}: {:.3} ns/pixel (p10 {:.3}, p90 {:.3}; {SAMPLES} samples)",
        samples[SAMPLES / 2],
        samples[SAMPLES / 10],
        samples[SAMPLES * 9 / 10],
    );
}

fn main() {
    let pixels: Vec<_> = (0..PIXELS)
        .map(|i| {
            let value = (i as u32).wrapping_mul(0x9e37_79b9);
            let channel = |shift| ((value >> shift) & 255u32) as f32 / 255.0;
            vec4(channel(0), channel(8), channel(16), channel(24))
        })
        .collect();
    let color = black_box(Matrix4::from_cols(
        vec4(0.75, 0.125, 0.25, 0.0),
        vec4(0.125, 0.75, 0.125, 0.0),
        vec4(0.125, 0.125, 0.625, 0.0),
        vec4(0.0, 0.0, 0.0, 1.0),
    ));
    measure("RGBA color matrix", &pixels, |pixel| color * pixel);
    measure("RGBA channel square root", &pixels, |pixel| {
        #[cfg(feature = "simd")]
        return pixel.sqrt_element_wide();
        #[cfg(not(feature = "simd"))]
        pixel.map(f32::sqrt)
    });
}
