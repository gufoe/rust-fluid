#[allow(dead_code)]
pub struct Timer {
    times: Vec<(String, f64)>,
}

#[allow(dead_code)]
impl Timer {
    pub fn new(label: &str) -> Self {
        let mut x = Self { times: vec![] };
        x.tick(label);
        x
    }
    pub fn clear(&mut self) {
        self.times.clear();
    }
    pub fn tick(&mut self, label: &str) -> (String, f64) {
        self.times.push((label.to_string(), now()));
        self.times
            .get(self.times.len() - 2)
            .unwrap_or(&("start".to_string(), 0.0))
            .clone()
    }
    pub fn diff_or_0(&self, a: usize, b: usize) -> f64 {
        let a = self.times.get(a).unwrap_or(&("start".to_string(), 0.0)).1;
        let b = self.times.get(b).unwrap_or(&("start".to_string(), a)).1;
        a - b
    }
    pub fn show(&self) {
        for i in 0..self.times.len() {
            println!("{:>30}: {:.9}", self.times[i].0, self.diff_or_0(i, i - 1));
        }
        println!(
            "{:>30}: {:.9}",
            "total",
            self.diff_or_0(self.times.len() - 1, 0)
        );
    }
}

#[allow(dead_code)]
pub fn rand_int(max: u32) -> u32 {
    use rand::Rng;
    rand::thread_rng().gen_range(0..max)
}
#[allow(dead_code)]
pub fn rand_usize(max: usize) -> usize {
    use rand::Rng;
    rand::thread_rng().gen_range(0..max)
}
#[allow(dead_code)]
pub fn rand_intr(min: i32, max: i32) -> i32 {
    use rand::Rng;
    rand::thread_rng().gen_range(min..max)
}
#[allow(dead_code)]
pub fn rand_float(min: f32, max: f32) -> f32 {
    use rand::Rng;
    rand::thread_rng().gen_range(min..max)
}
#[allow(dead_code)]
pub fn maybe(pty: f32) -> bool {
    rand_float(0., 1.) < pty
}

#[allow(dead_code)]
pub fn now() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let ste = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    ste.as_secs() as f64 + ste.subsec_micros() as f64 / 1_000_000.0
}

pub fn avg(numbers: &[f32]) -> f32 {
    numbers.iter().sum::<f32>() / numbers.len() as f32
}
pub fn vavg(a: &mut [f32], b: &[f32], x: f32) {
    for i in 0..a.len() {
        a[i] += b[i] * x;
        a[i] /= 1.0 + x;
    }
}
pub fn sum(a: &[f32; 3], b: &[f32; 3]) -> [f32; 3] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
pub fn eavg(a: &mut f32, b: f32, x: f32) {
    *a += b * x;
    *a /= 1.0 + x;
}

pub fn softmax(v: &mut [f32]) {
    norm(v);
    let sum = v.iter().fold(0.0, |sum, &val| sum + val);
    if sum > 0. {
        for n in v {
            *n /= sum
        }
    }
}
pub fn softmax_fast(v: &mut [f32]) {
    let sum = v.iter().fold(0.0, |sum, &val| sum + val);
    if sum > 0. {
        for n in v {
            *n /= sum
        }
    }
}

pub fn norm(v: &mut [f32]) {
    let mut max = -9999999999.0;
    let mut min = 9999999999.0;
    for i in v.iter() {
        if *i > max {
            max = *i;
        }
        if *i < min {
            min = *i;
        }
    }
    let mut n = max - min;
    if n == 0. {
        n = 1.;
    }
    for i in v.iter_mut() {
        *i -= min;
        *i /= n;
    }
}
pub fn norm_2(v: &mut [f32]) {
    let mut max = -9999999999.0;
    let mut min = 9999999999.0;
    for i in v.iter() {
        if *i > max {
            max = *i;
        }
        if *i < min {
            min = *i;
        }
    }
    let mut n = max - min;
    if n == 0. {
        n = 1.;
    }
    for i in v.iter_mut() {
        *i /= n;
    }
}
