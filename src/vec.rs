#[derive(Clone, Copy, Debug)]
pub struct Vec {
    pub x: f32,
    pub y: f32,
}

impl Vec {
    #[allow(dead_code)]
    pub fn new() -> Vec {
        Vec { x: 0., y: 0. }
    }
    pub fn from_tuple(tuple: &(f32, f32)) -> Self {
        Vec {
            x: tuple.0,
            y: tuple.1,
        }
    }
    pub fn as_tuple(&self) -> (f32, f32) {
        (self.x, self.y)
    }
    #[allow(dead_code)]
    pub fn new_from(x: f32, y: f32) -> Vec {
        Vec { x, y }
    }
    #[allow(dead_code)]
    pub fn add(&mut self, v: &Vec) -> &mut Vec {
        self.x += v.x;
        self.y += v.y;
        self
    }
    #[allow(dead_code)]
    pub fn sub(&mut self, v: &Vec) -> &mut Vec {
        self.x -= v.x;
        self.y -= v.y;
        self
    }
    #[allow(dead_code)]
    pub fn mul(&mut self, v: f32) -> &mut Vec {
        self.x *= v;
        self.y *= v;
        self
    }
    #[allow(dead_code)]
    pub fn div(&mut self, v: f32) -> &mut Vec {
        self.x /= v;
        self.y /= v;
        self
    }
    #[allow(dead_code)]
    pub fn limit(&mut self, v: f32) -> &mut Vec {
        let mag = self.mag();
        if mag > v {
            self.mul(v / mag);
        }
        self
    }
    #[allow(dead_code)]
    pub fn limit_min(&mut self, v: f32) -> &mut Vec {
        let mag = self.mag();
        if mag < v {
            self.mul(v / mag);
        }
        self
    }
    #[allow(dead_code)]
    pub fn norm(&mut self, v: f32) -> &mut Vec {
        let mag = self.mag();
        if mag > 0.0 {
            self.mul(v / mag);
        }
        self
    }
    #[allow(dead_code)]
    pub fn mag(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
    #[allow(dead_code)]
    pub fn dist(&self, v: &Vec) -> f32 {
        ((self.x - v.x).powi(2) + (self.y - v.y).powi(2)).sqrt()
    }
    #[allow(dead_code)]
    pub fn squared_dist(&self, v: &Vec) -> f32 {
        ((self.x - v.x).powi(2) + (self.y - v.y).powi(2))
    }
    #[allow(dead_code)]
    pub fn square_dist(&self, v: &Vec) -> f32 {
        (self.x - v.x).abs() + (self.y - v.y).abs()
    }
    #[allow(dead_code)]
    pub fn dist_mod(&self, v: &Vec, w: f32, h: f32) -> f32 {
        self.dist(&self.rel(&v, w, h))
    }
    #[allow(dead_code)]
    pub fn rel(&self, v: &Vec, w: f32, h: f32) -> Vec {
        let x = if (self.x - v.x).abs() < w / 2.0 {
            v.x
        } else {
            v.x + if v.x < w / 2.0 { w } else { -w }
        };
        let y = if (self.y - v.y).abs() < h / 2.0 {
            v.y
        } else {
            v.y + if v.y < h / 2.0 { h } else { -h }
        };

        Vec { x, y }
    }
}
