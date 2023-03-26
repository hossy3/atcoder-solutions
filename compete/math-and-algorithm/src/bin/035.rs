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

fn f(p0: &Vec2f, r0: f64, p1: &Vec2f, r1: f64) -> usize {
    let dist = (*p0 - *p1).len();
    if dist < (r0 - r1).abs()  {
        1
    } else if dist == (r0 - r1).abs() {
        2
    } else if dist < r0 + r1 {
        3
    } else if dist == r0 + r1 {
        4
    } else {
        5
    }
}

fn main() {
    input! {
        x1: f64,
        y1: f64,
        r1: f64,
        x2: f64,
        y2: f64,
        r2: f64,
    }

    let p1 = Vec2f::new(x1, y1);
    let p2 = Vec2f::new(x2, y2);
    let result = f(&p1, r1, &p2, r2);
    println!("{}", result);
}
