use ultraviolet::{self as uv, f32x8};

pub fn integrate_x8(pos: &mut [uv::Vec3x8], vel: &mut [uv::Vec3x8], acc: &[uv::Vec3x8], dt: f32x8) {
    for ((position, velocity), acceleration) in pos.iter_mut().zip(vel).zip(acc) {
        *velocity = *velocity + *acceleration * dt;
        *position = *position + *velocity * dt;
    }
}
