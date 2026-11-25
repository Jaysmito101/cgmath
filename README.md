# cgmath-rs

[![Build Status](https://github.com/rustgd/cgmath/workflows/tests/badge.svg)](https://github.com/rustgd/cgmath/actions)
[![Documentation](https://docs.rs/cgmath/badge.svg)](https://docs.rs/cgmath)
[![Version](https://img.shields.io/crates/v/cgmath.svg)](https://crates.io/crates/cgmath)
[![License](https://img.shields.io/crates/l/cgmath.svg)](https://github.com/rustgd/cgmath/blob/master/LICENSE)
[![Downloads](https://img.shields.io/crates/d/cgmath.svg)](https://crates.io/crates/cgmath)
[![Gitter](https://badges.gitter.im/brendanzab/cgmath.svg)](https://gitter.im/brendanzab/cgmath)

A linear algebra and mathematics library for computer graphics.

The library provides:

- vectors: `Vector2`, `Vector3`, `Vector4`
- square matrices: `Matrix2`, `Matrix3`, `Matrix4`
- a quaternion type: `Quaternion`
- rotation matrices: `Basis2`, `Basis3`
- angle units: `Rad`, `Deg`
- points: `Point2`, `Point3`
- perspective projections: `Perspective`, `PerspectiveFov`, `Ortho`, `PlanarFov`
- spatial transformations: `AffineMatrix3`, `Transform3`

Not all of the functionality has been implemented yet, and the existing code
is not fully covered by the testsuite. If you encounter any mistakes or
omissions please let me know by posting an issue, or even better: send me a
pull request with a fix.

## Conventions

cgmath interprets its vectors as column matrices (also known as "column
vectors"), meaning when transforming a vector with a matrix, the matrix goes
on the left. This is reflected in the fact that cgmath implements the
multiplication operator for Matrix * Vector, but not Vector * Matrix.

## Features

### Swizzling
This library offers an optional feature called
["swizzling"](https://en.wikipedia.org/wiki/Swizzling_(computer_graphics))
widely familiar to GPU programmers. To enable swizzle operators, pass the
`--features="swizzle"` option to cargo. Enabling this feature will increase
the size of the cgmath library by approximately 0.6MB. This isn't an
issue if the library is linked in the "normal" way by adding cgmath as a
dependency in Cargo.toml, which will link cgmath statically so all unused
swizzle operators will be optimized away by the compiler in release mode.

#### Example
If we have
```rust
let v = Vector3::new(1.0, 2.0, 3.0);
```
then `v.xyxz()` produces a
```rust
Vector4 { x: 1.0, y: 2.0, z: 1.0, w: 3.0 }
```
and `v.zy()` produces a
```rust
Vector2 { x: 3.0, y: 2.0 }
```
### SIMD optimizations

Enable the optional `simd` feature with `--features simd`. It uses
[`wide`](https://docs.rs/wide/1.7.1/wide/) for `Matrix4<f32> * Vector4<f32>`
and the `Vector4<f32>` methods `sqrt_element_wide`, `recip_element_wide`,
and `rsqrt_element_wide`. The feature builds on stable Rust 1.89 or newer.
`wide` selects SIMD at compile time and supplies a scalar fallback.

Run `cargo bench --bench simd` and `cargo bench --bench simd --features simd`
to compare two RGBA image-processing kernels on stable Rust.

## Limitations

cgmath is _not_ an n-dimensional library and is aimed at computer graphics
applications rather than general linear algebra. It only offers the 2, 3, and
4 dimensional structures that are more than enough for most computer graphics
applications. This design decision was made in order to simplify the
implementation (Rust cannot parameterize over constants at compile time), and to
make dimension-specific optimisations easier in the future.

## Contributing

Pull requests are most welcome, especially in the realm of performance
enhancements and fixing any mistakes I may have made along the way. Unit tests
and benchmarks are also required, so help on that front would be most
appreciated.
