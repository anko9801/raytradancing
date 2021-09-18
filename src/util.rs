use std::f64;
const RAND_MAX: usize = usize::MAX;

pub struct Rand {
    x: usize,
}

impl Rand {
    pub fn new() -> Rand {
        Rand {
            x: 88172645463325252,
        }
    }

    pub fn next_int(&mut self) -> usize {
        self.x = self.x ^ (self.x << 13);
        self.x = self.x ^ (self.x >> 7);
        self.x = self.x ^ (self.x << 17);
        self.x
    }

    pub fn next_double(&mut self) -> f64 {
        let rand = self.next_int() as f64;
        rand / (RAND_MAX as f64 + 1.0)
    }

    pub fn next_specified_double(&mut self, min: f64, max: f64) -> f64 {
        min + (max - min) * self.next_double()
    }
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * f64::consts::PI / 180.0;
}
