// Copyright The CGMath Developers. This file is licensed under the Apache
// License, Version 2.0. See LICENSE for details.

extern crate cgmath;

use cgmath::{vec4, Vector4};
use std::hint::black_box;
use std::time::Instant;

fn bench(name: &str, pixels: &[Vector4<f32>]) {
    const PASSES: usize = 128;
    let mut output = vec![None::<Vector4<u8>>; pixels.len()];
    let mut samples = [0.0; 15];
    for sample in &mut samples {
        let start = Instant::now();
        for _ in 0..PASSES {
            for (pixel, dst) in black_box(pixels).iter().zip(output.iter_mut()) {
                *dst = pixel.cast();
            }
            black_box(&output);
        }
        *sample = start.elapsed().as_nanos() as f64 / (pixels.len() * PASSES) as f64;
    }
    samples.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("{} {:.6}", name, samples[samples.len() / 2]);
}

fn main() {
    let mut state = 16u32;
    let mut channel = || {
        state = state.wrapping_mul(1664525).wrapping_add(1013904223);
        (state >> 16) as f32 / 256.0
    };
    let mut pixels: Vec<_> = (0..4096)
        .map(|_| vec4(channel(), channel(), channel(), channel()))
        .collect();
    bench("rgba8_valid", &pixels);

    for (i, pixel) in pixels.iter_mut().step_by(8).enumerate() {
        pixel[i % 4] = [-1.0, f32::NAN, 256.0, f32::INFINITY][(i / 4) % 4];
    }
    bench("rgba8_mixed", &pixels);
}
