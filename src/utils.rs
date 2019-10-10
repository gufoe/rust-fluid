
#[allow(dead_code)]
pub fn rand_int(max: u32) -> u32 {
    use rand::Rng;
    rand::thread_rng().gen_range(0, max)
}
#[allow(dead_code)]
pub fn rand_usize(max: usize) -> usize {
    use rand::Rng;
    rand::thread_rng().gen_range(0, max)
}
#[allow(dead_code)]
pub fn rand_intr(min: i32, max: i32) -> i32 {
    use rand::Rng;
    rand::thread_rng().gen_range(min, max)
}
#[allow(dead_code)]
pub fn rand_float(min: f32, max: f32) -> f32 {
    use rand::Rng;
    rand::thread_rng().gen_range(min, max)
}
#[allow(dead_code)]
pub fn maybe(pty: f32) -> bool{
    rand_float(0., 1.) < pty
}

#[allow(dead_code)]
pub fn now() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let ste = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
    ste.as_secs() as f64 + ste.subsec_micros() as f64 / 1_000_000.0
}


pub fn avg(numbers: &[f32]) -> f32 {
    numbers.iter().sum::<f32>() / numbers.len() as f32
}
