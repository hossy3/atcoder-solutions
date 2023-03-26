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
        axy: (f64, f64),
        bxy: (f64, f64),
        cxy: (f64, f64),
    }

    let a = Vec2f::new(axy.0, axy.1);
    let b = Vec2f::new(bxy.0, bxy.1);
    let c = Vec2f::new(cxy.0, cxy.1);
    let ba = a - b;
    let bc = c - b;
    let scale = ba.inner_product(&bc) / (bc.len() * bc.len());
    let result = if scale <= 0.0 {
        ba.len()
    } else if scale >= 1.0 {
        (c - a).len()
    } else {
        (a - (b + bc.scaled(scale))).len()
    };
    println!("{}", result);
}
