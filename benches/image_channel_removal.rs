extern crate cgmath;

use cgmath::{vec4, Vector3, Vector4};
use std::hint::black_box;
use std::time::Instant;

const PIXELS: usize = 4096;
const PASSES: usize = 128;
const SAMPLES: usize = 12;

#[inline(never)]
fn measure<S: Copy>(
    name: &str,
    pixels: &[Vector4<S>],
    channels: &[u8],
    remove: impl Fn(&Vector4<S>, u8) -> Vector3<S>,
    mut output: Vec<Vector3<S>>,
) {
    for _ in 0..3 {
        for ((pixel, &channel), dst) in pixels.iter().zip(channels).zip(&mut output) {
            *dst = remove(pixel, channel);
        }
        black_box(&output);
    }
    for _ in 0..SAMPLES {
        let start = Instant::now();
        for _ in 0..PASSES {
            for ((pixel, &channel), dst) in pixels.iter().zip(channels).zip(&mut output) {
                *dst = remove(pixel, channel);
            }
            black_box(&output);
        }
        let ns_per_pixel = start.elapsed().as_secs_f64() * 1e9 / (PASSES * PIXELS) as f64;
        println!("{}, {:.6}", name, ns_per_pixel);
    }
}

fn main() {
    let mut seed = 0x4c957f2du32;
    let mut random = || {
        seed ^= seed << 13;
        seed ^= seed >> 17;
        seed ^= seed << 5;
        seed
    };
    let pixels: Vec<_> = (0..PIXELS)
        .map(|_| {
            vec4(
                random() as u8,
                random() as u8,
                random() as u8,
                random() as u8,
            )
        })
        .collect();
    let floats: Vec<_> = pixels
        .iter()
        .map(|pixel| pixel.map(|channel| f32::from(channel) / 255.0))
        .collect();
    let mixed: Vec<_> = (0..PIXELS).map(|_| (random() & 3) as u8).collect();
    let mostly_alpha: Vec<_> = mixed
        .iter()
        .map(|&channel| if random() & 7 == 0 { channel } else { 3 })
        .collect();
    let alpha = vec![3; PIXELS];

    for &(name, channels) in &[
        ("f32_mixed", mixed.as_slice()),
        ("f32_mostly_alpha", mostly_alpha.as_slice()),
        ("f32_runtime_alpha", alpha.as_slice()),
    ] {
        measure(
            name,
            &floats,
            channels,
            |pixel, channel| pixel.truncate_n(channel as isize),
            vec![Vector3::new(0.0, 0.0, 0.0); PIXELS],
        );
    }
    measure(
        "f32_fixed_alpha",
        &floats,
        &mixed,
        |pixel, _| pixel.truncate_n(3),
        vec![Vector3::new(0.0, 0.0, 0.0); PIXELS],
    );
    measure(
        "u8_mixed",
        &pixels,
        &mixed,
        |pixel, channel| pixel.truncate_n(channel as isize),
        vec![Vector3::new(0, 0, 0); PIXELS],
    );
}
