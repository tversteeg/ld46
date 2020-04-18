pub fn range(min: f64, max: f64) -> f64 {
    let r = unsafe { miniquad::rand() as f64 / miniquad::RAND_MAX as f64 };

    r * (max - min) + min
}

pub fn bool() -> bool {
    let r = unsafe { miniquad::rand() };

    r > miniquad::RAND_MAX as i32 / 2
}
