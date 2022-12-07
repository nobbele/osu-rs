mod helper;
mod spline;

pub use spline::*;

fn difficulty_range(difficulty: f32, min: f32, mid: f32, max: f32) -> f32 {
    if difficulty > 5.0 {
        mid + (max - mid) * (difficulty - 5.0) / 5.0
    } else {
        mid - (mid - min) * (5.0 - difficulty) / 5.0
    }
}

pub fn ar_to_ms(ar: f32) -> f32 {
    difficulty_range(ar, 1800.0, 1200.0, 450.0)
}

/// Returns circle radius.
pub fn cs_to_px(cs: f32) -> f32 {
    54.4 - 4.48 * cs
}
