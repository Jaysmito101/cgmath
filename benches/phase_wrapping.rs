// Copyright 2026 The CGMath Developers. For a full listing of the authors,
// refer to the Cargo.toml file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate cgmath;

use cgmath::{Angle, Deg, Rad};
use std::hint::black_box;
use std::time::Instant;

const PIXELS: usize = 4096;
const PASSES: usize = 24;
const SAMPLES: usize = 12;

fn measure<A: Angle>(name: &str, input: &[A], wrap: impl Fn(A) -> A) {
    let mut output = input.to_vec();
    let mut sample = || {
        let start = Instant::now();
        for _ in 0..PASSES {
            for (&hue, dst) in black_box(input).iter().zip(output.iter_mut()) {
                *dst = wrap(hue);
            }
            black_box(output.as_slice());
        }
        start.elapsed().as_secs_f64() * 1e9 / (PASSES * input.len()) as f64
    };

    black_box(sample());
    for _ in 0..SAMPLES {
        println!("{name},{:.6}", sample());
    }
}

fn main() {
    let hues: Vec<_> = (0..PIXELS)
        .map(|i| ((i * 2549 % PIXELS) as f32 + 0.5) * (360.0 / PIXELS as f32))
        .collect();
    let phase_f32: Vec<_> = hues
        .iter()
        .map(|&hue| Rad((hue - 180.0).to_radians()))
        .collect();
    let phase_f64: Vec<_> = hues
        .iter()
        .map(|&hue| Rad(f64::from(hue - 180.0).to_radians()))
        .collect();
    let shifted_hue: Vec<_> = hues.iter().map(|&hue| Deg(hue + 90.0)).collect();
    let multi_turn: Vec<_> = phase_f32
        .iter()
        .enumerate()
        .map(|(i, &phase)| phase + Rad(if i % 2 == 0 { 37.0 } else { -37.0 }))
        .collect();
    let mixed: Vec<_> = phase_f32
        .iter()
        .zip(&multi_turn)
        .enumerate()
        .map(|(i, (&phase, &large))| if i % 8 < 2 { large } else { phase })
        .collect();

    measure("signed_phase_f32", &phase_f32, Angle::normalize_signed);
    measure("signed_phase_f64", &phase_f64, Angle::normalize_signed);
    measure("shifted_hue_f32", &shifted_hue, Angle::normalize_signed);
    measure("mixed_phase_f32", &mixed, Angle::normalize_signed);
    measure("multi_turn_f32", &multi_turn, Angle::normalize_signed);
}
