extern crate cgmath;

use cgmath::{BaseFloat, Matrix4, One, Point3, SquareMatrix, Transform};
use std::hint::black_box;
use std::time::Instant;

const BATCH: usize = 4096;
const SAMPLES: usize = 30;

fn warps<S: BaseFloat>() -> Vec<Matrix4<S>> {
    (0..BATCH)
        .map(|i| {
            let x = (i % 64) as f64 / 64.0;
            let y = (i / 64) as f64 / 64.0;
            Matrix4::from([
                [0.9 + 0.2 * x, 0.12 * y, -0.04, 0.07 * x],
                [-0.08 * x, 1.1 - 0.25 * y, 0.06 * x, -0.03 * y],
                [0.06 * y, -0.09 * x, 0.95 + 0.04 * y, 0.09],
                [0.2 * x, -0.3 * y, 0.05 * (x - y), 1.0],
            ])
            .cast()
            .unwrap()
        })
        .collect()
}

fn measure<S, T, F>(name: &str, input: &[Matrix4<S>], mut output: Vec<T>, run: F)
where
    F: Fn(&Matrix4<S>) -> T,
{
    let mut samples = [0.0; SAMPLES];
    for sample in &mut samples {
        let start = Instant::now();
        for (matrix, result) in black_box(input).iter().zip(&mut output) {
            *result = run(matrix);
        }
        black_box(&output);
        *sample = start.elapsed().as_secs_f64() * 1e9 / input.len() as f64;
    }
    for sample in &samples {
        println!("{},{:.6}", name, sample);
    }
}

fn main() {
    let f32_warps = warps::<f32>();
    let f64_warps = warps::<f64>();
    measure("invert_f32", &f32_warps, vec![Matrix4::one(); BATCH], |m| {
        m.invert().unwrap()
    });
    measure("invert_f64", &f64_warps, vec![Matrix4::one(); BATCH], |m| {
        m.invert().unwrap()
    });

    let corners = [
        Point3::new(-0.5, -0.5, 0.0),
        Point3::new(0.5, -0.5, 0.0),
        Point3::new(-0.5, 0.5, 0.0),
        Point3::new(0.5, 0.5, 0.0),
    ];
    measure("tile_corners_f32", &f32_warps, vec![corners; BATCH], |m| {
        let inverse = m.inverse_transform().unwrap();
        corners.map(|corner| inverse.transform_point(corner))
    });
}
