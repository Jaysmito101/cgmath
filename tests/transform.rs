// Copyright 2014 The CGMath Developers. For a full listing of the authors,
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

extern crate approx;
extern crate cgmath;

#[cfg(feature = "serde")]
extern crate serde_json;

use cgmath::*;

#[test]
fn test_mul() {
    let t1 = Decomposed {
        scale: 2.0f64,
        rot: Quaternion::new(0.5f64.sqrt(), 0.5f64.sqrt(), 0.0, 0.0),
        disp: Vector3::new(1.0f64, 2.0, 3.0),
    };
    let t2 = Decomposed {
        scale: 3.0f64,
        rot: Quaternion::new(0.5f64.sqrt(), 0.0, 0.5f64.sqrt(), 0.0),
        disp: Vector3::new(-2.0, 1.0, 0.0),
    };

    let actual = t1 * t2;

    let expected = Decomposed {
        scale: 6.0f64,
        rot: Quaternion::new(0.5, 0.5, 0.5, 0.5),
        disp: Vector3::new(-3.0, 2.0, 5.0),
    };

    assert_ulps_eq!(actual, expected);
}

#[test]
fn test_mul_one() {
    let t = Decomposed {
        scale: 2.0f64,
        rot: Quaternion::new(0.5f64.sqrt(), 0.5f64.sqrt(), 0.0, 0.0),
        disp: Vector3::new(1.0f64, 2.0, 3.0),
    };
    let one = Decomposed::one();

    assert_ulps_eq!(t * one, t);
    assert_ulps_eq!(one * t, t);
}

#[test]
fn test_invert() {
    let v = Vector3::new(1.0f64, 2.0, 3.0);
    let t = Decomposed {
        scale: 1.5f64,
        rot: Quaternion::new(0.5f64, 0.5, 0.5, 0.5),
        disp: Vector3::new(6.0f64, -7.0, 8.0),
    };
    let ti = t
        .inverse_transform()
        .expect("Expected successful inversion");
    let vt = t.transform_vector(v);
    assert_ulps_eq!(&v, &ti.transform_vector(vt));
}

#[test]
fn test_inverse_vector() {
    let v = Vector3::new(1.0f64, 2.0, 3.0);
    let t = Decomposed {
        scale: 1.5f64,
        rot: Quaternion::new(0.5f64, 0.5, 0.5, 0.5),
        disp: Vector3::new(6.0f64, -7.0, 8.0),
    };
    let vt = t
        .inverse_transform_vector(v)
        .expect("Expected successful inversion");
    assert_ulps_eq!(v, t.transform_vector(vt));
}

#[test]
fn matrix4_inverse_preserves_cofactor_arithmetic() {
    fn reference<S: BaseFloat>(matrix: Matrix4<S>) -> Option<Matrix4<S>> {
        let determinant = matrix.determinant();
        if determinant == S::zero() {
            return None;
        }
        let inverse_determinant = S::one() / determinant;
        let r = matrix.transpose();
        let rows = [
            (r.y, r.z, r.w),
            (r.x, r.z, r.w),
            (r.x, r.y, r.w),
            (r.x, r.y, r.z),
        ];
        Some(Matrix4::from(std::array::from_fn(|i| {
            let (a, b, c) = rows[i];
            std::array::from_fn(|j| {
                let sign = [S::one(), -S::one()][(i + j) % 2];
                let j = j as isize;
                let minor = Matrix3::from_cols(a.truncate_n(j), b.truncate_n(j), c.truncate_n(j));
                minor.determinant() * sign * inverse_determinant
            })
        })))
    }

    macro_rules! check {
        ($scalar:ty) => {{
            let dense = (0..128).map(|i| {
                Matrix4::from(std::array::from_fn(|col| {
                    std::array::from_fn(|row| {
                        let n = (i * 16 + col * 4 + row + 1) as i64;
                        ((n * n * 1237 % 4999) as $scalar - 2500.0) / 301.0
                    })
                }))
            });
            let special = [
                0.0,
                -0.0,
                <$scalar>::MIN_POSITIVE,
                <$scalar>::INFINITY,
                <$scalar>::NAN,
            ];
            let diagonal = special
                .iter()
                .map(|&x| Matrix4::from_nonuniform_scale(x, -2.0, 1.0));
            for matrix in dense.chain(diagonal).chain(std::iter::once(Matrix4::one())) {
                match (matrix.inverse_transform(), reference(matrix)) {
                    (Some(actual), Some(expected)) => {
                        let actual: [[$scalar; 4]; 4] = actual.into();
                        let expected: [[$scalar; 4]; 4] = expected.into();
                        for (&a, &b) in actual.iter().flatten().zip(expected.iter().flatten()) {
                            assert!(a.to_bits() == b.to_bits() || (a.is_nan() && b.is_nan()));
                        }
                    }
                    (None, None) => {}
                    pair => panic!("inverse results disagree for {:?}: {:?}", matrix, pair),
                }
            }
        }};
    }
    check!(f32);
    check!(f64);
}

#[test]
#[allow(deprecated)]
fn test_look_at() {
    let eye = Point3::new(0.0f64, 0.0, -5.0);
    let center = Point3::new(0.0f64, 0.0, 0.0);
    let up = Vector3::new(1.0f64, 0.0, 0.0);
    let t: Decomposed<Vector3<f64>, Quaternion<f64>> = Transform::look_at(eye, center, up);
    assert_ulps_eq!(
        t,
        Decomposed::<Vector3<f64>, Quaternion<f64>>::look_at(eye, center, up)
    );
    let point = Point3::new(1.0f64, 0.0, 0.0);
    let view_point = Point3::new(0.0f64, 1.0, 5.0);
    assert_ulps_eq!(&t.transform_point(point), &view_point);
}

#[test]
fn test_look_at_lh() {
    let eye = Point3::new(0.0f64, 0.0, -5.0);
    let center = Point3::new(0.0f64, 0.0, 0.0);
    let up = Vector3::new(1.0f64, 0.0, 0.0);
    let t: Decomposed<Vector3<f64>, Quaternion<f64>> = Transform::look_at_lh(eye, center, up);
    assert_ulps_eq!(
        t,
        Decomposed::<Vector3<f64>, Quaternion<f64>>::look_at_lh(eye, center, up)
    );
    let point = Point3::new(1.0f64, 0.0, 0.0);
    let view_point = Point3::new(0.0f64, 1.0, 5.0);
    assert_ulps_eq!(&t.transform_point(point), &view_point);

    // Decomposed::look_at_lh and Matrix4::look_at_lh should be consistent
    let t: Matrix4<f64> = Transform::look_at_lh(eye, center, up);
    assert_ulps_eq!(t, Matrix4::<f64>::look_at_lh(eye, center, up));
    assert_ulps_eq!(&t.transform_point(point), &view_point);

    // Decomposed::look_at is inconsistent and deprecated, but verify that the behavior
    // remains the same until removed.
    #[allow(deprecated)]
    let t: Decomposed<Vector3<f64>, Quaternion<f64>> = Transform::look_at(eye, center, up);
    assert_ulps_eq!(&t.transform_point(point), &view_point);
}

#[test]
fn test_look_at_rh() {
    let eye = Point3::new(0.0f64, 0.0, -5.0);
    let center = Point3::new(0.0f64, 0.0, 0.0);
    let up = Vector3::new(1.0f64, 0.0, 0.0);
    let t: Decomposed<Vector3<f64>, Quaternion<f64>> = Transform::look_at_rh(eye, center, up);
    assert_ulps_eq!(
        t,
        Decomposed::<Vector3<f64>, Quaternion<f64>>::look_at_rh(eye, center, up)
    );
    let point = Point3::new(1.0f64, 0.0, 0.0);
    let view_point = Point3::new(0.0f64, 1.0, -5.0);
    assert_ulps_eq!(&t.transform_point(point), &view_point);

    // Decomposed::look_at_rh and Matrix4::look_at_rh should be consistent
    let t: Matrix4<f64> = Transform::look_at_rh(eye, center, up);
    assert_ulps_eq!(t, Matrix4::<f64>::look_at_rh(eye, center, up));
    assert_ulps_eq!(&t.transform_point(point), &view_point);
}

#[cfg(feature = "serde")]
#[test]
fn test_serialize() {
    let t = Decomposed {
        scale: 1.5f64,
        rot: Quaternion::new(0.5f64, 0.5, 0.5, 0.5),
        disp: Vector3::new(6.0f64, -7.0, 8.0),
    };

    let serialized = serde_json::to_string(&t).unwrap();
    let deserialized: Decomposed<Vector3<f64>, Quaternion<f64>> =
        serde_json::from_str(&serialized).unwrap();

    assert_ulps_eq!(&t, &deserialized);
}
