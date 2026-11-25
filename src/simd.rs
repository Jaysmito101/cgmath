use try_specialize::Specialization;
use wide::f32x4;

use {BaseFloat, Matrix4, Vector4};

impl Vector4<f32> {
    /// Compute and return the square root of each element.
    #[inline]
    pub fn sqrt_element_wide(self) -> Self {
        f32x4::new(self.into()).sqrt().to_array().into()
    }

    /// Compute and return the approximate reciprocal of each element's square root.
    #[inline]
    pub fn rsqrt_element_wide(self) -> Self {
        f32x4::new(self.into()).recip_sqrt().to_array().into()
    }

    /// Compute and return the approximate reciprocal of each element.
    #[inline]
    pub fn recip_element_wide(self) -> Self {
        f32x4::new(self.into()).recip().to_array().into()
    }
}

#[inline]
pub(crate) fn mul_matrix4_vector4<S: BaseFloat>(
    matrix: &Matrix4<S>,
    vector: &Vector4<S>,
) -> Option<Vector4<S>> {
    let lanes = Specialization::<[S; 4], [f32; 4]>::try_new()?;
    let pack = |column: Vector4<S>| f32x4::new(lanes.specialize(column.into()));
    let [x, y, z, w] = lanes.specialize((*vector).into());
    // Match Vector4::dot's right-associated sum, without fused multiply-adds.
    let result =
        pack(matrix.x) * x + (pack(matrix.y) * y + (pack(matrix.z) * z + pack(matrix.w) * w));
    Some(lanes.rev().specialize(result.to_array()).into())
}
