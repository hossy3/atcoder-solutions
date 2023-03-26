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

        pub fn len2(&self) -> f64 {
            self.x * self.x + self.y * self.y
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

fn f(v0: &Vec2f, v1: &Vec2f, v2: &Vec2f, v3: &Vec2f) -> bool {
    let n012 = (*v1 - *v0).outer_product(&(*v2 - *v1));
    let n013 = (*v1 - *v0).outer_product(&(*v3 - *v1));
    let n230 = (*v3 - *v2).outer_product(&(*v0 - *v3));
    let n231 = (*v3 - *v2).outer_product(&(*v1 - *v3));
    if n012 == 0.0 && n013 == 0.0 && n230 == 0.0 && n231 == 0.0 {
        let u = (*v1 - *v0).normalized();
        let k1 = (*v1 - *v0).len();
        let k2 = u.inner_product(&(*v2 - *v0));
        let k3 = u.inner_product(&(*v3 - *v0));
        if (0.0 <= k2 && k2 <= k1)
            || (0.0 <= k3 && k3 <= k1)
            || (k2 < 0.0 && k1 < k3)
            || (k3 < 0.0 && k1 < k2)
        {
            return true; // v01 and v23 is on same infinite-line
        }
        return false; // parallel
    }

    (n012 * n013 <= 0.0) && (n230 * n231 <= 0.0)
}

#[test]
fn test_func() {
    assert_eq!(
        f(
            &Vec2f::new(0.0, 0.0),
            &Vec2f::new(2000.0, 3000.0),
            &Vec2f::new(192.0, 1.0),
            &Vec2f::new(1500.0, 2250.0)
        ),
        true
    );
}

fn main() {
    input! {
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
        x3: usize,
        y3: usize,
        x4: usize,
        y4: usize,
    }
    let yes = f(
        &Vec2f::new(x1 as f64, y1 as f64),
        &Vec2f::new(x2 as f64, y2 as f64),
        &Vec2f::new(x3 as f64, y3 as f64),
        &Vec2f::new(x4 as f64, y4 as f64),
    );
    println!("{}", if yes { "Yes" } else { "No" });
}
