use std::f64::consts::PI;

pub fn sin(angle: f64) -> f64 {
    libm::sin(angle)
}

pub fn triangle(angle: f64) -> f64 {
    libm::asin(libm::sin(angle)) / (PI / 2.0)
}

pub fn square(angle: f64) -> f64 {
    if libm::sin(angle) >= 0.0 { 1.0 } else { -1.0 }
}

pub fn sawtooth(angle: f64) -> f64 {
    (angle / (2.0 * PI)).rem_euclid(2.0) - 1.0
}