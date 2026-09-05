// src/anim.rs
//
// Small generic helpers shared by every animated widget: a `Lerp`
// trait so any widget's style struct can blend itself field-by-field,
// plus a frame-rate independent smoothing factor.

use macroquad::prelude::*;

/// Types that know how to interpolate between two values of themselves.
///
/// Implemented for the primitive types widgets are built out of
/// (`f32`, `Color`, `Vec2`), and typically implemented for style
/// structs by delegating to each field's own `lerp`.
// Named `lerp_towards` rather than `lerp` so it never collides with
// glam's own inherent `Vec2::lerp` (inherent methods always win over
// trait methods in method-call resolution, which would otherwise
// silently call the wrong implementation).
pub trait Lerp {
    fn lerp_towards(&self, other: &Self, t: f32) -> Self;
}

impl Lerp for f32 {
    fn lerp_towards(&self, other: &Self, t: f32) -> Self {
        self + (other - self) * t
    }
}

impl Lerp for Color {
    fn lerp_towards(&self, other: &Self, t: f32) -> Self {
        Color {
            r: self.r.lerp_towards(&other.r, t),
            g: self.g.lerp_towards(&other.g, t),
            b: self.b.lerp_towards(&other.b, t),
            a: self.a.lerp_towards(&other.a, t),
        }
    }
}

impl Lerp for Vec2 {
    fn lerp_towards(&self, other: &Self, t: f32) -> Self {
        vec2(self.x.lerp_towards(&other.x, t), self.y.lerp_towards(&other.y, t))
    }
}

/// Turns a per-second "speed" and a frame delta into a frame-rate
/// independent interpolation factor via exponential smoothing.
///
/// `speed <= 0.0` snaps instantly (`t = 1.0`), matching the old
/// behaviour widgets used before this was factored out.
pub fn smoothing_factor(speed: f32, dt: f32) -> f32 {
    if speed <= 0.0 {
        1.0
    } else {
        1.0 - (-speed * dt).exp()
    }
}
