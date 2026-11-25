extern crate cgmath;

use cgmath::{vec3, Matrix3, SquareMatrix, Vector3};
use std::hint::black_box;
use std::time::Instant;

const PIXELS: usize = 4096;
const PASSES: usize = 16;
const SAMPLES: usize = 30;

fn measure<T: Copy, U: Copy>(name: &str, input: &[T], zero: U, kernel: impl Fn(T) -> U) {
    let mut output = vec![zero; input.len()];
    let mut batch = || {
        for _ in 0..PASSES {
            for (dst, &src) in output.iter_mut().zip(black_box(input)) {
                *dst = kernel(src);
            }
            black_box(&output);
        }
    };

    batch();
    for _ in 0..SAMPLES {
        let start = Instant::now();
        batch();
        let ns_per_item = start.elapsed().as_nanos() as f64 / (PASSES * input.len()) as f64;
        println!("{name},{ns_per_item:.4}");
    }
}

fn main() {
    let pixels: Vec<_> = (0..PIXELS)
        .map(|i| {
            let value = (i as u32).wrapping_mul(0x045d_9f3b);
            let channel = |shift| ((value >> shift) & 255u32) as f32 / 255.0;
            vec3(channel(0), channel(8), channel(16))
        })
        .collect();
    let zero = vec3(0.0, 0.0, 0.0);
    let color = black_box(Matrix3::from_cols(
        vec3(0.75, 0.125, 0.25),
        vec3(0.125, 0.75, 0.125),
        vec3(0.125, 0.125, 0.625),
    ));
    measure("rgb_f32", &pixels, zero, |pixel| color * pixel);

    let pixels_f64: Vec<Vector3<f64>> = pixels.iter().map(|pixel| pixel.map(f64::from)).collect();
    let color_f64 = black_box(Matrix3::from_cols(
        color.x.map(f64::from),
        color.y.map(f64::from),
        color.z.map(f64::from),
    ));
    measure("rgb_f64", &pixels_f64, zero.map(f64::from), |pixel| {
        color_f64 * pixel
    });

    let adjustments: Vec<_> = pixels.iter().copied().map(Matrix3::from_diagonal).collect();
    measure("compose_f32", &adjustments, color, |adjustment| {
        color * adjustment
    });
}
