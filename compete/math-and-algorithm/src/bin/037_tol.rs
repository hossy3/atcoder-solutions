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
    let v01 = *v1 - *v0;
    let v23 = *v3 - *v2;
    let v02 = *v2 - *v0;
    let dist = v02.outer_product(&v23) / v23.len();

    let sin = v01.outer_product(&v23) / (v01.len2() * v23.len2()).sqrt();
    if sin == 0.0 {
        if dist == 0.0 {
            let u = v01.normalized();
            let k1 = v01.len();
            let k2 = u.inner_product(&v02);
            let k3 = u.inner_product(&(*v3 - *v0));
            if (0.0 <= k2 && k2 <= k1)
                || (0.0 <= k3 && k3 <= k1)
                || (k2 < 0.0 && k1 < k3)
                || (k3 < 0.0 && k1 < k2)
            {
                return true; // v01 and v23 is on same infinite-line
            }
        }
        return false; // parallel
    }

    const TOL: f64 = 1e-6;

    let k = dist / sin;
    if k + TOL < 0.0 || k - TOL > v01.len() {
        return false; // cross point is out of v01
    }
    let v4 = *v0 + v01.normalized().scaled(k);
    let v24 = v4 - *v2;
    let k = v23.inner_product(&v24) / v23.len();
    if k + TOL < 0.0 || k - TOL > v23.len() {
        return false; // cross point is out of v23
    }
    true
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
