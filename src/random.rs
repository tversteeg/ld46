pub fn range(min: f64, max: f64) -> f64 {
    let r = quad_rand::rand() as f64 / u32::MAX as f64;

    r * (max - min) + min
}

pub fn usize(max: usize) -> usize {
    let r = quad_rand::rand() as usize;

    (r / (u32::MAX as usize / max)).min(max - 1)
}

pub fn bool() -> bool {
    let r = quad_rand::rand();

    r > u32::MAX / 2
}

pub fn index<T>(v: &Vec<T>) -> &T {
    let len = v.len();
    let r = quad_rand::rand() as usize / (u32::MAX as usize / len);

    &v[r.min(len - 1)]
}
