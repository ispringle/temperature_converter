pub fn f_to_c(f: f32) -> f32 {
    (f - 32.0) * 0.55
}

pub fn f_to_k(f: f32) -> f32 {
    (f + 459.67) * 0.55
}

pub fn c_to_f(c: f32) -> f32 {
    (c * 1.8) + 32.0
}

pub fn c_to_k(c: f32) -> f32 {
    c + 273.15
}

pub fn k_to_c(k: f32) -> f32 {
    k - 273.15
}

pub fn k_to_f(k: f32) -> f32 {
    0.55 * (k - 273.0) + 32.0
}
