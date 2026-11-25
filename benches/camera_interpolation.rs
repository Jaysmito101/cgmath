extern crate cgmath;

use cgmath::{BaseFloat, Quaternion, Rad, Rotation, Rotation3, Vector3};
use std::hint::black_box;
use std::time::Instant;

fn measure<S: BaseFloat, T: Copy>(
    name: &str,
    frames: &[(Quaternion<S>, Quaternion<S>)],
    initial: T,
    mut interpolate: impl FnMut(Quaternion<S>, Quaternion<S>) -> T,
) {
    const SAMPLES: usize = 30;
    const PASSES: usize = 32;
    let mut output = vec![initial; frames.len()];

    // Discard the first sample so the same loop also warms the input and output buffers.
    for sample in 0..=SAMPLES {
        let start = Instant::now();
        for _ in 0..PASSES {
            for (dst, &(a, b)) in output.iter_mut().zip(black_box(frames)) {
                *dst = interpolate(a, b);
            }
            black_box(output.as_slice());
        }
        if sample != 0 {
            let ns_per_pair = start.elapsed().as_secs_f64() * 1e9 / (PASSES * frames.len()) as f64;
            println!("{} {:.6}", name, ns_per_pair);
        }
    }
}

fn frames<S: BaseFloat>(scale: S) -> Vec<(Quaternion<S>, Quaternion<S>)> {
    (0..4096)
        .map(|i| {
            let base = Quaternion::from_angle_y(Rad(S::from(i).unwrap() / S::from(4096).unwrap()));
            let angle = scale * S::from(i % 51).unwrap() / S::from(50).unwrap();
            let next = base * Quaternion::from_axis_angle(Vector3::unit_x(), Rad(angle));
            (base, if i % 2 == 0 { next } else { -next })
        })
        .collect()
}

fn main() {
    println!("case ns/pair");
    let close = frames(0.04_f32);
    measure("close_f32", &close, close[0].0, |a, b| a.slerp(b, 0.35));
    let close_f64 = frames(0.04_f64);
    measure("close_f64", &close_f64, close_f64[0].0, |a, b| {
        a.slerp(b, 0.35)
    });
    let wide = frames(1.0_f32);
    measure("wide_f32", &wide, wide[0].0, |a, b| a.slerp(b, 0.35));
    let wide_f64 = frames(1.0_f64);
    measure("wide_f64", &wide_f64, wide_f64[0].0, |a, b| {
        a.slerp(b, 0.35)
    });

    // Rotate a representative pixel ray with the interpolated camera orientation.
    let ray = Vector3::new(0.25, -0.4, 1.0);
    measure("pixel_ray_f32", &close, ray, |a, b| {
        a.slerp(b, 0.35).rotate_vector(ray)
    });
}
