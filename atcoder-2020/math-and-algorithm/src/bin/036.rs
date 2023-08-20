use std::f64::consts::PI;

use proconio::input;

pub mod vec2f {
    use std::ops::{Add, Sub};

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct Vec2f {
        x: f64,
        y: f64,
    }

    impl Add for Vec2f {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            Self {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    impl Sub for Vec2f {
        type Output = Self;

        fn sub(self, other: Self) -> Self {
            Self {
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }

    impl Vec2f {
        pub fn new(x: f64, y: f64) -> Vec2f {
            Vec2f { x, y }
        }

        pub fn inner_product(&self, other: &Self) -> f64 {
            self.x * other.x + self.y * other.y
        }

        pub fn outer_product(&self, other: &Self) -> f64 {
            self.x * other.y - self.y * other.x
        }

        pub fn len(&self) -> f64 {
            (self.x * self.x + self.y * self.y).sqrt()
        }

        pub fn scaled(&self, scale: f64) -> Vec2f {
            Vec2f {
                x: self.x * scale,
                y: self.y * scale,
            }
        }

        pub fn normalized(&self) -> Vec2f {
            let len = self.len();
            Vec2f {
                x: self.x / len,
                y: self.y / len,
            }
        }
    }
}

use vec2f::Vec2f;

fn main() {
    input! {
        a: usize,
        b: usize,
        h: usize,
        m: usize,
    }
    let rad0 = PI * (h * 60 + m) as f64 / 360.0;
    let pos0 = Vec2f::new(rad0.cos(), rad0.sin()).scaled(a as f64);
    let rad1 = PI * m as f64 / 30.0;
    let pos1 = Vec2f::new(rad1.cos(), rad1.sin()).scaled(b as f64);
    let result = (pos1 - pos0).len();
    println!("{}", result);
}
